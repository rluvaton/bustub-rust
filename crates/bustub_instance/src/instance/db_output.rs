use crate::result_writer::ResultWriter;
use crate::rows::Rows;

// The output of `\` command
#[derive(Debug, Clone)]
pub struct SystemOutput {
    header: Vec<String>,
    results: Vec<Vec<String>>,
    simplified_output: bool,
}

impl From<SystemOutput> for DBOutput {
    fn from(value: SystemOutput) -> Self {
        DBOutput::System(value)
    }
}

impl SystemOutput {
    pub fn new(header: Vec<String>,
               results: Vec<Vec<String>>,
               simplified_output: bool) -> Self {
        Self {
            header,
            results,
            simplified_output,
        }
    }

    pub fn single_cell(cell: String) -> Self {
        Self {
            header: vec![],
            results: vec![vec![cell]],
            simplified_output: true,
        }
    }

    fn is_single_cell(&self) -> bool {
        self.header.is_empty() && self.results.len() == 1 && self.results[0].len() == 1
    }

    pub fn write_results<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        if self.is_single_cell() {
            writer.write_cell(self.results[0][0].as_str());
            return;
        }
        writer.begin_table(self.simplified_output);

        if !self.header.is_empty() {
            writer.begin_header();
            self.header
                .iter()
                .for_each(|header| writer.write_header_cell(header.as_str()));
            writer.end_header();
        }

        self.results
            .iter()
            .for_each(|row| {
                writer.begin_row();

                row.iter().for_each(|cell| writer.write_cell(cell.as_str()));

                writer.end_row();
            });

        writer.end_table();
    }
}

/// A statement in the query has completed.
///
/// The number of rows modified or deleted or tables modified.
/// with additional info message
#[derive(Debug, Clone)]
pub struct StatementOutput {
    affected: u64,
    info: Option<String>,
}

impl StatementOutput {

    pub fn new_affected(affected: u64) -> Self {
        Self {
            affected,
            info: None
        }
    }

    pub fn new_with_info(affected: u64, info: String) -> Self {
        Self {
            affected,
            info: Some(info)
        }
    }

    pub fn new(affected: u64, info: Option<String>) -> Self {
        Self {
            affected,
            info
        }
    }

    pub fn write_results<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        if let Some(info) = &self.info {
            writer.one_cell(info.as_str());
        } else {
            writer.one_cell(self.affected.to_string().as_str());
        }
    }
}


impl From<StatementOutput> for SqlDBOutput {
    fn from(value: StatementOutput) -> Self {
        SqlDBOutput::Statement(value)
    }
}

impl From<Rows> for SqlDBOutput {
    fn from(value: Rows) -> Self {
        SqlDBOutput::Rows(value)
    }
}

#[derive(Debug, Clone)]
pub enum SqlDBOutput {

    // The rows output
    Rows(Rows),

    /// A statement in the query has completed.
    ///
    /// The number of rows modified or deleted or tables modified.
    /// with additional info message
    Statement(StatementOutput),
}

impl SqlDBOutput {
    pub fn write_results<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        match self {
            SqlDBOutput::Rows(rows) => rows.write_results(writer),
            SqlDBOutput::Statement(output) => output.write_results(writer),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultipleCommandsOutput(Vec<SqlDBOutput>);


impl MultipleCommandsOutput {

    pub fn new(outputs: Vec<SqlDBOutput>) -> Self {
        Self(outputs)
    }

    pub fn write_results<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        self.0.iter().for_each(|res| res.write_results(writer));
    }
}



impl From<MultipleCommandsOutput> for DBOutput {
    fn from(value: MultipleCommandsOutput) -> Self {
        DBOutput::SQL(value)
    }
}



#[derive(Debug, Clone)]
pub enum DBOutput {
    SQL(MultipleCommandsOutput),

    // The output of `\` command
    System(SystemOutput),
}

impl DBOutput {
    pub fn write_results<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        match self {
            DBOutput::SQL(rows) => rows.write_results(writer),
            DBOutput::System(output) => output.write_results(writer)
        }
    }
}
