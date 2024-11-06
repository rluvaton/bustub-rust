use std::fmt::{Debug, Display, Formatter};
use data_types::DBTypeId;
use crate::column_options::ColumnOptions;

// TODO - implement src/include/catalog/column.h
#[derive(Clone, PartialEq)]
pub struct Column {
    /// Column name.
    column_name: String,

    /// Column value's type.
    column_type: DBTypeId,

    /// For a non-inlined column, this is the size of a pointer. Otherwise, the size of the fixed length column.
    fixed_length: u32,

    /// For an inlined column, 0. Otherwise, the length of the variable length column.
    variable_length: u32,

    /// Column offset in the tuple.
    pub(super) column_offset: u32,

    options: ColumnOptions
}

impl Column {
    /// Create non-variable-length column
    ///
    /// # Arguments
    ///
    /// * `column_name`: name of the column
    /// * `type_id`: type of the column
    ///
    /// returns: Column
    pub fn new_fixed_size(column_name: String, type_id: DBTypeId) -> Self {
        assert_ne!(type_id, DBTypeId::VARCHAR, "Wrong function for VARCHAR type.");

        Self {
            column_name,
            column_type: type_id,
            fixed_length: type_id.get_size() as u32,
            variable_length: 0,
            column_offset: 0,
            options: ColumnOptions::default()
        }
    }

    /// Create variable-length column.
    ///
    /// # Arguments
    ///
    /// * `column_name`: name of the column
    /// * `type_id`: type of the column
    ///
    /// returns: Column
    pub fn new_variable_size(column_name: String, type_id: DBTypeId, length: u32) -> Self {
        assert_eq!(type_id, DBTypeId::VARCHAR, "Wrong function for non-VARCHAR type.");

        Self {
            column_name,
            column_type: type_id,
            fixed_length: type_id.get_size() as u32,
            variable_length: length,
            column_offset: 0,
            options: ColumnOptions::default(),
        }
    }

    /// Replicate a Column with a different name.
    ///
    /// # Arguments
    ///
    /// * `column_name`: name of the column
    /// * `column`: the original column
    ///
    /// returns: Column
    pub fn create_new_name(column_name: String, column: &Self) -> Self {
        Self {
            column_name,
            column_type: column.column_type,
            fixed_length: column.fixed_length,
            variable_length: column.variable_length,
            column_offset: column.column_offset,
            options: column.options.clone(),
        }
    }

    pub fn with_options(mut self, options: ColumnOptions) -> Self {
        self.options = options;

        self
    }

    /// get column name
    pub fn get_name(&self) -> &String {
        &self.column_name
    }

    /// get column length
    pub fn get_length(&self) -> u32 {
        if self.is_inlined() { self.fixed_length } else { self.variable_length }
    }

    /// get fixed length
    pub fn get_fixed_length(&self) -> u32 {
        self.fixed_length
    }

    /// get variable length
    pub fn get_variable_length(&self) -> u32 {
        self.variable_length
    }

    /// get column's offset in the tuple
    pub fn get_offset(&self) -> usize {
        self.column_offset as usize
    }

    /// get column type
    pub fn get_type(&self) -> DBTypeId {
        self.column_type
    }

    /// true if column is inlined, false otherwise
    pub fn is_inlined(&self) -> bool {
        !matches!(self.column_type, DBTypeId::VARCHAR)
    }

    pub fn get_column_options(&self) -> &ColumnOptions {
        &self.options
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.column_name, self.column_type)
    }
}


impl Debug for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Column[{}, {}, Offset: {}, ", self.column_name, self.column_type, self.column_offset)?;

        if self.is_inlined() {
            write!(f, "FixedLength: {}", self.fixed_length)?;
        } else {
            write!(f, "VarLength: {}", self.variable_length)?;
        }

        write!(f, "]")
    }
}

