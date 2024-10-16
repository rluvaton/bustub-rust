use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use rid::RID;
use tuple::Tuple;
use crate::context::ExecutorContext;

// TODO - avoid Rc
pub(crate) type ExecutorRef = Box<dyn Executor>;

pub(crate) type ExecutorItem = (Tuple, RID);

/**
 * The AbstractExecutor implements the Volcano tuple-at-a-time iterator model.
 * This is the base class from which all executors in the BustTub execution
 * engine inherit, and defines the minimal interface that all executors support.
 */
pub(crate) trait ExecutorMetadata: Debug {

    /** @return The schema of the tuples that this executor produces */
    fn get_output_schema(&self) -> Arc<Schema>;

    /** @return The executor context in which this executor runs */
    fn get_context(&self) -> &ExecutorContext;
}

pub(crate) trait Executor: ExecutorMetadata + Iterator<Item = ExecutorItem> where Self: 'static {

    fn into_ref(self) -> ExecutorRef where Self: Sized {
        Box::new(self)
    }
}
