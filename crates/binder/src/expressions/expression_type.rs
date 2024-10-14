use crate::expressions::{Alias, ColumnRef};

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
    Alias(Alias),     // < Alias expression type.
    Invalid
}
