use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use data_types::{BigIntType, DBTypeIdImpl, IntType, Value};
use sqlparser::ast::Expr;

/// A bound constant, e.g., `1`.
#[derive(Clone, Debug, PartialEq)]
pub struct Constant {
    pub value: Value
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
            sqlparser::ast::Value::Number(num_as_string, long) => {
                // TODO - cast value based on context if it's without explicit casting - e.g. ig in insert values for example
                let value = if *long {
                    BigIntType::try_from(num_as_string.as_str()).expect("Number is too large").into()
                } else {
                    IntType::try_from(num_as_string.as_str()).expect("Number is too large").into()
                };
                Constant::new(value)
            }
            sqlparser::ast::Value::SingleQuotedString(str) | sqlparser::ast::Value::DoubleQuotedString(str) => {
                Constant::new(Value::from(str.as_str()))
            }
            sqlparser::ast::Value::Boolean(val) => Constant::new(Value::from(*val)),

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

    fn try_parse_from_expr(expr: &Expr, _binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Value(v) => v.try_into(),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
