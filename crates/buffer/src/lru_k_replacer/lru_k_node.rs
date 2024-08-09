use common::config::FrameId;

pub struct LRUKNode {
    /** History of last seen K timestamps of this page. Least recent timestamp stored in front. */

    // Remove #[allow(dead_code)] if you start using them. Feel free to change the member variables as you want.

    // in cpp it was std::list<size_t>
    #[allow(dead_code)]
    pub(crate) history: Vec<isize>,

    #[allow(dead_code)]
    pub(crate) k: isize,

    #[allow(dead_code)]
    pub(crate) fid: FrameId,

    #[allow(dead_code)]
    pub(crate) current_timestamp: isize,

    // TODO - set default to false
    #[allow(dead_code)]
    pub(crate) is_evictable: bool,
}
