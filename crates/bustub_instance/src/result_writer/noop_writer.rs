use super::ResultWriter;

pub struct NoopWriter;

impl ResultWriter for NoopWriter {
    fn write_cell(&mut self, _cell: &str) {
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
}
