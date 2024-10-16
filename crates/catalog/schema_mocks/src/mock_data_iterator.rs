use data_types::Value;
use rid::RID;
use crate::MockTableName;

pub struct MockDataIterator {
    len: usize,
    cursor: usize,
    shuffled_idx: Vec<usize>,
    f: Box<dyn Fn(usize)-> Vec<Value>>
}

impl From<MockTableName> for MockDataIterator {
    fn from(value: MockTableName) -> Self {
        todo!()
    }
}
impl Iterator for MockDataIterator {
    type Item = (Vec<Value>, RID);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.len {
            return None;
        }

        let index = if self.shuffled_idx.is_empty() {
            self.cursor
        } else {
            self.shuffled_idx[self.cursor]
        };

        let values = (self.f)(index);

        self.cursor += 1;

        Some((values, RID::from(0)))
    }
}

