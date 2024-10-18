use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use tuple::Tuple;
use crate::expression_type::ExpressionType;

pub(crate) const NO_CHILDREN: &[ExpressionRef] = &[];

// TODO - remove RC
pub type ExpressionRef = Rc<ExpressionType>;

/**
 * AbstractExpression is the base class of all the expressions in the system.
 * Expressions are modeled as trees, i.e. every expression may have a variable number of children.
 */
pub trait Expression: Clone + Display + Debug + Into<ExpressionType> {

    /** @return The value obtained by evaluating the tuple with the given schema */
    fn evaluate(&self, tuple: &Tuple, schema: &Schema) -> Value;

    /**
     * Returns the value obtained by evaluating a JOIN.
     * @param left_tuple The left tuple
     * @param left_schema The left tuple's schema
     * @param right_tuple The right tuple
     * @param right_schema The right tuple's schema
     * @return The value obtained by evaluating a JOIN on the left and right
     */
    fn evaluate_join(&self, left_tuple: &Tuple, left_schema: &Schema, right_tuple: &Tuple, right_schema: &Schema) -> Value;

    /** @return the children of this expression, ordering may matter */
    fn get_children(&self) -> &[ExpressionRef];

    /** @return the child_idx'th child of this expression */
    fn get_child_at(&self, child_idx: usize) -> &ExpressionRef {
        &self.get_children()[child_idx]
    }

    /** @return the type of this expression if it were to be evaluated */
    fn get_return_type(&self) -> DBTypeId;

    fn into_ref(self) -> ExpressionRef {
        Rc::new(self.into())
    }
}

