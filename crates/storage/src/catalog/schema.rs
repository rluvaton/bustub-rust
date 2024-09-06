use std::ops::Deref;
use anyhow::{anyhow, Result};
use std::sync::Arc;
use common::types::TypeId;
use crate::catalog::Column;


// TODO - implement src/include/catalog/schema.h
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

    pub fn get_column(&self, col_idx: u32) -> &Column {
        unimplemented!();
    }

    pub fn get_column_count(&self) -> u32 {
        unimplemented!();
    }

    // exported only for tests
    #[cfg(test)]
    pub fn parse_create_statement(sql_base: &str) -> Result<Arc<Schema>> {
        let mut n: usize;
        let mut v: Vec<Column> = vec![];
        let mut column_name: &str;
        let mut column_type: &str;
        let mut column_length = 0;
        let mut type_id: TypeId = TypeId::INVALID;

        // create a copy of the sql query
        let sql: &str = sql_base.clone();

        // Preprocess, transform sql string into lower case
        let sql = sql.to_lowercase();
        let tok: Vec<&str> = sql.split(",").collect();

        // Iterate through returned result

        for t in tok {
            type_id = TypeId::INVALID;
            column_length = 0;

            // whitespace separate column name and type
            if let Some(white_space_position) = t.find(" ") {
                n = white_space_position;

                column_name = &t[0..n];
                column_type = &t[n + 1..];
            } else {
                column_name = t.clone();
                column_type = t.clone();
            }


            // Deal with varchar(size) situation
            if let Some(white_space_position) = t.find("(") {
                n = white_space_position;

                column_length = i32::from_ne_bytes(column_type.as_bytes()[n + 1..].try_into()?);
                column_type = &column_type[..n];
            }

            if column_type == "bool" || column_type == "boolean" {
                type_id = TypeId::BOOLEAN;
            } else if column_type == "tinyint" {
                type_id = TypeId::TINYINT;
            } else if column_type == "smallint" {
                type_id = TypeId::SMALLINT;
            } else if column_type == "int" || column_type == "integer" {
                type_id = TypeId::INTEGER;
            } else if column_type == "bigint" {
                type_id = TypeId::BIGINT;
            } else if column_type == "double" || column_type == "float" {
                type_id = TypeId::DECIMAL;
            } else if column_type == "varchar" || column_type == "char" {
                type_id = TypeId::VARCHAR;
                if column_length == 0 {
                    column_length = 32;
                }
            }

            // Construct each column
            if matches!(type_id, TypeId::INVALID) {
                // TODO - return custom error type instead
                // throw Exception(ExceptionType::UNKNOWN_TYPE, "unknown type for create table");
                return Err(anyhow!("unknown type for create table"));
            }

            if matches!(type_id, TypeId::VARCHAR) {
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

    // TODO - ADD TO STRING
}

