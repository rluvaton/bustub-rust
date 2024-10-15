use sqlparser::ast::{DuplicateTreatment, FunctionArg, FunctionArgExpr, FunctionArguments};
use crate::Binder;
use crate::expressions::{Expression, ExpressionTypeImpl, StarExpr};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};

pub(crate) trait FunctionExt {
    fn parse_args(&self, binder: &Binder) -> ParseASTResult<Vec<Box<ExpressionTypeImpl>>>;
    fn is_distinct(&self) -> bool;
}

impl FunctionExt for sqlparser::ast::Function {
    fn parse_args(&self, binder: &Binder) -> ParseASTResult<Vec<Box<ExpressionTypeImpl>>> {
        let arg_list = match &self.args {
            FunctionArguments::None => return Ok(vec![]),
            FunctionArguments::List(l) => l,
            _ => return Err(ParseASTError::Unimplemented(format!("{} is not supported", self.args)))
        };

        let arguments: ParseASTResult<Vec<Box<ExpressionTypeImpl>>> = arg_list.args
            .iter()
            .map(|arg| {
                let named = match arg {
                    FunctionArg::Named { .. } => return Err(ParseASTError::Unimplemented("Named argument is not supported".to_string())),
                    FunctionArg::Unnamed(n) => n
                };

                match named {
                    FunctionArgExpr::Expr(expr) => ExpressionTypeImpl::try_parse_from_expr(expr, binder),
                    FunctionArgExpr::QualifiedWildcard(_) => Err(ParseASTError::Unimplemented("QualifiedWildcard is not supported".to_string())),
                    FunctionArgExpr::Wildcard => Ok(StarExpr::new().into())
                }
            })
            .map(|item| item.map(|arg| Box::new(arg)))
            .collect();

        arguments
    }

    fn is_distinct(&self) -> bool {
        match &self.args {
            FunctionArguments::List(l) => l.duplicate_treatment.is_some_and(|d| {
                match d {
                    DuplicateTreatment::Distinct => true,
                    DuplicateTreatment::All => false
                }
            }),
            _ => false
        }
    }
}
