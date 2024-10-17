use data_types::Value;
use generics::Shuffle;
use rid::RID;
use crate::MockTableName;

pub struct MockDataIterator {
    len: usize,
    cursor: usize,
    shuffled_idx: Vec<usize>,
    f: Box<dyn Fn(usize) -> Vec<Value>>,
}

impl From<MockTableName> for MockDataIterator {
    fn from(value: MockTableName) -> Self {
        match value {
            MockTableName::Table123 => {
                Self {
                    len: value.get_size_of(),
                    cursor: 0,
                    shuffled_idx: if value.is_shuffled() { (0..value.get_size_of()).shuffle() } else { vec![] },
                    f: Box::new(|index| {
                        vec![Value::from(index as i32)]
                    })
                }
            }
            _ => unimplemented!()
        }
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
