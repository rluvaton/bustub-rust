use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::config::{PageId, INVALID_PAGE_ID};
use crate::page_traits::PageValue;

#[derive(Clone, PartialEq)]
pub struct RID {
    // Default is Invalid
    page_id: PageId,

    // logical offset from 0, 1...
    slot_num: u32,
}

impl RID {
    /// Creates a new Record Identifier for the given page identifier and slot number.
    ///
    /// # Arguments
    ///
    /// * `page_id`: page identifier
    /// * `slot_num`: slot number
    ///
    /// returns: RID
    pub fn new(page_id: PageId, slot_num: u32) -> Self {
        Self {
            page_id,
            slot_num,
        }
    }

    pub fn get(&self) -> i64 {
        self.into()
    }

    pub fn get_page_id(&self) -> PageId {
        self.page_id
    }

    pub fn get_slot_num(&self) -> u32 {
        self.slot_num
    }

    pub fn set(&mut self, page_id: PageId, slot_num: u32) {
        self.page_id = page_id;
        self.slot_num = slot_num;
    }
}

/// The default constructor creates an invalid RID!
impl Default for RID {
    fn default() -> Self {
        Self::new(INVALID_PAGE_ID, 0)
    }
}

impl Debug for RID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "page_id: {} slot_num: {}", self.page_id, self.slot_num)
    }
}

impl Display for RID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "page_id: {} slot_num: {}", self.page_id, self.slot_num)
    }
}

impl From<i64> for RID {
    fn from(value: i64) -> Self {
        RID::new(
            (value >> 32) as PageId,
            value as u32,
        )
    }
}
impl Into<i64> for RID {
    fn into(self) -> i64 {
        ((self.page_id as i64) << 32) | self.slot_num as i64
    }
}
impl Into<i64> for &RID {
    fn into(self) -> i64 {
        ((self.page_id as i64) << 32) | self.slot_num as i64
    }
}

impl PageValue for RID {}


impl Hash for RID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}
