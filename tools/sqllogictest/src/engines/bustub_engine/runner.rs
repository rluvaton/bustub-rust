use std::sync::Arc;
use std::{path::PathBuf, time::Duration};

use async_trait::async_trait;
use log::info;
use sqllogictest::DBOutput;
use bustub_instance::{BustubInstance, SqlDBOutput};
use execution_common::CheckOptions;
use crate::engines::bustub_engine::normalize::cell_to_string;
use super::{error::Result, normalize, BustubSqlLogicTestError};

use crate::engines::output::{BustubColumnType, BustubOutput};

pub struct Bustub {
    ctx: BustubInstance,
    relative_path: PathBuf,
}

impl Bustub {
    pub fn new(ctx: BustubInstance, relative_path: PathBuf) -> Self {
        Self { ctx, relative_path }
    }
}

#[async_trait]
impl sqllogictest::AsyncDB for Bustub {
    type Error = BustubSqlLogicTestError;
    type ColumnType = BustubColumnType;

    async fn run(&mut self, sql: &str) -> Result<BustubOutput> {
        info!(
            "[{}] Running query: \"{}\"",
            self.relative_path.display(),
            sql
        );
        run_query(&self.ctx, sql).await
    }

    /// Engine name of current database.
    fn engine_name(&self) -> &str {
        "Bustub"
    }

    /// [`Bustub`] calls this function to perform sleep.
    ///
    /// The default implementation is `std::thread::sleep`, which is universal to any async runtime
    /// but would block the current thread. If you are running in tokio runtime, you should override
    /// this by `tokio::time::sleep`.
    async fn sleep(dur: Duration) {
        tokio::time::sleep(dur).await;
    }
}

async fn run_query(ctx: &BustubInstance, sql: impl Into<String>) -> Result<BustubOutput> {
    let results = ctx.execute_user_input(sql.into().as_str(), CheckOptions::default())?;

    let results = match results {
        bustub_instance::DBOutput::SQL(res) => res,

        // Should not be used
        bustub_instance::DBOutput::System(_) => unreachable!("System commands should not be used i the sql logic tests")
    };

    assert_eq!(results.len(), 1, "Results must only have single result (1 query was ran)");

    let result = &results[0];

    match result {
        SqlDBOutput::Rows(rows) => {
            let res = DBOutput::Rows {
                types: rows
                    .get_schema()
                    .get_columns()
                    .iter()
                    .map(|col| col.get_type().into())
                    .collect(),
                rows: rows
                    .get_rows()
                    .iter()
                    .map(|row| row.iter()
                        .map(|cell| cell_to_string(cell))
                        .collect::<Vec<_>>()
                    )
                    .collect::<Vec<_>>(),
            };

            Ok(res)
        }
        SqlDBOutput::Statement(statement) => {
            Ok(DBOutput::StatementComplete(statement.get_affected()))
        }
    }
}
