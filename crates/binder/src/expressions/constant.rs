use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use data_types::{DBTypeIdImpl, IntType, Value};
use sqlparser::ast::Expr;

/// A bound constant, e.g., `1`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Constant {
    pub(crate) value: Value
}

impl Constant {

    pub(crate) fn new(value: Value) -> Self {
        Self {
            value
        }
    }
}

impl TryFrom<&sqlparser::ast::Value> for Constant {
    type Error = ParseASTError;

    fn try_from(value: &sqlparser::ast::Value) -> Result<Self, Self::Error> {
        Ok(match value {
            sqlparser::ast::Value::Number(num_as_string, _long) => {
                // TODO - what is the boolean value?

                Constant::new(Value::new(DBTypeIdImpl::INT(IntType::from(num_as_string.as_bytes()))))
            }
            sqlparser::ast::Value::SingleQuotedString(str) | sqlparser::ast::Value::DoubleQuotedString(str) => {
                unimplemented!();
                // Constant::new(Value::new(DBTypeIdImpl::
            }
            sqlparser::ast::Value::Boolean(val) => Constant::new(Value::new(DBTypeIdImpl::BOOLEAN(val.into()))),

            // TODO(chi): cast integer null to other types
            sqlparser::ast::Value::Null => Constant::new(Value::new(DBTypeIdImpl::INT(None.into()))),
            _ => return Err(ParseASTError::Unimplemented(format!("value type {} is not supported", value)))
        })
    }
}


impl Into<ExpressionTypeImpl> for Constant {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::Constant(self)
    }
}

impl Expression for Constant {
    fn has_aggregation(&self) -> bool {
        false
    }

    fn try_parse_from_expr(expr: &Expr, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Value(v) => v.try_into(),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
