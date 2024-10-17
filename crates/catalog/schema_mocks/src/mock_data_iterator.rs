use data_types::Value;
use generics::Shuffle;
use rid::RID;
use crate::MockTableName;

type GeneratorFunction = Box<dyn Fn(usize) -> Vec<Value>>;

pub struct MockDataIterator {
    len: usize,
    cursor: usize,
    shuffled_idx: Vec<usize>,
    f: GeneratorFunction,
}

impl From<MockTableName> for MockDataIterator {
    fn from(value: MockTableName) -> Self {
        let f: GeneratorFunction = match value {
            MockTableName::Table1 => Box::new(|index| vec![
                Value::from(index as i32),
                Value::from(index as i32 * 100),
            ]),
            // MockTableName::Table2 => {}
            // MockTableName::Table3 => {}
            // MockTableName::TableTas2022 => {}
            // MockTableName::TableTas2023 => {}
            // MockTableName::TableTas2023Fall => {}
            // MockTableName::AggInputSmall => {}
            // MockTableName::AggInputBig => {}
            // MockTableName::TableSchedule2022 => {}
            // MockTableName::TableSchedule2023 => {}
            MockTableName::Table123 => Box::new(|index| vec![Value::from(index as i32)]),
            // MockTableName::Graph => {}
            // MockTableName::T1 => {}
            // MockTableName::T41M => {}
            // MockTableName::T51M => {}
            // MockTableName::T61M => {}
            // MockTableName::T7 => {}
            // MockTableName::T8 => {}
            // MockTableName::T9 => {}
            _ => unreachable!()
        };

        Self {
            len: value.get_size_of(),
            cursor: 0,
            shuffled_idx: if value.is_shuffled() { (0..value.get_size_of()).shuffle() } else { vec![] },
            f,
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

