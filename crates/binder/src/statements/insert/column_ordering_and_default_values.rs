use sqlparser::ast::Ident;
use catalog_schema::{Column, Schema};
use data_types::Value;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColumnOrderingAndDefaultValuesForInsert(
    /// This is a vector that for each column in the schema
    /// if the value is:
    /// - None: we should use the default value
    /// - Some(index): we should use the value at the provided index from values row
    Vec<Option<usize>>
);

impl ColumnOrderingAndDefaultValuesForInsert {
    pub(crate) fn from_ast_and_schema(ast_columns: &[Ident], schema: &Schema) -> Self {
        if ast_columns.len() == 0 {
            return Self::use_same_columns_ordering(schema)
        }
        let schema_columns = schema.get_columns();

        assert!(schema_columns.len() >= ast_columns.len(), "Number of schema columns ({}) must be greater than or equal to the number of the AST columns {}", schema_columns.len(), ast_columns.len());

        let value = schema_columns
            .iter()
            .map(|col| {
                let col_name = col.get_name();

                ast_columns
                    .iter()
                    .position(|query_col| query_col.to_string() == col_name.as_str())
            })
            .collect::<Vec<_>>();

        Self(value)
    }

    pub(crate) fn use_same_columns_ordering(schema: &Schema) -> Self {
        let positions = (0..schema.get_column_count()).map(|index| Some(index)).collect::<Vec<_>>();

        Self(positions)
    }

    /// Reorder column to be in the same order as the schema one
    ///
    /// # Safety
    /// This is unsafe as it assume the following:
    /// 1. schema columns length >= provided columns length
    /// 2. Every index in the schema columns map to existing column in the provided columns array
    ///
    pub unsafe fn reorder_unchecked(&self, columns: &[Value]) -> Vec<Option<Value>> {
        assert!(self.0.len() >= columns.len(), "Number of schema columns ({}) must be greater than or equal to the number of the provided columns {}", self.0.len(), columns.len());

        self.0
            .iter()
            // TODO - AVOID CLONE
            .map(|optional_input_column_index| optional_input_column_index.map(|index| columns[index].clone()))
            .collect()
    }

    pub fn should_use_column_ordering(&self) -> bool {
        self.0.iter()
            .enumerate()
            .any(|(value_index, schema_index_op)| {
                if let Some(schema_index) = schema_index_op {
                    value_index != *schema_index
                } else {
                    true
                }
            })
    }


    /// Create new schema
    ///
    /// # Arguments
    ///
    /// * `table_schema`:
    ///
    /// returns: Schema
    pub fn create_new_schema(&self, table_schema: &Schema) -> Schema {
        let mut index_and_columns: Vec<(usize, &Column)> = self.0.iter()
            .zip(table_schema.get_columns().iter())
            .filter_map(|(optional_input_column_index, schema_column)| {
                optional_input_column_index.map(|input_columns_index| (input_columns_index, schema_column))
            })
            .collect();

        index_and_columns.sort_by_key(|t| t.0);

        index_and_columns
            .iter()
            .map(|(_, item)| *item)
            .cloned()
            .into()
    }

    #[must_use]
    pub fn map_values_based_on_schema<GetDefaultFn: Fn(&Column) -> Value>(&self, table_schema: &Schema, values: &[Value], get_default: GetDefaultFn) -> Vec<Value> {
        let number_of_values = values.len();
        let expected_number_of_values = self.0.iter().filter(|index| index.is_some()).count();

        assert_eq!(number_of_values, expected_number_of_values, "Number of values must have the same number of values as there are columns");
        assert_eq!(table_schema.get_column_count(), self.0.len(), "Table schema must have the same number of columns as this column mapping");

        self.0.iter()
            .zip(table_schema.get_columns().iter())
            .map(|(optional_input_column_index, schema_column)| {
                match optional_input_column_index {
                    None => {
                        let default = get_default(schema_column);

                        assert_eq!(default.get_db_type_id(), schema_column.get_type(), "Default DB type ({}) must be the same as the column type {}", default.get_db_type_id(), schema_column.get_type());

                        default
                    }
                    // TODO - AVOID CLONE
                    Some(index) => values[*index].clone()
                }
            })
            .collect()
    }
}
