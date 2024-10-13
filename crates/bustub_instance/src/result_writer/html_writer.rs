use std::io;
use std::io::Write;
use crate::result_writer::ResultWriter;

pub struct HtmlWriter {
    string: String,
    simplified_output: bool,
}

impl Default for HtmlWriter {
    fn default() -> Self {
        HtmlWriter {
            string: "".to_string(),
            simplified_output: false
        }
    }
}

impl HtmlWriter {
    fn escape(data: &str) -> String {
        let mut buffer = String::with_capacity(data.len());

        for ch in data.chars() {
            let mapped: String = match ch {
                '&' => "&amp;".to_string(),
                '"' => "&quot;".to_string(),
                '\'' => "&apos;".to_string(),
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                _ => ch.to_string()
            };

            buffer.push_str(mapped.as_str());
        }

        buffer
    }
}

impl ResultWriter for HtmlWriter {
    fn write_cell(&mut self, cell: &str) {
        print!("{}", cell);
        if !self.simplified_output {
            self.string.push_str("<td>");
            self.string.push_str(Self::escape(cell).as_str());
            self.string.push_str("</td>");
        } else {
            self.string.push_str(Self::escape(cell).as_str());
        }
    }

    fn write_header_cell(&mut self, cell: &str) {
        if !self.simplified_output {
            self.string.push_str("<td>");
            self.string.push_str(Self::escape(cell).as_str());
            self.string.push_str("</td>");
        } else {
            self.string.push_str(Self::escape(cell).as_str());
        }
    }

    fn begin_header(&mut self) {
        if !self.simplified_output {
            self.string.push_str("<thead><tr>");
        }
    }

    fn end_header(&mut self) {
        if !self.simplified_output {
            self.string.push_str("</tr></thead>");
        }
    }

    fn begin_row(&mut self) {
        if !self.simplified_output {
            self.string.push_str("<tr>");
        }
    }

    fn end_row(&mut self) {
        if !self.simplified_output {
            self.string.push_str("</tr>");
        }
    }

    fn begin_table(&mut self, simplified_output: bool) {
        self.simplified_output = simplified_output;

        if !self.simplified_output {
            self.string.push_str("<table>");
        } else {
            self.string.push_str("<div>");
        }
    }

    fn end_table(&mut self) {
        if !self.simplified_output {
            self.string.push_str("</table>");
        } else {
            self.string.push_str("</div>");
        }
    }
}
