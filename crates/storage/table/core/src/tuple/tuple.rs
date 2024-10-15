use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use common::PageKey;
use rid::RID;

///
/// Tuple format:
/// ---------------------------------------------------------------------
/// | FIXED-SIZE or VARIED-SIZED OFFSET | PAYLOAD OF VARIED-SIZED FIELD |
/// ---------------------------------------------------------------------
///
/// TODO - implement src/include/storage/table/tuple.h
#[derive(Clone, Debug, Hash)]
pub struct Tuple {
    rid: RID,
    data: Vec<u8>
}

impl Tuple {
    // table heap tuple
    pub fn new(rid: RID) -> Self {
        Self {
            rid,
            data: vec![]
        }
    }

    pub fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn get_length(&self) -> u32 {
        self.data.len() as u32
    }
}


impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl PageKey for Tuple {

}


// to create a dummy tuple
impl Default for Tuple {
    fn default() -> Self {
        Self {
            rid: RID::default(),
            data: vec![]
        }
    }
}
