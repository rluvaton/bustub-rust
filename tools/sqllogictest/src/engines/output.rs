// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use sqllogictest::{ColumnType, DBOutput};
use data_types::DBTypeId;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BustubColumnType(DBTypeId);

impl Default for BustubColumnType {
    fn default() -> Self {
        BustubColumnType(DBTypeId::INVALID)
    }
}

impl From<DBTypeId> for BustubColumnType {
    fn from(value: DBTypeId) -> Self {
        Self(value)
    }
}

impl ColumnType for BustubColumnType {
    fn from_char(value: char) -> Option<Self> {
        match value {
            'B' => Some(DBTypeId::BOOLEAN.into()),
            'I' => Some(DBTypeId::INT.into()),
            'L' => Some(DBTypeId::BIGINT.into()),
            'P' => Some(DBTypeId::TIMESTAMP.into()),
            'R' => Some(DBTypeId::DECIMAL.into()),
            'T' => Some(DBTypeId::VARCHAR.into()),
            _ => Some(BustubColumnType::default()),
        }
    }

    fn to_char(&self) -> char {
        match self.0 {
            DBTypeId::BOOLEAN => 'B',
            DBTypeId::INT => 'I',
            DBTypeId::BIGINT => 'L',
            DBTypeId::TIMESTAMP => 'P',
            DBTypeId::DECIMAL => 'R',
            DBTypeId::VARCHAR => 'T',
            _ => '?'
        }
    }
}


pub(crate) type BustubOutput = DBOutput<BustubColumnType>;
