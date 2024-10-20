use crate::expressions::{AliasExpr, Expression, ExpressionTypeImpl, StarExpr};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{RenameSelectItem, SelectItem};

pub(crate) trait SelectItemExt {
    fn parse(&self, binder: &Binder) -> ParseASTResult<ExpressionTypeImpl>;
}

impl SelectItemExt for SelectItem {
    fn parse(&self, binder: &Binder) -> ParseASTResult<ExpressionTypeImpl> {
        let expr = match self {
            SelectItem::UnnamedExpr(expr) => ExpressionTypeImpl::try_parse_from_expr(expr, binder)?,
            SelectItem::ExprWithAlias { expr, alias } => {
                AliasExpr::new(
                    alias.to_string(),
                    Box::new(ExpressionTypeImpl::try_parse_from_expr(expr, binder)?),
                ).into()
            }
            SelectItem::Wildcard(opt) => {
                if let Some(rename) = &opt.opt_rename {
                    match rename {
                        RenameSelectItem::Single(alias) => AliasExpr::new(alias.alias.to_string(), Box::new(StarExpr::new().into())).into(),
                        RenameSelectItem::Multiple(_) => return Err(ParseASTError::Unimplemented("Wildcard with multiple rename is not supported".to_string()))
                    }
                } else {
                    StarExpr::new().into()
                }
            }
            SelectItem::QualifiedWildcard(_, _) => return Err(ParseASTError::Unimplemented("QualifiedWildcard is not supported".to_string())),
        };

        Ok(expr)
    }
}
