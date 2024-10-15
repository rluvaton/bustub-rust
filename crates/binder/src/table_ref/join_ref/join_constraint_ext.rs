use sqlparser::ast::JoinConstraint;
use crate::Binder;
use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};

pub(super) trait JoinConstraintExt {
    fn convert_to_condition(&self, binder: &Binder) -> ParseASTResult<Option<ExpressionTypeImpl>>;
}

impl JoinConstraintExt for JoinConstraint {
    fn convert_to_condition(&self, binder: &Binder) -> ParseASTResult<Option<ExpressionTypeImpl>> {
        match &self {
            JoinConstraint::On(expr) => Ok(Some(ExpressionTypeImpl::try_parse_from_expr(expr, binder)?)),
            JoinConstraint::Using(_) => Err(ParseASTError::Unimplemented("Using in join is not supported".to_string())),
            JoinConstraint::Natural => Err(ParseASTError::Unimplemented("Natual join is not supported".to_string())),
            JoinConstraint::None => Ok(None)
        }
    }
}
