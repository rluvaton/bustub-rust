use sqlparser::ast::Expr;
use crate::{fallback_on_incompatible_2_args, Binder};
use crate::expressions::{AliasExpr, UnaryOpExpr, ColumnRef, Constant, Expression, BinaryOpExpr, StarExpr, AggCallExpr, FuncCallExpr, WindowExpr};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};

#[derive(Copy, Clone, PartialEq)]
pub enum ExpressionType {
    Invalid,    // < Invalid expression type.
    Constant,   // < Constant expression type.
    ColumnRef, // < A column in a table.
    TypeCast,  // < Type cast expression type.
    Function,   // < Function expression type.
    AggCall,   // < Aggregation function expression type.
    Star,       // < Star expression type, will be rewritten by binder and won't appear in plan.
    UnaryOp,   // < Unary expression type.
    BinaryOp,  // < Binary expression type.
    Alias,     // < Alias expression type.
    FuncCall, // < Function call expression type.
    Window,    // < Window Aggregation expression type.
}


#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionTypeImpl {
    ColumnRef(ColumnRef), // < A column in a table.
    Constant(Constant),
    Alias(AliasExpr),     // < Alias expression type.
    BinaryOp(BinaryOpExpr),
    UnaryOp(UnaryOpExpr),
    Star(StarExpr),
    FuncCall(FuncCallExpr),
    AggCall(AggCallExpr),
    Window(WindowExpr),
    Invalid,
}

impl ExpressionTypeImpl {
    pub fn parse_expression_list(list: &Vec<Expr>, binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
        list
            .iter()
            .map(|item| Self::try_parse_from_expr(item, binder))
            .collect()
    }
    pub fn parse_boxed_expression_list(list: &Vec<Expr>, binder: &Binder) -> ParseASTResult<Vec<Box<ExpressionTypeImpl>>> {
        list
            .iter()
            .map(|item| Self::try_parse_from_expr(item, binder).map(|exp| Box::new(exp)))
            .collect()
    }
}

impl From<ExpressionTypeImpl> for ExpressionType {
    fn from(value: ExpressionTypeImpl) -> Self {
        match value {
            ExpressionTypeImpl::ColumnRef(_) => ExpressionType::ColumnRef,
            ExpressionTypeImpl::Constant(_) => ExpressionType::Constant,
            ExpressionTypeImpl::Alias(_) => ExpressionType::Alias,
            ExpressionTypeImpl::Invalid => ExpressionType::Invalid,
            ExpressionTypeImpl::BinaryOp(_) => ExpressionType::BinaryOp,
            ExpressionTypeImpl::UnaryOp(_) => ExpressionType::UnaryOp,
            ExpressionTypeImpl::Star(_) => ExpressionType::Star,
            ExpressionTypeImpl::FuncCall(_) => ExpressionType::FuncCall,
            ExpressionTypeImpl::AggCall(_) => ExpressionType::AggCall,
            ExpressionTypeImpl::Window(_) => ExpressionType::Window,
        }
    }
}

impl Expression for ExpressionTypeImpl {
    fn has_aggregation(&self) -> bool {
        match &self {
            ExpressionTypeImpl::ColumnRef(e) => e.has_aggregation(),
            ExpressionTypeImpl::Alias(e) => e.has_aggregation(),
            ExpressionTypeImpl::Constant(e) => e.has_aggregation(),
            ExpressionTypeImpl::BinaryOp(e) => e.has_aggregation(),
            ExpressionTypeImpl::UnaryOp(e) => e.has_aggregation(),
            ExpressionTypeImpl::Star(e) => e.has_aggregation(),
            ExpressionTypeImpl::FuncCall(e) => e.has_aggregation(),
            ExpressionTypeImpl::AggCall(e) => e.has_aggregation(),
            ExpressionTypeImpl::Window(e) => e.has_aggregation(),

            // TODO - throw unreachable
            ExpressionTypeImpl::Invalid => false,
        }
    }

    fn has_window_function(&self) -> bool {
        match &self {
            ExpressionTypeImpl::ColumnRef(e) => e.has_window_function(),
            ExpressionTypeImpl::Alias(e) => e.has_window_function(),
            ExpressionTypeImpl::Constant(e) => e.has_window_function(),
            ExpressionTypeImpl::BinaryOp(e) => e.has_window_function(),
            ExpressionTypeImpl::UnaryOp(e) => e.has_window_function(),
            ExpressionTypeImpl::Star(e) => e.has_window_function(),
            ExpressionTypeImpl::FuncCall(e) => e.has_window_function(),
            ExpressionTypeImpl::AggCall(e) => e.has_window_function(),
            ExpressionTypeImpl::Window(e) => e.has_window_function(),

            // TODO - throw unreachable
            ExpressionTypeImpl::Invalid => false,
        }
    }

    fn try_parse_from_expr(expr: &Expr, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized,
    {
        fallback_on_incompatible_2_args!(try_parse_from_expr, expr, binder, {
            ColumnRef,
            Constant,
            AliasExpr,
            BinaryOpExpr,
            UnaryOpExpr,
            StarExpr,
            WindowExpr,
            AggCallExpr,
            FuncCallExpr
        });

        Err(ParseASTError::IncompatibleType)
    }
}
