use data_types::Value;
use generics::Shuffle;
use rid::RID;
use crate::constants::{COURSE_ON_DATE, GRAPH_NODE_CNT, TA_LIST_2022, TA_LIST_2023, TA_LIST_2023_FALL, TA_OH_2022, TA_OH_2023, TA_OH_2023_FALL};
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
            MockTableName::Table2 => Box::new(|index| vec![
                Value::from(format!("{}-\u{1F4A9}", index)), // The poop icon
                Value::from("\u{1F607}".repeat(index % 8)), // The innocent icon
            ]),
            MockTableName::Table3 => Box::new(|index| vec![
                Value::from(if index % 2 == 0 { Some(index as i32) } else { None }),
                Value::from(format!("{}-\u{1F4A9}", index)), // The poop icon
            ]),
            MockTableName::TableTas2022 => Box::new(|index| vec![
                Value::from(TA_LIST_2022[index]),
                Value::from(TA_OH_2022[index]),
            ]),
            MockTableName::TableTas2023 => Box::new(|index| vec![
                Value::from(TA_LIST_2023[index]),
                Value::from(TA_OH_2023[index]),
            ]),
            MockTableName::TableTas2023Fall => Box::new(|index| vec![
                Value::from(TA_LIST_2023_FALL[index]),
                Value::from(TA_OH_2023_FALL[index]),
            ]),
            MockTableName::TableSchedule2022 => Box::new(|index| vec![
                Value::from(COURSE_ON_DATE[index]),
                Value::from(if index == 1 || index == 3 { 1_i32 } else { 0_i32 }),
            ]),
            MockTableName::TableSchedule2023 => Box::new(|index| vec![
                Value::from(COURSE_ON_DATE[index]),
                Value::from(if index == 0 || index == 2 { 1_i32 } else { 0_i32 }),
            ]),
            MockTableName::AggInputSmall => Box::new(|index| vec![
                Value::from((index as i32 + 2) % 10),
                Value::from(index as i32),
                Value::from((index as i32 + 50) % 10),
                Value::from(index as i32 / 100),
                Value::from(233_i32),
                Value::from("\u{1F4A9}".repeat((index % 8) + 1)), // The poop icon
            ]),
            MockTableName::AggInputBig => Box::new(|index| vec![
                Value::from((index as i32 + 2) % 10),
                Value::from(index as i32),
                Value::from((index as i32 + 50) % 10),
                Value::from(index as i32 / 100),
                Value::from(233_i32),
                Value::from("\u{1F4A9}".repeat((index % 16) + 1)), // The poop icon
            ]),
            MockTableName::Table123 => Box::new(|index| vec![Value::from(index as i32)]),
            MockTableName::Graph => Box::new(|index| {
                let src = index as i32 % GRAPH_NODE_CNT as i32;
                let dst = index as i32 % GRAPH_NODE_CNT as i32;

                vec![
                    Value::from(src),
                    Value::from(dst),
                    Value::from(format!("{:03}", src)),
                    Value::from(format!("{:03}", dst)),
                    Value::from(if src == dst { None } else { Some(1_i32) }),
                ]
            }),
            MockTableName::T1 => Box::new(|index| vec![
                Value::from((index as i32) / 10000),
                Value::from((index as i32) % 10000),
            ]),
            MockTableName::T41M => Box::new(|index| vec![
                Value::from((index as i32) % 500000),
                Value::from(((index as i32) % 500000) * 10),
            ]),
            MockTableName::T51M => Box::new(|index| vec![
                Value::from((index as i32 + 30000) % 500000),
                Value::from(((index as i32 + 30000) % 500000) * 10),
            ]),
            MockTableName::T61M => Box::new(|index| vec![
                Value::from((index as i32 + 60000) % 500000),
                Value::from(((index as i32 + 60000) % 500000) * 10),
            ]),
            MockTableName::T7 => Box::new(|index| vec![
                Value::from(index as i32 % 20),
                Value::from(index as i32),
                Value::from(index as i32),
            ]),
            MockTableName::T8 => Box::new(|index| vec![
                Value::from(index as i32),
            ]),
            MockTableName::T9 => Box::new(|index| vec![
                Value::from(index as i32 / 10000),
                Value::from((10000000 - (index / 2 + ((index / 10000) % 2) * ((index / 2) % 2))) as i32),
            ]),
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

