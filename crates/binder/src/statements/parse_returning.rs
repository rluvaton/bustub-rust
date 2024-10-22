use std::rc::Rc;
use sqlparser::ast::SelectItem;
use crate::{Binder, ExpressionTypeImpl, TableReferenceTypeImpl};
use crate::sql_parser_helper::SelectItemExt;
use crate::try_from_ast_error::ParseASTResult;

pub(crate) fn parse_returning(returning: &Option<Vec<SelectItem>>, table: Rc<TableReferenceTypeImpl>, binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
    if returning.is_none() {
        return Ok(vec![]);
    }

    let returning = returning.as_ref().unwrap();

    // This is the current context for the returning

    let ctx_guard = binder.new_context();

    ctx_guard.context.lock().scope.replace(table);
    
    returning.iter().map(|item| item.parse(binder)).collect()
}