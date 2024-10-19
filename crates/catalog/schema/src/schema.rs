use error_utils::anyhow::anyhow;
use data_types::DBTypeId;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use crate::Column;
use error_utils::ToAnyhowResult;

// TODO - implement src/include/catalog/schema.h
#[derive(Clone, PartialEq)]
pub struct Schema {
    /** Fixed-length column size, i.e. the number of bytes used by one tuple. */
    length: u32,

    /** All the columns in the schema, inlined and uninlined. */
    columns: Vec<Column>,

    /** True if all the columns are inlined, false otherwise. */
    // TODO - default true
    tuple_is_inlined: bool,

    /** Indices of all uninlined columns. */
    uninlined_columns: Vec<u32>,
}

impl Schema {
    ///
    /// Constructs the schema corresponding to the vector of columns, read left-to-right.
    ///
    /// # Arguments
    /// columns: columns that describe the schema's individual columns
    ///
    pub fn new(columns: Vec<Column>) -> Self {
        let mut tuple_is_inlined: bool = true;
        let mut uninlined_columns: Vec<u32> = vec![];

        let mut curr_offset: u32 = 0;

        let self_columns = columns
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, mut column)| {
                // handle uninlined column
                if !column.is_inlined() {
                    tuple_is_inlined = false;
                    uninlined_columns.push(index as u32);
                }

                // set column offset
                column.column_offset = curr_offset;
                curr_offset += column.get_fixed_length();

                // add column
                column
            })
            .collect::<Vec<Column>>();

        Self {
            // set tuple length
            length: curr_offset,
            tuple_is_inlined,
            uninlined_columns,
            columns: self_columns,
        }
    }


    pub fn copy_schema(&self, attrs: &[u32]) -> Self {
        let mut columns: Vec<Column> = Vec::with_capacity(attrs.len());

        for &i in attrs {
            columns.push(self.columns[i as usize].clone())
        }

        Schema::new(columns)
    }

    /// Return all the columns in the schema
    pub fn get_columns(&self) -> &Vec<Column> {
        &self.columns
    }

    /// Returns a specific column from the schema.
    ///
    /// # Arguments
    ///
    /// * `col_idx`: index of requested column
    ///
    /// returns: &Column requested column
    pub fn get_column(&self, col_idx: usize) -> &Column {
        &self.columns[col_idx]
    }


    /// Looks up and returns the index of the first column in the schema with the specified name.
    /// If multiple columns have the same name, the first such index is returned.
    ///
    /// # Arguments
    ///
    /// * `col_name` - name of column to look for
    ///
    /// returns: usize The index of a column with the given name, panic if not exists
    pub fn get_col_idx(&self, col_name: &str) -> usize {
        self.try_get_col_idx(col_name).expect("Column must exist")
    }


    /// Looks up and returns the index of the first column in the schema with the specified name.
    /// If multiple columns have the same name, the first such index is returned.
    ///
    /// # Arguments
    ///
    /// * `col_name` - name of column to look for
    ///
    /// returns: Option<usize> The index of a column with the given name, `None` if missing
    ///
    pub fn try_get_col_idx(&self, col_name: &str) -> Option<usize> {
        self.columns.iter().position(|col| *col.get_name() == col_name)
    }


    /// return the indices of non-inlined columns
    pub fn get_unlined_columns(&self) -> &Vec<u32> { &self.uninlined_columns }

    /// return the number of columns in the schema for the tuple
    pub fn get_column_count(&self) -> usize {
        self.columns.len()
    }

    /// return the number of non-inlined columns
    pub fn get_unlined_column_count(&self) -> usize {
        self.uninlined_columns.len()
    }

    /// the number of bytes used by one tuple
    pub fn get_length(&self) -> u32 {
        self.length
    }

    /// the number of bytes used by one tuple
    pub fn is_inlined(&self) -> bool {
        self.tuple_is_inlined
    }

    // Only for tests
    pub fn parse_create_statement(sql_base: &str) -> error_utils::anyhow::Result<Arc<Schema>> {
        let mut n: usize;
        let mut v: Vec<Column> = vec![];
        let mut column_name: &str;
        let mut column_type: &str;
        let mut column_length;
        let mut type_id: DBTypeId;

        // create a copy of the sql query
        let sql: &str = sql_base;

        // Preprocess, transform sql string into lower case
        let sql = sql.to_lowercase();
        let tok: Vec<&str> = sql.split(",").collect();

        // Iterate through returned result

        for t in tok {
            type_id = DBTypeId::INVALID;
            column_length = 0;

            // whitespace separate column name and type
            if let Some(white_space_position) = t.find(" ") {
                n = white_space_position;

                column_name = &t[0..n];
                column_type = &t[n + 1..];
            } else {
                column_name = t;
                column_type = t;
            }


            // Deal with varchar(size) situation
            if let Some(white_space_position) = t.find("(") {
                n = white_space_position;

                column_length = i32::from_ne_bytes(<&[u8] as TryInto<[u8;4]>>::try_into(&column_type.as_bytes()[n + 1..]).to_anyhow()?);
                column_type = &column_type[..n];
            }

            if column_type == "bool" || column_type == "boolean" {
                type_id = DBTypeId::BOOLEAN;
            } else if column_type == "tinyint" {
                type_id = DBTypeId::TINYINT;
            } else if column_type == "smallint" {
                type_id = DBTypeId::SMALLINT;
            } else if column_type == "int" || column_type == "integer" {
                type_id = DBTypeId::INT;
            } else if column_type == "bigint" {
                type_id = DBTypeId::BIGINT;
            } else if column_type == "double" || column_type == "float" {
                type_id = DBTypeId::DECIMAL;
            } else if column_type == "varchar" || column_type == "char" {
                type_id = DBTypeId::VARCHAR;
                if column_length == 0 {
                    column_length = 32;
                }
            }

            // Construct each column
            if matches!(type_id, DBTypeId::INVALID) {
                // TODO - return custom error type instead
                // throw Exception(ExceptionType::UNKNOWN_TYPE, "unknown type for create table");
                return Err(anyhow!("unknown type for create table"));
            }

            if matches!(type_id, DBTypeId::VARCHAR) {
                assert!(column_length >= 0, "Column length must be greater than or equal to 0, received {}", column_length);
                let col = Column::new_variable_size(column_name.to_string(), type_id, column_length as u32);
                v.push(col);
            } else {
                let col = Column::new_fixed_size(column_name.to_string(), type_id);
                v.push(col);
            }
        }

        Ok(Arc::new(Schema::new(v)))
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.columns.iter().map(|column| format!("{}", column)).collect::<Vec<String>>().join(", "))
    }
}

impl Debug for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Schema[NumColumns: {}, IsInlined: {}, Length: {}] :: ({})",
               self.get_column_count(),
               self.is_inlined(),
               self.get_length(),
               self.columns.iter().map(|column| format!("{:?}", column)).collect::<Vec<String>>().join(", ")
        )
    }
}

impl<I: Iterator<Item=Column>> From<I> for Schema {
    fn from(value: I) -> Self {
        Schema::new(value.collect::<Vec<Column>>())
    }
}
