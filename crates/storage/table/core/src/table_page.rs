use pages::{PageId, PAGE_SIZE};
use rid::RID;
use tuple::{Tuple, TupleMeta};

type TupleInfo = (u16, u16, TupleMeta);

const TABLE_PAGE_HEADER_SIZE: usize = 8;
const TUPLE_INFO_SIZE: usize = 24;

//noinspection RsAssertEqual
const _: () = {
    assert!(size_of::<PageId>() == 4);
    assert!(size_of::<TablePage>() == TABLE_PAGE_HEADER_SIZE);
    assert!(size_of::<TupleInfo>() == TUPLE_INFO_SIZE);
};


/// Slotted page format:
///  ---------------------------------------------------------
///  | HEADER | ... FREE SPACE ... | ... INSERTED TUPLES ... |
///  ---------------------------------------------------------
///                                ^
///                                free space pointer
///
///  Header format (size in bytes):
///  ----------------------------------------------------------------------------
///  | NextPageId (4)| NumTuples(2) | NumDeletedTuples(2) |
///  ----------------------------------------------------------------------------
///  ----------------------------------------------------------------
///  | Tuple_1 offset+size (4) | Tuple_2 offset+size (4) | ... |
///  ----------------------------------------------------------------
///
/// Tuple format:
/// | meta | data |
///
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct TablePage {
    // Start of the page pointer
    page_start: [u8; 0],

    next_page_id: PageId,
    num_tuples: u16,
    num_deleted_tuples: u16,

    // Until end of page data
    tuple_info: [TupleInfo; 0],
}

impl TablePage {
    /// Initialize the TablePage header.
    pub fn init(&mut self) {
        self.next_page_id = 0;
        self.num_tuples = 0;
        self.num_deleted_tuples = 0;
    }

    /** @return number of tuples in this page */
    pub fn get_num_tuples(&self) -> u32 { self.num_tuples as u32 }

    /** @return the page ID of the next table page */
    pub fn get_next_page_id(&self) -> PageId { self.next_page_id }

    /** Set the page id of the next page in the table. */
    pub fn set_next_page_id(&mut self, next_page_id: PageId) { self.next_page_id = next_page_id; }

    /** Get the next offset to insert, return nullopt if this tuple cannot fit in this page */
    pub fn get_next_tuple_offset(&self, meta: &TupleMeta, tuple: &Tuple) -> Option<usize> {
        let slot_end_offset: isize;

        if self.num_tuples > 0 {
            let (offset, _, _) = self.tuple_info[self.num_tuples as usize - 1];

            slot_end_offset = offset as isize;
        } else {
            slot_end_offset = PAGE_SIZE as isize;
        }

        let tuple_offset = slot_end_offset - tuple.get_length() as isize;
        let offset_size = (TABLE_PAGE_HEADER_SIZE as isize) + (TUPLE_INFO_SIZE as isize) * (self.num_tuples as isize + 1);

        if tuple_offset < offset_size {
            None
        } else {
            Some(tuple_offset as usize)
        }
    }

    /**
     * Insert a tuple into the table.
     * @param tuple tuple to insert
     * @return true if the insert is successful (i.e. there is enough space)
     */
    pub fn insert_tuple(&mut self, meta: &TupleMeta, tuple: &Tuple) -> Option<u16> {
        let tuple_offset = self.get_next_tuple_offset(meta, tuple)?;
        let tuple_id = self.num_tuples;
        self.set_tuple(tuple_id as u32, tuple_offset as u16, tuple.get_length() as u16, tuple, *meta);
        self.num_tuples += 1;

        Some(tuple_id)
    }

    /**
     * Update a tuple.
     */
    pub fn update_tuple_meta(&mut self, meta: &TupleMeta, rid: &RID) {
        let tuple_id = rid.get_slot_num();

        assert!(tuple_id < self.num_tuples as u32, "Tuple ID out of range");
        let (offset, size, old_meta) = self.tuple_info[self.num_tuples as usize - 1];

        if !old_meta.is_deleted && meta.is_deleted {
            self.num_deleted_tuples += 1;
        }

        self.tuple_info[tuple_id as usize] = (offset, size, *meta)

    }

    /**
     * Read a tuple from a table.
     */
    pub fn get_tuple(&self, rid: &RID) -> (TupleMeta, Tuple) {
        let tuple_id = rid.get_slot_num();

        assert!(tuple_id < self.num_tuples as u32, "Tuple ID out of range");
        let (offset, size, meta) = self.tuple_info[self.num_tuples as usize - 1];

        let mut tuple = Tuple::from(&self.page_start.as_slice()[offset as usize..offset as usize + size as usize]);

        tuple.set_rid(*rid);

        (meta, tuple)
    }

    /**
     * Read a tuple meta from a table.
     */
    pub fn get_tuple_meta(&self, rid: &RID) -> TupleMeta {
        let tuple_id = rid.get_slot_num();
        assert!(tuple_id < self.num_tuples as u32, "Tuple ID out of range");
        let (_, _, meta) = self.tuple_info[self.num_tuples as usize - 1];

        meta
    }

    /**
     * Update a tuple in place.
     */
    pub unsafe fn update_tuple_in_place(&mut self, meta: &TupleMeta, tuple: &Tuple, rid: RID) {
        let tuple_id = rid.get_slot_num();
        assert!(tuple_id < self.num_tuples as u32, "Tuple ID out of range");
        let (offset, size, old_meta) = self.tuple_info[tuple_id as usize];

        assert_eq!(size as u32, tuple.get_length(), "Tuple size mismatch");

        if !old_meta.is_deleted && meta.is_deleted {
            self.num_deleted_tuples += 1;
        }

        self.set_tuple(tuple_id, offset, size, tuple, *meta);
    }

    fn set_tuple(&mut self, tuple_id: u32, offset: u16, size: u16, tuple: &Tuple, meta: TupleMeta) {
        self.tuple_info[tuple_id as usize] = (offset, size, meta);
        self.page_start.as_mut_slice()[offset as usize..offset as usize + tuple.get_length() as usize].copy_from_slice(tuple.get_data());
    }
}
