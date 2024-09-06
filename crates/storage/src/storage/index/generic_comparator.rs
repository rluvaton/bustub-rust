use crate::catalog::Schema;
use crate::storage::{Comparator, GenericKey};
use common::types::CmpBool;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::sync::Arc;

pub struct GenericComparator<const KEY_SIZE: usize> {
    schema: Arc<Schema>,
}

impl<const KEY_SIZE: usize> Comparator<GenericKey<KEY_SIZE>> for GenericComparator<KEY_SIZE> {
    ///
    /// Function object returns true if lhs < rhs, used for trees
    ///
    fn cmp(&self, lhs: &GenericKey<KEY_SIZE>, rhs: &GenericKey<KEY_SIZE>) -> Ordering {
        let column_count = self.schema.get_column_count();

        for i in 0..column_count {
            let lhs_value = lhs.to_value(&self.schema, i);
            let rhs_value = rhs.to_value(&self.schema, i);

            if (matches!(lhs_value.compare_less_than(&rhs_value), CmpBool::TRUE)) {
                return Ordering::Less;
            }
            if (matches!(lhs_value.compare_greater_than(&rhs_value), CmpBool::TRUE)) {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}

impl<const KEY_SIZE: usize> From<Arc<Schema>> for GenericComparator<KEY_SIZE> {
    fn from(value: Arc<Schema>) -> Self {
        GenericComparator {
            schema: value
        }
    }
}
