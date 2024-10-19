use sqlparser::ast::{Ident, TableConstraint};
use crate::sql_parser_helper::ColumnDefExt;
use crate::try_from_ast_error::{ParseASTResult, ParseASTError};

pub(super) trait SqlParserCreateStatementExt {
    fn try_get_primary_columns(&self) -> ParseASTResult<Vec<String>>;
}

impl SqlParserCreateStatementExt for sqlparser::ast::CreateTable {
    fn try_get_primary_columns(&self) -> ParseASTResult<Vec<String>> {

        let primary_columns_from_column_definition: Vec<String> = {
            let primary_columns: ParseASTResult<Vec<(bool, String)>> = self.columns
                .iter()
                .map(|item| item.try_is_primary_column().map(|is_primary| (is_primary, item.name.value.clone())))
                .collect();

            primary_columns?.iter()
                .filter(|col| col.0)
                .map(|col| col.1.clone())
                .collect()
        };

        let primary_columns_from_table_constraints: Vec<String> = {
            let primary_key_columns: ParseASTResult<Vec<&Vec<Ident>>> = self.constraints.iter().map(|constraint| {
                match constraint {
                    TableConstraint::PrimaryKey { columns, .. } => Ok(columns),
                    _ => return Err(ParseASTError::Unimplemented(format!("Constraints {} is not supported", constraint)))
                }
            }).collect();

            let primary_key_columns = primary_key_columns?;
            if primary_key_columns.len() > 1 {
                return Err(ParseASTError::Unimplemented("Multiple primary key constraint on a single table is not supported".to_string()));
            }
            let default_primary_columns = &vec![];
            let primary_key_columns = primary_key_columns.get(0).unwrap_or(&default_primary_columns);

            primary_key_columns.iter().map(|item| item.value.clone()).collect()
        };

        Ok([primary_columns_from_column_definition, primary_columns_from_table_constraints].concat())
    }
}
