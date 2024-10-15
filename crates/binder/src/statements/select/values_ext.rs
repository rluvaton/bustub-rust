use std::rc::Rc;
use std::sync::Arc;
use crate::Binder;
use crate::expressions::{ColumnRef, ExpressionTypeImpl};
use crate::statements::select::builder::SelectStatementBuilder;
use crate::table_ref::ExpressionListRef;
use crate::try_from_ast_error::ParseASTResult;

pub(super) trait ValuesExt {
    fn add_values_to_select_builder(&self, builder: SelectStatementBuilder, binder: &Binder) -> ParseASTResult<SelectStatementBuilder>;
}

impl ValuesExt for sqlparser::ast::Values {
    fn add_values_to_select_builder(&self, builder: SelectStatementBuilder, binder: &Binder) -> ParseASTResult<SelectStatementBuilder> {
        let values_list_name = format!("__values#{}", binder.get_and_increment_universal_id());

        let value_list: ExpressionListRef = ExpressionListRef::try_parse_from_values(Some(values_list_name.clone()), self, binder)?;

        let exprs: Vec<ExpressionTypeImpl> = value_list
            .values[0]
            .iter()
            .enumerate()
            .map(|(i, _)| ColumnRef::new(vec![values_list_name.clone(), i.to_string()]).into())
            .collect();

        Ok(
            builder
                .with_table(Rc::new(value_list.into()))
                .with_select_list(exprs)
        )
    }
}
