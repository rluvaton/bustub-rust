use catalog_schema::Schema;
use crate::result_writer::ResultWriter;
use crate::rows::Rows;

pub struct SQLTestWriter {
    pub rows: Option<Rows>,
}

impl Default for SQLTestWriter {
    fn default() -> Self {
        Self { rows: None }
    }
}

impl ResultWriter for SQLTestWriter {
    fn write_cell(&mut self, cell: &str) {
    }

    fn write_header_cell(&mut self, _cell: &str) {
    }

    fn begin_header(&mut self) {
    }

    fn end_header(&mut self) {
    }

    fn begin_row(&mut self) {
    }

    fn end_row(&mut self) {
    }

    fn begin_table(&mut self, _simplified_output: bool) {
    }

    fn end_table(&mut self) {

    }

    fn save_rows(&mut self, rows: Rows) {
        self.rows.replace(rows);
    }
}
