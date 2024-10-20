use std::fmt::{Debug, Formatter};
use pages::{PageId, INVALID_PAGE_ID, PAGE_SIZE};
use rid::RID;
use tuple::{Tuple, TupleMeta};

type TupleInfo = (u16, u16, TupleMeta);

const TABLE_PAGE_HEADER_SIZE: usize = 8;
const TUPLE_INFO_SIZE: usize = 24;
const TABLE_PAGE_DATA_WITHOUT_HEADER: usize = PAGE_SIZE - TABLE_PAGE_HEADER_SIZE;

//noinspection RsAssertEqual
const _: () = {
    assert!(size_of::<PageId>() == 4);
    assert!(size_of::<TupleInfo>() == TUPLE_INFO_SIZE);
    assert!(size_of::<TablePage>() == PAGE_SIZE);
};

// This is the maximum length of a tuple that can be inserted in a table page when the table page is empty
pub const LARGEST_TUPLE_SIZE_WITHOUT_OVERFLOW: usize = TABLE_PAGE_DATA_WITHOUT_HEADER - TUPLE_INFO_SIZE;


/// ```plain
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
/// ```
///
#[derive(PartialEq)]
#[repr(C)]
pub struct TablePage {
    next_page_id: PageId,
    num_tuples: u16,
    num_deleted_tuples: u16,

    // Because there is no easy way to model the table page in this struct
    // (as we want to add to the end the tuple data and add to the beginning tuple info which are both unsized)
    // we are manually handling tuple info/data locations
    rest: [u8; TABLE_PAGE_DATA_WITHOUT_HEADER],
}

impl TablePage {
    /// Initialize the TablePage header.
    pub fn init(&mut self) {
        self.next_page_id = 0;
        self.num_tuples = 0;
        self.num_deleted_tuples = 0;
    }

    /** @return number of tuples in this page */
    pub(crate) fn get_num_tuples(&self) -> u32 { self.num_tuples as u32 }

    /** @return the page ID of the next table page */
    pub(crate) fn get_next_page_id(&self) -> PageId { self.next_page_id }

    /** Set the page id of the next page in the table. */
    pub(crate) fn set_next_page_id(&mut self, next_page_id: PageId) { self.next_page_id = next_page_id; }

    /** Get the next offset to insert, return nullopt if this tuple cannot fit in this page */
    pub(crate) fn get_next_tuple_offset(&self, _meta: &TupleMeta, tuple: &Tuple) -> Option<usize> {
        let slot_end_offset: isize;

        if self.num_tuples > 0 {
            let (offset, _, _) = unsafe { self.get_tuple_info(self.num_tuples as usize - 1) };

            slot_end_offset = offset as isize;
        } else {
            slot_end_offset = TABLE_PAGE_DATA_WITHOUT_HEADER as isize;
        }

        let tuple_offset = slot_end_offset - tuple.get_length() as isize;
        let offset_size = (TUPLE_INFO_SIZE as isize) * (self.num_tuples as isize + 1);

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
        // TODO - add check that there is enough space
        let tuple_offset = self.get_next_tuple_offset(meta, tuple)?;
        let tuple_id = self.num_tuples;
        self.num_tuples += 1;
        self.set_tuple(tuple_id as u32, tuple_offset as u16, tuple.get_length() as u16, tuple, *meta);

        Some(tuple_id)
    }

    /**
     * Update a tuple.
     */
    pub fn update_tuple_meta(&mut self, meta: &TupleMeta, rid: &RID) {
        let tuple_id = rid.get_slot_num();

        assert!(tuple_id < self.num_tuples as u32, "Tuple ID out of range");
        let (offset, size, old_meta) = unsafe {
            self.get_tuple_info(self.num_tuples as usize - 1)
        };

        if !old_meta.is_deleted && meta.is_deleted {
            self.num_deleted_tuples += 1;
        }

        unsafe {
            *self.get_tuple_info_mut(tuple_id as usize) = (offset, size, *meta)
        }
    }

    /**
     * Read a tuple from a table.
     */
    pub fn get_tuple(&self, rid: &RID) -> (TupleMeta, Tuple) {
        let tuple_id = rid.get_slot_num();

        assert!(tuple_id < self.num_tuples as u32, "Tuple ID ({tuple_id}) out of range 0 to {}", self.num_tuples);
        let (offset, size, meta) = unsafe {
            self.get_tuple_info(tuple_id as usize)
        };

        let mut tuple = Tuple::from(self.get_tuple_data_slice(offset as usize, size as usize));

        tuple.set_rid(*rid);

        (meta, tuple)
    }

    /**
     * Read a tuple meta from a table.
     */
    pub fn get_tuple_meta(&self, rid: &RID) -> TupleMeta {
        let tuple_id = rid.get_slot_num();
        assert!(tuple_id < self.num_tuples as u32, "Tuple ID out of range");

        let (_, _, meta) = unsafe {
            self.get_tuple_info(tuple_id as usize)
        };

        meta
    }

    /**
     * Update a tuple in place.
     */
    pub unsafe fn update_tuple_in_place(&mut self, meta: &TupleMeta, tuple: &Tuple, rid: &RID) {
        let tuple_id = rid.get_slot_num();
        assert!(tuple_id < self.num_tuples as u32, "Tuple ID out of range");

        let (offset, size, old_meta) = unsafe {
            self.get_tuple_info(tuple_id as usize)
        };

        assert_eq!(size as u32, tuple.get_length(), "Tuple size mismatch");

        if !old_meta.is_deleted && meta.is_deleted {
            self.num_deleted_tuples += 1;
        }

        self.set_tuple(tuple_id, offset, size, tuple, *meta);
    }

    fn set_tuple(&mut self, tuple_id: u32, offset: u16, size: u16, tuple: &Tuple, meta: TupleMeta) {
        unsafe {
            *self.get_tuple_info_mut(tuple_id as usize) = (offset, size, meta);
        }
        self.get_mut_tuple_data_slice(offset as usize, tuple.get_length() as usize).copy_from_slice(tuple.get_data());
    }

    fn get_all_tuples(&self) -> Vec<(TupleMeta, Tuple)> {
        (0..self.num_tuples)
            .into_iter()
            .map(|slot_num| RID::new(INVALID_PAGE_ID, slot_num as u32))
            .map(|rid| self.get_tuple(&rid))
            .collect()
    }

    // Functions to handle with the raw data in the rest
    // -----------------------------------------------

    /// Assert the following guarantees
    /// 1. the index is smaller than the `self.num_tuples`
    /// 2. The location of the tuple info is not outside the data bounds
    fn assert_valid_tuple_index_before_raw_access(&self, index: usize) {
        assert!(index < self.num_tuples as usize, "tuple index must be between 0 and {}, instead got {index}", self.num_tuples);
        assert!(index * TUPLE_INFO_SIZE < TABLE_PAGE_DATA_WITHOUT_HEADER, "tuple location must be between 0 and {}, instead got {}", TABLE_PAGE_DATA_WITHOUT_HEADER, index * TUPLE_INFO_SIZE);
    }

    /// Get tuple info at index
    ///
    /// # Safety
    /// The caller must ensure that:
    /// 1. the index is smaller than the `self.num_tuples`
    /// 2. The location of the tuple info is not outside the data bounds
    unsafe fn get_tuple_info(&self, index: usize) -> TupleInfo {
        self.assert_valid_tuple_index_before_raw_access(index);

        let pos = index * TUPLE_INFO_SIZE;
        let tuple_info_data = &self.rest[pos..pos + TUPLE_INFO_SIZE];

        let a = tuple_info_data.as_ptr().cast::<TupleInfo>();

        *a
    }

    /// Get Mutable referance for tuple info at index
    ///
    /// # Safety
    /// The caller must ensure that:
    /// 1. the index is smaller than the `self.num_tuples`
    /// 2. The location of the tuple info is not outside the data bounds
    unsafe fn get_tuple_info_mut(&mut self, index: usize) -> &mut TupleInfo {
        self.assert_valid_tuple_index_before_raw_access(index);

        let pos = index * TUPLE_INFO_SIZE;
        let tuple_info_data = &mut self.rest[pos..pos + TUPLE_INFO_SIZE];

        let a = tuple_info_data.as_mut_ptr().cast::<TupleInfo>();

        &mut *a
    }

    /// Assert the following guarantees
    /// 2. The `offset` is after the last tuple location
    fn assert_valid_tuple_data_before_raw_access(&self, offset: usize) {
        assert!(offset >= self.num_tuples as usize * TUPLE_INFO_SIZE, "offset must not be in the tuples info region");
    }

    /// Get tuple data slice
    ///
    /// # Safety
    /// The caller must ensure that:
    /// 1. The `offset` + `length` is not outside the data bounds
    /// 2. The `offset` is after the last tuple location
    fn get_tuple_data_slice(&self, offset: usize, length: usize) -> &[u8] {
        self.assert_valid_tuple_data_before_raw_access(offset);

        &self.rest[offset..offset + length]
    }

    /// Get Mutable referance for tuple info at index
    ///
    /// # Safety
    /// The caller must ensure that:
    /// 1. The `offset` + `length` is not outside the data bounds
    /// 2. The `offset` is after the last tuple location
    fn get_mut_tuple_data_slice(&mut self, offset: usize, length: usize) -> &mut [u8] {
        self.assert_valid_tuple_data_before_raw_access(offset);

        &mut self.rest[offset..offset + length]
    }
}

impl Debug for TablePage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TablePage")
            .field("next_page_id", &self.next_page_id)
            .field("num_tuples", &self.num_tuples)
            .field("num_deleted_tuples", &self.num_deleted_tuples)
            .field("tuples", &self.get_all_tuples())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::TablePage;
    use catalog_schema::{Column, Schema};
    use data_types::{BigIntUnderlyingType, DBTypeId, IntUnderlyingType, SmallIntUnderlyingType, TinyIntUnderlyingType, Value};
    use pages::{Page, PageId, PAGE_SIZE};
    use rand::rngs::ThreadRng;
    use rand::Rng;
    use rid::RID;
    use tuple::{Tuple, TupleMeta};

    #[test]
    fn should_allow_inserting_tuples() {
        let mut rng = rand::thread_rng();

        let columns = vec![
            Column::new_fixed_size("my_col1".to_string(), DBTypeId::INT),
            Column::new_fixed_size("my_col2".to_string(), DBTypeId::BOOLEAN),
            Column::new_fixed_size("my_col3".to_string(), DBTypeId::BIGINT),
            Column::new_fixed_size("my_col4".to_string(), DBTypeId::TINYINT),
            Column::new_fixed_size("my_col5".to_string(), DBTypeId::SMALLINT),
        ];
        // Tuple should take 16 bytes
        assert_eq!(columns.iter().fold(0, |size, item| size + item.get_length()), 16);

        let schema = Schema::new(columns);

        fn generate_row_values(rng: &mut ThreadRng) -> Vec<Value> {
            let (
                my_col1,
                my_col2,
                my_col3,
                my_col4,
                my_col5,
            ): (IntUnderlyingType, bool, BigIntUnderlyingType, TinyIntUnderlyingType, SmallIntUnderlyingType) = rng.gen();
            vec![
                Value::from(my_col1),
                Value::from(my_col2),
                Value::from(my_col3),
                Value::from(my_col4),
                Value::from(my_col5),
            ]
        }

        let page = Page::default();
        let mut page_guard = page.write();
        let page_id = page_guard.get_page_id();

        fn insert_random_tuple(rng: &mut ThreadRng, schema: &Schema, table_page: &mut TablePage, page_id: PageId) -> Tuple {
            let mut tuple = Tuple::from_value(generate_row_values(rng), &schema);
            let rid = table_page.insert_tuple(&TupleMeta::invalid(), &tuple).expect("Must add");
            tuple.set_rid(RID::new(page_id, rid as u32));

            tuple
        }

        let table_page = page_guard.cast_mut::<TablePage>();

        assert_eq!(table_page.get_num_tuples(), 0);

        let tuple1 = insert_random_tuple(&mut rng, &schema, table_page, page_id);

        {
            let (_, found_tuple1) = table_page.get_tuple(tuple1.get_rid());
            assert_eq!(tuple1.get_data(), found_tuple1.get_data());
        }

        assert_eq!(table_page.get_num_tuples(), 1);

        let tuple2 = insert_random_tuple(&mut rng, &schema, table_page, page_id);
        {
            let (_, found_tuple2) = table_page.get_tuple(tuple2.get_rid());
            assert_eq!(tuple2.get_data(), found_tuple2.get_data());
        }

        assert_eq!(table_page.get_num_tuples(), 2);

        {
            let (_, found_tuple1) = table_page.get_tuple(tuple1.get_rid());
            assert_eq!(tuple1.get_data(), found_tuple1.get_data());
        }

        {
            let (_, found_tuple2) = table_page.get_tuple(tuple2.get_rid());
            assert_eq!(tuple2.get_data(), found_tuple2.get_data());
        }
    }

    #[test]
    fn full_page() {
        let mut rng = rand::thread_rng();

        let columns = vec![
            Column::new_fixed_size("my_col1".to_string(), DBTypeId::INT),
            Column::new_fixed_size("my_col2".to_string(), DBTypeId::BOOLEAN),
            Column::new_fixed_size("my_col3".to_string(), DBTypeId::BIGINT),
            Column::new_fixed_size("my_col4".to_string(), DBTypeId::TINYINT),
            Column::new_fixed_size("my_col5".to_string(), DBTypeId::SMALLINT),
        ];
        // Tuple should take 16 bytes
        assert_eq!(columns.iter().fold(0, |size, item| size + item.get_length()), 16);

        let schema = Schema::new(columns);

        fn generate_row_values(rng: &mut ThreadRng) -> Vec<Value> {
            let (
                my_col1,
                my_col2,
                my_col3,
                my_col4,
                my_col5,
            ): (IntUnderlyingType, bool, BigIntUnderlyingType, TinyIntUnderlyingType, SmallIntUnderlyingType) = rng.gen();
            vec![
                Value::from(my_col1),
                Value::from(my_col2),
                Value::from(my_col3),
                Value::from(my_col4),
                Value::from(my_col5),
            ]
        }

        let page = Page::default();
        let mut page_guard = page.write();
        let page_id = page_guard.get_page_id();

        fn try_insert_random_tuple(rng: &mut ThreadRng, schema: &Schema, table_page: &mut TablePage, page_id: PageId) -> Option<Tuple> {
            let mut tuple = Tuple::from_value(generate_row_values(rng), &schema);
            let rid = table_page.insert_tuple(&TupleMeta::invalid(), &tuple)?;
            tuple.set_rid(RID::new(page_id, rid as u32));

            Some(tuple)
        }

        let table_page = page_guard.cast_mut::<TablePage>();
        
        let mut tuples = vec![];
        
        let mut iteration = 0;
        
        loop {
            iteration += 1;
            let next_tuple = try_insert_random_tuple(&mut rng, &schema, table_page, page_id);
            
            if next_tuple.is_none() {
                break;
            }
            
            // This is here to ensure that we are not looping forever
            if iteration > PAGE_SIZE {
                panic!("Infinite loop when trying to insert tuple");
            }
            
            let next_tuple = next_tuple.unwrap();
            
            tuples.push(next_tuple);
        }
        
        for tuple in tuples {
            let (_, found_tuple) = table_page.get_tuple(tuple.get_rid());
            assert_eq!(tuple.get_data(), found_tuple.get_data());
        }
            
    }
}