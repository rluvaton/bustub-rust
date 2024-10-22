use crate::table_generator::column_insert_meta::{ColumnInsertMeta, GenerateMeta, GenerateType};
use crate::table_generator::table_insert_meta::TableInsertMeta;
use data_types::{DBTypeId, IntUnderlyingType, Value};
use db_core::catalog::TableInfo;
use execution_engine::ExecutorContext;
use rand::{thread_rng, ThreadRng};
use std::cmp::min;
use std::ops::Deref;
use std::string::ToString;
use std::sync::Arc;
use catalog_schema::{Column, Schema};
use transaction::Transaction;
use tuple::{Tuple, TupleMeta};
use crate::table_generator::constants::{TEST1_SIZE, TEST2_SIZE, TEST_VARLEN_SIZE};
use crate::table_generator::dist::Dist;
use crate::table_generator::generate_values::GenerateValues;


pub(crate) struct TableGenerator<'a> {
    exec_ctx: &'a ExecutorContext<'a>,
}

impl<'a> TableGenerator<'a>
{
    pub fn generate_test_tables(&self) {
        for mut table_meta in Self::get_insert_meta() {
            // Create Schema
            let schema: Schema = table_meta.col_meta
                .iter()
                .map(|col_meta| {
                    let db_type: DBTypeId = col_meta.generate_type.into();

                    match db_type {
                        DBTypeId::INVALID => unreachable!(),
                        DBTypeId::VARCHAR => Column::new_variable_size(col_meta.name.to_string(), db_type, TEST_VARLEN_SIZE as u32),
                        _ => Column::new_fixed_size(col_meta.name.to_string(), db_type)
                    }
                })
                .into();

            let info = self.exec_ctx.get_catalog().lock().create_table(
                self.exec_ctx.get_transaction().clone(),
                table_meta.name.to_string(),
                Arc::new(schema),
                None,
            ).expect("Should be able to create table");
            
            self.fill_table(info, &mut table_meta);
        }
    }

    fn fill_table(&self, info: Arc<TableInfo>, table_meta: &mut TableInsertMeta) {
        let mut rng = thread_rng();
        let mut num_inserted = 0;
        let batch_size = 128;

        let dummy_txn = Arc::new(Transaction::default());
        while num_inserted < table_meta.num_rows {
            let num_values = min(batch_size, table_meta.num_rows - num_inserted);

            let values = table_meta.col_meta
                .iter_mut()
                .map(|item| self.make_values(item, num_values, &mut rng))
                .collect::<Vec<_>>();

            (0..num_values).for_each(|i| {
                let entry = values
                    .iter()
                    // TODO - remove clone
                    .map(|col| col[i].clone())
                    .collect::<Vec<_>>();

                info.get_table_heap().insert_tuple(
                    &TupleMeta::new(0, false),
                    &Tuple::from_value(entry.as_slice(), info.get_schema().deref()),
                    &None,
                    &dummy_txn,
                    None,
                ).expect("Sequential insertion cannot fail");

                num_inserted += 1;
            });
        }
    }

    fn make_values(&self, col_meta: &mut ColumnInsertMeta, count: usize, rng: &mut ThreadRng) -> Vec<Value> {
        col_meta.generate_type.gen_numeric_values(col_meta.dist, &mut col_meta.serial_counter, count, rng)
        // match col_meta.db_type_id {
        //     DBTypeId::TINYINT => self.gen_numeric_values::<TinyIntUnderlyingType>(col_meta, count),
        //     DBTypeId::SMALLINT => self.gen_numeric_values::<SmallIntUnderlyingType>(col_meta, count),
        //     DBTypeId::INT => self.gen_numeric_values::<IntUnderlyingType>(col_meta, count),
        //     DBTypeId::BIGINT => self.gen_numeric_values::<BigIntUnderlyingType>(col_meta, count),
        //     DBTypeId::DECIMAL => self.gen_numeric_values::<DecimalUnderlyingType>(col_meta, count),
        //     DBTypeId::INVALID => unreachable!("This is not valid"),
        //     _ => unimplemented!()
        // }
    }
    
    fn get_insert_meta() -> Vec<TableInsertMeta> {

        /// This array configures each of the test tables. Each table is configured
        /// with a name, size, and schema. We also configure the columns of the table. If
        /// you add a new table, set it up here.

        vec![
            TableInsertMeta {
                name: "empty_table".to_string(),
                num_rows: 0,
                col_meta: vec![
                    ColumnInsertMeta {
                        name: "colA".to_string(),
                        nullable: false,
                        dist: Dist::Serial,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            // Unused
                            min: 0,
                            max: 0,
                        }),
                        serial_counter: 0,
                    }
                ],
            },
            TableInsertMeta {
                name: "test_simple_seq_1".to_string(),
                num_rows: 10,
                col_meta: vec![
                    ColumnInsertMeta {
                        name: "col1".to_string(),
                        nullable: false,
                        dist: Dist::Serial,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 10,
                        }),
                        serial_counter: 0,
                    },
                ],
            },
            TableInsertMeta {
                name: "test_simple_seq_2".to_string(),
                num_rows: 10,
                col_meta: vec![
                    ColumnInsertMeta {
                        name: "col1".to_string(),
                        nullable: false,
                        dist: Dist::Serial,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 10,
                        }),
                        serial_counter: 0,
                    },
                    ColumnInsertMeta {
                        name: "col2".to_string(),
                        nullable: false,
                        dist: Dist::Serial,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 10,
                            max: 20,
                        }),
                        serial_counter: 0,
                    },
                ],
            },
            TableInsertMeta {
                name: "test_1".to_string(),
                num_rows: TEST1_SIZE,
                col_meta: vec![
                    ColumnInsertMeta {
                        name: "colA".to_string(),
                        nullable: false,
                        dist: Dist::Serial,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 0,
                        }),
                        serial_counter: 0,
                    },
                    ColumnInsertMeta {
                        name: "colB".to_string(),
                        nullable: false,
                        dist: Dist::Uniform,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 9,
                        }),
                        serial_counter: 0,
                    },
                    ColumnInsertMeta {
                        name: "colC".to_string(),
                        nullable: false,
                        dist: Dist::Uniform,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 9999,
                        }),
                        serial_counter: 0,
                    },
                    ColumnInsertMeta {
                        name: "colD".to_string(),
                        nullable: false,
                        dist: Dist::Uniform,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 99999,
                        }),
                        serial_counter: 0,
                    },
                ],
            },
            TableInsertMeta {
                name: "test_2".to_string(),
                num_rows: TEST2_SIZE,
                col_meta: vec![
                    ColumnInsertMeta {
                        name: "colA".to_string(),
                        nullable: false,
                        dist: Dist::Serial,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 99,
                        }),
                        serial_counter: 0,
                    },
                    ColumnInsertMeta {
                        name: "colB".to_string(),
                        nullable: true,
                        dist: Dist::Uniform,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 999,
                        }),
                        serial_counter: 0,
                    },
                    ColumnInsertMeta {
                        name: "colC".to_string(),
                        nullable: true,
                        dist: Dist::Cyclic,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 9,
                        }),
                        serial_counter: 0,
                    },
                    ColumnInsertMeta {
                        name: "colD".to_string(),
                        nullable: false,
                        dist: Dist::Uniform,

                        generate_type: GenerateType::Int(GenerateMeta::<IntUnderlyingType> {
                            min: 0,
                            max: 99999,
                        }),
                        serial_counter: 0,
                    },
                ],
            },


            // // Table 3
            // {"test_3",
            //  TEST3_SIZE,
            //  {{"colA", TypeId::INTEGER, false, Dist::Serial, 0, 0}, {"colB", TypeId::INTEGER, true, Dist::Serial, 0, 0}}},

            // // Table 4
            // {"test_4",
            //  TEST4_SIZE,
            //  {{"colA", TypeId::BIGINT, false, Dist::Serial, 0, 0},
            //   {"colB", TypeId::INTEGER, true, Dist::Serial, 0, 0},
            //   {"colC", TypeId::INTEGER, true, Dist::Uniform, 0, 9}}},

            // // Table 5
            // {"test_5",
            //  0,
            //  {{"colA", TypeId::BIGINT, false, Dist::Serial, 0, 0}, {"colB", TypeId::INTEGER, true, Dist::Serial, 0, 0}}},

            // // Table 6
            // {"test_6",
            //  TEST6_SIZE,
            //  {{"colA", TypeId::BIGINT, false, Dist::Serial, 0, 0},
            //   {"colB", TypeId::INTEGER, true, Dist::Serial, 0, 0},
            //   {"colC", TypeId::INTEGER, true, Dist::Uniform, 0, 9}}},

            // // Table 7
            // {"test_7",
            //  TEST7_SIZE,
            //  {{"col1", TypeId::SMALLINT, false, Dist::Serial, 0, 0},
            //   {"col2", TypeId::INTEGER, true, Dist::Uniform, 0, 9},
            //   {"col3", TypeId::BIGINT, false, Dist::Uniform, 0, 1024},
            //   {"col4", TypeId::INTEGER, true, Dist::Uniform, 0, 2048}}},

            // // Table 8
            // {"test_8",
            //  TEST8_SIZE,
            //  {{"colA", TypeId::BIGINT, false, Dist::Serial, 0, 0}, {"colB", TypeId::INTEGER, true, Dist::Serial, 0, 0}}},

            // // Table 9
            // {"test_9",
            //  TEST9_SIZE,
            //  {{"colA", TypeId::BIGINT, false, Dist::Serial, 0, 0}, {"colB", TypeId::INTEGER, true, Dist::Serial, 0, 0}}},

            // // Empty table with two columns
            // {"empty_table2",
            //  0,
            //  {{"colA", TypeId::INTEGER, false, Dist::Serial, 0, 0}, {"colB", TypeId::INTEGER, false, Dist::Uniform, 0,
            //  9}}},

            // // Empty table with two columns
            // {"empty_table3",
            //  0,
            //  {{"colA", TypeId::BIGINT, false, Dist::Serial, 0, 0}, {"colB", TypeId::INTEGER, false, Dist::Uniform, 0, 9}}},
        ]
    }
}

impl<'a> From<&'a ExecutorContext<'a>> for TableGenerator<'a> {
    fn from(value: &'a ExecutorContext<'a>) -> Self {
        TableGenerator {
            exec_ctx: value,
        }
    }
}