
///
/// Tuple format:
/// ---------------------------------------------------------------------
/// | FIXED-SIZE or VARIED-SIZED OFFSET | PAYLOAD OF VARIED-SIZED FIELD |
/// ---------------------------------------------------------------------
///
/// TODO - implement src/include/storage/table/tuple.h
#[derive(Clone)]
pub struct Tuple {
    data: Vec<u8>
}

impl Tuple {
    pub fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn get_length(&self) -> u32 {
        self.data.len() as u32
    }
}
