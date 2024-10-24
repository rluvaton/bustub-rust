use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::sql_parser_helper::SelectItemExt;
use crate::statements::SelectStatementBuilder;
use crate::table_ref::TableReferenceTypeImpl;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{Distinct, GroupByExpr};
use std::rc::Rc;

pub(crate) trait SelectExt {
    fn add_select_to_select_builder(&self, builder: SelectStatementBuilder, binder: &Binder) -> ParseASTResult<SelectStatementBuilder>;
}

impl SelectExt for sqlparser::ast::Select {
    fn add_select_to_select_builder(&self, mut builder: SelectStatementBuilder, binder: &Binder) -> ParseASTResult<SelectStatementBuilder> {
        // FROM
        {
            let table = TableReferenceTypeImpl::try_to_parse_tables_with_joins(self.from.as_slice(), binder)?;
            let table = Rc::new(table);

            builder = builder.with_table(table.clone());

            binder.context.lock().scope.replace(table);
        }

        // DISTINCT
        {
            let mut is_distinct = false;
            if let Some(distinct) = &self.distinct {
                match distinct {
                    Distinct::Distinct => is_distinct = true,
                    Distinct::On(_) => return Err(ParseASTError::Unimplemented("DISTINCT ON is not supported".to_string()))
                }
            }

            builder = builder.with_is_distinct(is_distinct);
        }

        // SELECT list
        {
            let select_list: ParseASTResult<Vec<ExpressionTypeImpl>> = self.projection
                .iter()
                .map(|select_item| select_item.parse(binder))
                .collect();

            builder = builder.with_select_list(select_list?);
        }

        // WHERE
        {
            if let Some(where_expr) = &self.selection {
                builder = builder.with_where_exp(ExpressionTypeImpl::try_parse_from_expr(where_expr, binder)?);
            }
        }

        // GROUP BY
        {
            match &self.group_by {
                GroupByExpr::All(_) => return Err(ParseASTError::Unimplemented("ALL in group by is not supported".to_string())),
                GroupByExpr::Expressions(expr, _) => {
                    builder = builder.with_group_by(ExpressionTypeImpl::parse_expression_list(expr, binder)?);
                }
            }
        }

        // HAVING
        {
            if let Some(having) = &self.having {
                builder = builder.with_having(ExpressionTypeImpl::try_parse_from_expr(having, binder)?);
            }
        }

        // LIMIT
        {
            if let Some(having) = &self.having {
                builder = builder.with_having(ExpressionTypeImpl::try_parse_from_expr(having, binder)?);
            }
        }


        Ok(builder)
    }
}
