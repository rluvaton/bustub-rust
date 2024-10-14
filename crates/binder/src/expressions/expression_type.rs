use sqlparser::ast::Expr;
use crate::Binder;
use crate::expressions::{Alias, UnaryOpExpr, ColumnRef, Constant, Expression, BinaryOpExpr};
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
    Alias(Alias),     // < Alias expression type.
    BinaryOp(BinaryOpExpr),
    UnaryOp(UnaryOpExpr),
    Invalid
}

impl ExpressionTypeImpl {
    pub fn parse_expression_list(list: &Vec<Expr>, binder: &mut Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
        todo!()
    }
}

impl TryFrom<&Expr> for ExpressionTypeImpl {
    type Error = ParseASTError;

    fn try_from(value: &Expr) -> Result<Self, Self::Error> {
        todo!()
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

            // TODO - throw unreachable
            ExpressionTypeImpl::Invalid => false,
        }
    }


    fn try_parse_from_expr(expr: &Expr, binder: &mut Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        // TODO - test every one
        unreachable!()
    }
}
