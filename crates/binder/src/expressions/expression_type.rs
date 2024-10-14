use crate::expressions::{Alias, ColumnRef, Constant, Expression};

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


#[derive(Debug, PartialEq)]
pub enum ExpressionTypeImpl {
    ColumnRef(ColumnRef), // < A column in a table.
    Constant(Constant),
    Alias(Alias),     // < Alias expression type.
    Invalid
}

impl From<ExpressionTypeImpl> for ExpressionType {
    fn from(value: ExpressionTypeImpl) -> Self {
        match value {
            ExpressionTypeImpl::ColumnRef(_) => ExpressionType::ColumnRef,
            ExpressionTypeImpl::Constant(_) => ExpressionType::Constant,
            ExpressionTypeImpl::Alias(_) => ExpressionType::Alias,
            ExpressionTypeImpl::Invalid => ExpressionType::Invalid
        }
    }
}

impl Expression for ExpressionTypeImpl {

    fn has_aggregation(&self) -> bool {
        match &self {
            ExpressionTypeImpl::ColumnRef(e) => e.has_aggregation(),
            ExpressionTypeImpl::Alias(e) => e.has_aggregation(),
            ExpressionTypeImpl::Constant(e) => e.has_aggregation(),

            // TODO - throw unreachable
            ExpressionTypeImpl::Invalid => false,
        }
    }

    fn has_window_function(&self) -> bool {
        match &self {
            ExpressionTypeImpl::ColumnRef(e) => e.has_window_function(),
            ExpressionTypeImpl::Alias(e) => e.has_window_function(),
            ExpressionTypeImpl::Constant(e) => e.has_window_function(),

            // TODO - throw unreachable
            ExpressionTypeImpl::Invalid => false,
        }
    }
}
