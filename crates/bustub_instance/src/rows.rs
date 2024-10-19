use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;
use catalog_schema::Schema;
use data_types::Value;
use tuple::Tuple;
use crate::result_writer::{ComfyTableWriter, ResultWriter};

pub struct Rows {
    schema: Arc<Schema>,
    rows: Vec<Vec<Value>>,
}

impl Rows {
    pub fn new(tuples: Vec<Tuple>, schema: Arc<Schema>) -> Self {
        let rows = tuples
            .iter()
            .map(|tuple| {
                (0..schema.get_column_count())
                    .map(|index| tuple.get_value(&schema, index))
                    .collect::<Vec<Value>>()
            })
            .collect::<Vec<Vec<Value>>>();

        Rows {
            rows,
            schema
        }
    }

    pub fn create_with_same_schema(&self, new_rows: Vec<Vec<Value>>) -> Self {
        Self {
            rows: new_rows,
            schema: self.schema.clone()
        }
    }

    pub fn write_results<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        // Generate header for the result set.
        writer.begin_table(false);

        writer.begin_header();
        for col in self.schema.get_columns() {
            writer.write_header_cell(col.get_name());
        }
        writer.end_header();

        // Transforming result set into strings
        for row in &self.rows {
            writer.begin_row();
            for col in row {
                writer.write_cell(col.to_string().as_str());
            }
            writer.end_row();
        }
        writer.end_table();
    }
}

impl Deref for Rows {
    type Target = Vec<Vec<Value>>;

    fn deref(&self) -> &Self::Target {
        &self.rows
    }
}

impl Debug for Rows {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = ComfyTableWriter::default();

        self.write_results(&mut writer);

        f.write_str(&*writer.get_tables()[0])
    }
}

// Only check the rows
impl PartialEq for Rows {
    fn eq(&self, other: &Self) -> bool {
        self.rows.eq(&other.rows)
    }
}

impl PartialEq<Vec<Vec<Value>>> for Rows {
    fn eq(&self, other: &Vec<Vec<Value>>) -> bool {
        self.rows.eq(other)
    }
}
