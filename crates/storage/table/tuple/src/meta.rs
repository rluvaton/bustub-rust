use common::config::Timestamp;

//noinspection RsAssertEqual
const _: () = {
    assert!(size_of::<TupleMeta>() == 16);
};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct TupleMeta {
    /// the ts / txn_id of this tuple. In project 3, simply set it to 0.
    pub ts: Timestamp,

    /// marks whether this tuple is marked removed from table heap.
    pub is_deleted: bool,
}
