use comfy_table::{Table, TableComponent};
use crate::result_writer::ResultWriter;

pub struct ComfyTableWriter {
    current_row: Option<Vec<String>>,
    current_header: Option<Vec<String>>,
    table: Table,
    tables: Vec<String>,
}

impl ComfyTableWriter {
    pub fn get_tables(&self) -> &Vec<String> {
        &self.tables
    }
}

impl Default for ComfyTableWriter {
    fn default() -> Self {
        ComfyTableWriter {
            current_row: None,
            current_header: None,
            table: Table::new(),
            tables: vec![],
        }
    }
}

impl ResultWriter for ComfyTableWriter {
    fn write_cell(&mut self, cell: &str) {
        self.current_row.as_mut().expect("Current row must be initialized").push(cell.to_string());
    }

    fn write_header_cell(&mut self, cell: &str) {
        self.current_header.as_mut().expect("Current header must be initialized").push(cell.to_string());
    }

    fn begin_header(&mut self) {
        assert_eq!(self.current_row, None, "Must not be in a middle of a row");
        assert_eq!(self.current_header, None, "Must not be in the middle of a header");

        self.current_header = Some(vec![]);
    }

    fn end_header(&mut self) {
        let current_header = self.current_header.take().expect("Must have header initialized before calling end header");
        self.table.set_header(current_header);
    }

    fn begin_row(&mut self) {
        assert_eq!(self.current_row, None, "Must not be in a middle of a row");
        assert_eq!(self.current_header, None, "Must not be in the middle of a header");

        self.current_row = Some(vec![]);
    }

    fn end_row(&mut self) {
        let current_row = self.current_row.take().expect("Must have row initialized before calling end row");
        self.table.add_row(current_row);
    }

    fn begin_table(&mut self, simplified_output: bool) {
        if simplified_output {
            // table_.set_border_style(FT_EMPTY_STYLE);

            // Remove the borders
            self.table.remove_style(TableComponent::HorizontalLines);
            self.table.remove_style(TableComponent::VerticalLines);
        }
    }

    fn end_table(&mut self) {
        self.tables.push(self.table.to_string());
        self.table = Table::new();
    }

    fn one_cell(&mut self, cell: &str) {
        self.tables.push(cell.to_string() + "\n");
    }


}
