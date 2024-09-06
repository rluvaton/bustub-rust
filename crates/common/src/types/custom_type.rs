use crate::types::{TypeId, Value};

pub enum CmpBool {
    FALSE,
    TRUE,
    NULL,
}

/// This is the Type class from `src/include/type/type.h`
pub struct CustomType {}

impl CustomType {
    pub fn get_instance(type_id: TypeId) -> CustomType {
        todo!()
    }


    /// Comparison functions
    ///
    /// NOTE:
    /// We could get away with only compare_less_than() being purely virtual, since
    /// the remaining comparison functions can derive their logic from
    /// compare_less_than(). For example:
    ///
    ///    compare_equals(o) = !compare_less_than(o) && !o.compare_less_than(this)
    ///    compare_not_equals(o) = !compare_equals(o)
    ///    compare_less_than_equals(o) = compare_less_than(o) || compare_equals(o)
    ///    compare_greater_than(o) = !compare_less_than_equals(o)
    ///    ... etc. ...
    ///
    /// We don't do this for two reasons:
    /// (1) The redundant calls to compare_less_than() may be a performance problem,
    ///     and since Value is a core component of the execution engine, we want to
    ///     make it as performant as possible.
    /// (2) Keep the interface consistent by making all functions purely virtual.
    pub fn compare_equals(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }
    pub fn compare_not_equals(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }
    pub fn compare_less_than(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }
    pub fn compare_less_than_equals(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }
    pub fn compare_greater_than(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }
    pub fn compare_greater_than_equals(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }
}
