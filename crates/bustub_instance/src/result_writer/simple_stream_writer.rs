use std::io;
use std::io::Write;
use crate::result_writer::ResultWriter;

pub struct SimpleStreamWriter {
    disable_header: bool,
    stream: Box<dyn Write>,
    separator: String,
}

impl SimpleStreamWriter {
    pub fn new(stream: Box<dyn Write>, disable_header: Option<bool>, separator: Option<String>) -> Self {
        Self {
            disable_header: disable_header.unwrap_or(false),
            separator: separator.unwrap_or("\t".to_string()),
            stream
        }
    }

    pub fn bold_on(stream: &mut Box<dyn Write>) -> io::Result<usize> {
        stream.write("\x1B[1m".as_bytes())
    }

    pub fn bold_off(stream: &mut Box<dyn Write>) -> io::Result<usize> {
        stream.write("\x1B[0m".as_bytes())
    }
}

impl ResultWriter for SimpleStreamWriter {
    fn write_cell(&mut self, cell: &str) {
        self.stream.write(cell.as_bytes()).unwrap();
        self.stream.write(self.separator.as_bytes()).unwrap();
    }

    fn write_header_cell(&mut self, cell: &str) {
        if !self.disable_header {
            Self::bold_on(&mut self.stream).unwrap();
            self.stream.write(cell.as_bytes()).unwrap();
            Self::bold_off(&mut self.stream).unwrap();
        }
    }

    fn begin_header(&mut self) {
    }

    fn end_header(&mut self) {
        if !self.disable_header {
            self.stream.write("\n".as_bytes()).unwrap();
        }
    }

    fn begin_row(&mut self) {
    }

    fn end_row(&mut self) {
        self.stream.write("\n".as_bytes()).unwrap();
    }

    fn begin_table(&mut self, _simplified_output: bool) {
    }

    fn end_table(&mut self) {
    }
}
