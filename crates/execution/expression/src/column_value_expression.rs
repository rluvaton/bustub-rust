use crate::expression_type::ExpressionType;
use crate::traits::{Expression, ExpressionRef, NO_CHILDREN};
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use std::fmt::{Debug, Display, Formatter};
use tuple::Tuple;


/// ColumnValueExpression maintains the tuple index and column index relative to a particular schema or join.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnValueExpression {
    /// From Abstract expression

    /** The return type of this expression. */
    ret_type: DBTypeId,

    //

    /** Tuple index 0 = left side of join, tuple index 1 = right side of join */
    tuple_idx: usize,

    /** Column index refers to the index within the schema of the tuple, e.g. schema {A,B,C} has indexes {0,1,2} */
    col_idx: usize,
}

impl ColumnValueExpression {

    /**
     * ColumnValueExpression is an abstraction around "Table.member" in terms of indexes.
     * @param tuple_idx {tuple index 0 = left side of join, tuple index 1 = right side of join}
     * @param col_idx the index of the column in the schema
     * @param ret_type the return type of the expression
     */
    pub fn new(tuple_idx: usize, col_idx: usize, ret_type: DBTypeId) -> Self {
        Self {
            ret_type,
            tuple_idx,
            col_idx,
        }
    }

    pub fn get_tuple_idx(&self) -> usize { self.tuple_idx }
    pub fn get_col_idx(&self) -> usize { self.col_idx }
}


impl Display for ColumnValueExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}.{}", self.tuple_idx, self.col_idx)
    }
}

impl Into<ExpressionType> for ColumnValueExpression {
    fn into(self) -> ExpressionType {
        ExpressionType::ColumnValue(self)
    }
}

impl Expression for ColumnValueExpression {
    fn evaluate(&self, tuple: &Tuple, schema: &Schema) -> Value {
        tuple.get_value(schema, self.col_idx)
    }

    fn evaluate_join(&self, left_tuple: &Tuple, left_schema: &Schema, right_tuple: &Tuple, right_schema: &Schema) -> Value {
        if self.tuple_idx == 0 {
            left_tuple.get_value(left_schema, self.col_idx)
        } else {
            right_tuple.get_value(right_schema, self.col_idx)
        }
    }

    fn get_children(&self) -> &[ExpressionRef] {
        NO_CHILDREN
    }

    fn get_return_type(&self) -> DBTypeId {
        self.ret_type
    }
}
