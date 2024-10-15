use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use common::PageKey;
use data_types::Value;
use rid::RID;


pub trait TupleExt {
    // constructor for creating a new tuple based on input value
    fn from_input(values: Vec<Value>, ) -> Self;
    // TODO(Amadou): It does not look like nulls are supported. Add a null bitmap?

}
