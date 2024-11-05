use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::sql_parser_helper::SelectItemExt;
use crate::statements::SelectStatementBuilder;
use crate::table_ref::TableReferenceTypeImpl;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::{Binder, TableRef};
use sqlparser::ast::{Distinct, GroupByExpr};
use std::rc::Rc;

pub(crate) trait SelectExt {
    fn add_select_to_select_builder(&self, builder: SelectStatementBuilder, binder: &Binder) -> ParseASTResult<SelectStatementBuilder>;

    fn parse_select_list(&self, binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>>;
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
            
            if is_distinct {
                return Err(ParseASTError::Unimplemented("DISTINCT is not supported at the moment".to_string()));
            }
        }

        // SELECT list
        builder = builder.with_select_list(self.parse_select_list(binder)?);

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
                    if !expr.is_empty() {
                        return Err(ParseASTError::Unimplemented("GROUP BY is not supported at the moment".to_string()));
                    }
                }
            }
            
            
        }

        // HAVING
        {
            if let Some(having) = &self.having {
                builder = builder.with_having(ExpressionTypeImpl::try_parse_from_expr(having, binder)?);
                
                return Err(ParseASTError::Unimplemented("HAVING is not supported at the moment".to_string()));
            }
        }

        Ok(builder)
    }

    fn parse_select_list(&self, binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
        let select_list: ParseASTResult<Vec<ExpressionTypeImpl>> = self.projection
            .iter()
            .map(|select_item| select_item.parse(binder))
            .collect();

        let select_list = select_list?;

        let has_select_star = select_list.iter().any(|item| matches!(item, ExpressionTypeImpl::Star(_)));

        if has_select_star {
            if select_list.len() > 1 {
                return Err(ParseASTError::Unimplemented("select * cannot have other expressions in list".to_string()));
            }

            let scope = binder.context.lock().scope.as_ref().expect("Must have scope for select *").clone();

            return scope.get_all_columns(binder);
        }


        let (has_agg, has_window_agg) = select_list.iter().fold((false, false), |(has_agg, has_window_agg), expr| {
            (has_agg || expr.has_aggregation(), has_window_agg || expr.has_window_function())
        });

        if has_agg && has_window_agg {
            return Err(ParseASTError::FailedParsing("cannot have both normal agg and window agg in same query".to_string()));
        }

        Ok(select_list)
    }
}
