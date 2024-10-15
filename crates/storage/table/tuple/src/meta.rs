use common::config::Timestamp;

//noinspection RsAssertEqual
const _: () = {
    assert!(size_of::<TupleMeta>() == 16);
};

#[derive(Clone, Debug, PartialEq)]
pub struct TupleMeta {
    /// the ts / txn_id of this tuple. In project 3, simply set it to 0.
    ts: Timestamp,

    /// marks whether this tuple is marked removed from table heap.
    is_deleted: bool,
}
