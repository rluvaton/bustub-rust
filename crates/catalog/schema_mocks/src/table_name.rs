use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;
use crate::constants::*;
use crate::MockDataIterator;

#[derive(EnumIter)]
pub enum MockTableName {
    Table1,
    Table2,
    Table3,
    TableTas2022,
    TableTas2023,
    TableTas2023Fall,
    AggInputSmall,
    AggInputBig,
    TableSchedule2022,
    TableSchedule2023,
    Table123,
    Graph,

    // For leaderboard Q1
    T1,

    // For leaderboard Q2
    T41M,
    T51M,
    T61M,

    // For leaderboard Q3
    T7,
    T8,
    T9,
}

impl MockTableName {
    pub fn get_table_list() -> Vec<&'static str> {
        MockTableName::iter()
            .map(|item| item.get_table_name())
            .collect()
    }

    pub fn get_table_name(&self) -> &'static str {
        match self {
            MockTableName::Table1 => "__mock_table_1",
            MockTableName::Table2 => "__mock_table_2",
            MockTableName::Table3 => "__mock_table_3",
            MockTableName::TableTas2022 => "__mock_table_tas_2022",
            MockTableName::TableTas2023 => "__mock_table_tas_2023",
            MockTableName::TableTas2023Fall => "__mock_table_tas_2023_fall",
            MockTableName::AggInputSmall => "__mock_agg_input_small",
            MockTableName::AggInputBig => "__mock_agg_input_big",
            MockTableName::TableSchedule2022 => "__mock_table_schedule_2022",
            MockTableName::TableSchedule2023 => "__mock_table_schedule_2023",
            MockTableName::Table123 => "__mock_table_123",
            MockTableName::Graph => "__mock_graph",
            MockTableName::T1 => "__mock_t1",
            MockTableName::T41M => "__mock_t4_1m",
            MockTableName::T51M => "__mock_t5_1m",
            MockTableName::T61M => "__mock_t6_1m",
            MockTableName::T7 => "__mock_t7",
            MockTableName::T8 => "__mock_t8",
            MockTableName::T9 => "__mock_t9",
        }
    }

    pub fn get_schema(&self) -> Schema {
        let columns: Vec<Column> = match self {
            MockTableName::Table1 => vec![
                Column::new_fixed_size("colA".to_string(), DBTypeId::INT),
                Column::new_fixed_size("colB".to_string(), DBTypeId::INT),
            ],
            MockTableName::Table2 => vec![
                Column::new_variable_size("colC".to_string(), DBTypeId::VARCHAR, 128),
                Column::new_variable_size("colD".to_string(), DBTypeId::VARCHAR, 128),
            ],
            MockTableName::Table3 => vec![
                Column::new_fixed_size("colE".to_string(), DBTypeId::INT),
                Column::new_variable_size("colF".to_string(), DBTypeId::VARCHAR, 128),
            ],
            MockTableName::TableTas2022 | MockTableName::TableTas2023 | MockTableName::TableTas2023Fall => vec![
                Column::new_variable_size("github_id".to_string(), DBTypeId::VARCHAR, 128),
                Column::new_variable_size("office_hour".to_string(), DBTypeId::VARCHAR, 128),
            ],
            MockTableName::TableSchedule2022 | MockTableName::TableSchedule2023 => vec![
                Column::new_variable_size("day_of_week".to_string(), DBTypeId::VARCHAR, 128),
                Column::new_fixed_size("has_lecture".to_string(), DBTypeId::INT),
            ],
            MockTableName::AggInputSmall | MockTableName::AggInputBig => vec![
                Column::new_fixed_size("v1".to_string(), DBTypeId::INT),
                Column::new_fixed_size("v2".to_string(), DBTypeId::INT),
                Column::new_fixed_size("v3".to_string(), DBTypeId::INT),
                Column::new_fixed_size("v4".to_string(), DBTypeId::INT),
                Column::new_fixed_size("v5".to_string(), DBTypeId::INT),
                Column::new_variable_size("v6".to_string(), DBTypeId::VARCHAR, 128),
            ],
            MockTableName::Graph => vec![
                Column::new_fixed_size("src".to_string(), DBTypeId::INT),
                Column::new_fixed_size("dst".to_string(), DBTypeId::INT),
                Column::new_variable_size("src_label".to_string(), DBTypeId::VARCHAR, 8),
                Column::new_variable_size("dst_label".to_string(), DBTypeId::VARCHAR, 8),
                Column::new_fixed_size("distance".to_string(), DBTypeId::INT),
            ],
            MockTableName::Table123 => vec![
                Column::new_fixed_size("number".to_string(), DBTypeId::INT),
            ],
            MockTableName::T41M | MockTableName::T51M | MockTableName::T61M => vec![
                Column::new_fixed_size("x".to_string(), DBTypeId::INT),
                Column::new_fixed_size("y".to_string(), DBTypeId::INT),
            ],
            MockTableName::T1 => vec![
                Column::new_fixed_size("x".to_string(), DBTypeId::INT),
                Column::new_fixed_size("y".to_string(), DBTypeId::INT),
                Column::new_fixed_size("z".to_string(), DBTypeId::INT),
            ],
            MockTableName::T7 => vec![
                Column::new_fixed_size("v".to_string(), DBTypeId::INT),
                Column::new_fixed_size("v1".to_string(), DBTypeId::INT),
                Column::new_fixed_size("v2".to_string(), DBTypeId::INT),
            ],
            MockTableName::T8 => vec![
                Column::new_fixed_size("v4".to_string(), DBTypeId::INT),
            ],
            MockTableName::T9 => vec![
                Column::new_fixed_size("x".to_string(), DBTypeId::INT),
                Column::new_fixed_size("y".to_string(), DBTypeId::INT),
            ],
        };

        Schema::new(columns)
    }

    pub fn get_size_of(&self) -> usize {
        match self {
            MockTableName::Table1 | MockTableName::Table2 | MockTableName::Table3 => 100,
            MockTableName::TableTas2022 => TA_LIST_2022.len(),
            MockTableName::TableTas2023 => TA_LIST_2023.len(),
            MockTableName::TableTas2023Fall => TA_LIST_2023_FALL.len(),
            MockTableName::AggInputSmall => 1000,
            MockTableName::AggInputBig => 10000,
            MockTableName::TableSchedule2022 | MockTableName::TableSchedule2023 => COURSE_ON_DATE.len(),
            MockTableName::Table123 => 3,
            MockTableName::Graph => GRAPH_NODE_CNT * GRAPH_NODE_CNT,
            MockTableName::T1 | MockTableName::T41M | MockTableName::T51M | MockTableName::T61M | MockTableName::T7 => 1000000,
            MockTableName::T8 => 10,
            MockTableName::T9 => 10000000,
            _ => 0
        }
    }

    pub fn is_shuffled(&self) -> bool {
        match self {
            MockTableName::T1 => true,
            _ => false,
        }
    }

    pub fn create_iter() -> MockTableNameIter {
        MockTableName::iter()
    }

    pub fn get_data_iter(self) -> MockDataIterator {
        MockDataIterator::from(self)
    }
}

impl Display for MockTableName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_table_name())
    }
}

impl TryFrom<String> for MockTableName {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl<'a> TryFrom<&'a str> for MockTableName {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        MockTableName::iter()
            .find(|table_name| table_name.get_table_name() == value)
            .ok_or(())
    }
}
