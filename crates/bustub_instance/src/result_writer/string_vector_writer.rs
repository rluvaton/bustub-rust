use crate::result_writer::ResultWriter;

pub struct StringVectorWriter(Vec<Vec<String>>);

impl Default for StringVectorWriter {
    fn default() -> Self {
        Self(vec![])
    }
}

impl ResultWriter for StringVectorWriter {
    fn write_cell(&mut self, cell: &str) {
        self.0.last_mut().expect("Must have row").push(cell.to_string());
    }

    fn write_header_cell(&mut self, _cell: &str) {
    }

    fn begin_header(&mut self) {
    }

    fn end_header(&mut self) {
    }

    fn begin_row(&mut self) {
        self.0.push(vec![]);
    }

    fn end_row(&mut self) {
    }

    fn begin_table(&mut self, _simplified_output: bool) {
        self.0.clear();
    }

    fn end_table(&mut self) {
    }
}
