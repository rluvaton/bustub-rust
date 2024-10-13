use std::sync::Arc;

pub struct BoundSubqueryRef {

}

pub type CTEList = Vec<Arc<BoundSubqueryRef>>;
