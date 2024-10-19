use std::fmt::Debug;
use std::sync::Arc;
use catalog_schema::Schema;
use rid::RID;
use tuple::Tuple;
use crate::context::ExecutorContext;
use crate::executors::ExecutorImpl;

// TODO - avoid Rc
pub(crate) type ExecutorRef<'a> = Box<ExecutorImpl<'a>>;

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

pub(crate) trait Executor<'a>: ExecutorMetadata + Iterator<Item = ExecutorItem> + Into<ExecutorImpl<'a>> where Self: Sized {
    //
    fn into_ref(self) -> ExecutorRef<'a> {
        Box::new(self.into())
    }
}
