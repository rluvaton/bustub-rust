use crate::sql_parser_helper::{ColumnDefExt, ConstraintExt};
use crate::statements::traits::Statement;
use crate::statements::StatementType;
use crate::try_from_ast_error::{ParseASTResult, TryFromASTError};
use db_core::catalog::Column;
use std::fmt::Debug;
use crate::Binder;
use crate::expressions::ExpressionTypeImpl;
use crate::table_ref::BaseTableRef;
use super::SqlParserDeleteStatementExt;

#[derive(Debug, PartialEq)]
pub struct DeleteStatement {
    pub(crate) table: BaseTableRef,
    pub(crate) expr: Box<ExpressionTypeImpl>,
}

impl DeleteStatement {
    pub fn new(table: BaseTableRef, expr: Box<ExpressionTypeImpl>) -> Self {
        Self {
            table,
            expr
        }
    }
}


// impl Statement for DeleteStatement {
//     const TYPE: StatementType = StatementType::Delete;
//     type ASTStatement = sqlparser::ast::Delete;
//
//     fn try_parse(ast: &Self::ASTStatement, binder: &mut Binder) -> ParseASTResult<Self> {
//         let delete = match ast {
//             sqlparser::ast::Statement::Delete(delete) => delete,
//             _ => return Err(TryFromASTError::IncompatibleType)
//         };
//
//         let columns: ParseASTResult<Vec<Column>> = delete.columns.iter().map(|item| item.try_convert_into_column()).collect();
//         let columns = columns?;
//
//         if columns.is_empty() {
//             return Err(TryFromASTError::FailedParsing("Columns cannot be empty".to_string()))
//         }
//
//
//         Ok(DeleteStatement {
//             table: delete.name.to_string(),
//             columns,
//             primary_key: delete.try_get_primary_columns()?,
//         })
//     }
// }


#[cfg(test)]
mod tests {
    use data_types::DBTypeId;
    use db_core::catalog::Column;

    use crate::try_from_ast_error::TryFromASTError;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use crate::Binder;
    use crate::statements::DeleteStatement;
    use crate::table_ref::BaseTableRef;

    fn parse_delete_sql(sql: &str) -> Result<Vec<DeleteStatement>, TryFromASTError> {
        // let binder = Binder::default();
        // let statements = Parser::parse_sql(&GenericDialect {}, sql).unwrap();
        // statements.iter().map(|item| DeleteStatement::try_parse_ast().try_into()).collect()
        todo!()
    }

    // #[test]
    // fn convert_delete_to_statement() {
    //     let sql = "DELETE FROM tasks WHERE status = 'DONE' RETURNING *";
    //
    //     let expected_create_statement = DeleteStatement::new(
    //         BaseTableRef::new()
    //         "distributors".to_string(),
    //         vec![
    //             Column::new_fixed_size("did".to_string(), DBTypeId::INT),
    //             Column::new_variable_size("name".to_string(), DBTypeId::VARCHAR, 40),
    //         ],
    //         vec!["did".to_string()],
    //     );
    //
    //     let create_statements = parse_create_sql(sql).expect("should parse");
    //
    //     assert_eq!(create_statements, vec![expected_create_statement]);
    // }
}
