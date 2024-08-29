pub trait Indentation: Sized {
    fn indent_spaces(&self, indent: usize) -> Self;
    fn indent_tabs(&self, indent: usize) -> Self;

    fn indent(&self, indent: usize) -> Self {
        self.indent_spaces(indent)
    }
}

impl Indentation for String {
    fn indent_spaces(&self, indent: usize) -> Self {
        " ".repeat(indent) + self
    }

    fn indent_tabs(&self, indent: usize) -> Self {
        "\t".repeat(indent) + self
    }
}

// impl Indentation for &str {
//     fn indent_spaces(&self, indent: usize) -> Self {
//         self.to_string().indent_spaces(indent).as_str()
//     }
//
//     fn indent_tabs(&self, indent: usize) -> Self {
//         self.to_string().indent_tabs(indent).as_str()
//     }
// }
