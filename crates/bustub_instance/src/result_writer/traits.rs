use crate::rows::Rows;

pub trait ResultWriter {
    fn write_cell(&mut self, cell: &str);
    fn write_header_cell(&mut self, cell: &str);
    fn begin_header(&mut self);
    fn end_header(&mut self);
    fn begin_row(&mut self);
    fn end_row(&mut self);
    fn begin_table(&mut self, simplified_output: bool);
    fn end_table(&mut self);
    fn one_cell(&mut self, cell: &str) {
        self.begin_table(true);
        self.begin_row();
        self.write_cell(cell);
        self.end_row();
        self.end_table();
    }
}
