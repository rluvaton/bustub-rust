use sqlparser::ast::{CharLengthUnits, CharacterLength, ColumnOption, DataType};
use catalog_schema::Column;
use common::config::VARCHAR_DEFAULT_LENGTH;
use data_types::DBTypeId;
use crate::try_from_ast_error::{ParseASTResult, ParseASTError};

pub(crate) trait ColumnDefExt {
    fn try_convert_into_column(&self) -> ParseASTResult<Column>;

    fn try_is_primary_column(&self) -> ParseASTResult<bool>;
}

// ColumnDef
impl ColumnDefExt for sqlparser::ast::ColumnDef {
    fn try_convert_into_column(&self) -> ParseASTResult<Column> {
        let name = &self.name.value;


        let fixed_size_data_type = match self.data_type {
            DataType::CharacterVarying(size) | DataType::CharVarying(size) | DataType::Varchar(size) => {
               let size = size.unwrap_or(CharacterLength::IntegerLength {
                    length: VARCHAR_DEFAULT_LENGTH as u64,
                    unit: Some(CharLengthUnits::Characters)
                });

                let length = match size {
                    CharacterLength::IntegerLength { length, unit } => {
                        match unit.unwrap_or(CharLengthUnits::Characters) {
                            CharLengthUnits::Characters => {}
                            CharLengthUnits::Octets => {
                                return Err(ParseASTError::Unimplemented("Unsupported unit of char size".to_string()))
                            }
                        }

                        length
                    }
                    CharacterLength::Max => return Err(ParseASTError::Unimplemented("Should specify max length".to_string()))
                };

                return Ok(Column::new_variable_size(name.clone(), DBTypeId::VARCHAR, length as u32));
            }
            DataType::TinyInt(unsupported) => {
                if unsupported.is_some() {
                    return Err(ParseASTError::Unimplemented(format!("Unsupported size for {}", self.data_type)))
                }

                DBTypeId::TINYINT
            }
            DataType::Int2(unsupported) | DataType::SmallInt(unsupported) => {
                if unsupported.is_some() {
                    return Err(ParseASTError::Unimplemented(format!("Unsupported size for {}", self.data_type)))
                }

                DBTypeId::SMALLINT
            }
            DataType::Int(unsupported) | DataType::Int4(unsupported) | DataType::Integer(unsupported) => {
                if unsupported.is_some() {
                    return Err(ParseASTError::Unimplemented(format!("Unsupported size for {}", self.data_type)))
                }

                DBTypeId::INT
            }
            DataType::Int8(unsupported) | DataType::BigInt(unsupported)=> {
                if unsupported.is_some() {
                    return Err(ParseASTError::Unimplemented(format!("Unsupported size for {}", self.data_type)))
                }

                DBTypeId::BIGINT
            }
            DataType::Float8 | DataType::Double | DataType::DoublePrecision => DBTypeId::DECIMAL,
            DataType::Bool | DataType::Boolean => DBTypeId::BOOLEAN,
            _ => return Err(ParseASTError::Unimplemented(format!("datatype {} is not supported", self.data_type)))
        };

        Ok(Column::new_fixed_size(name.clone(), fixed_size_data_type))
    }

    fn try_is_primary_column(&self) -> ParseASTResult<bool> {
        let mut is_col_primary = false;
        for option in &self.options {
            match option.option {
                // TODO - is supported?
                // ColumnOption::Null => {}
                // ColumnOption::NotNull => {}
                ColumnOption::Unique { is_primary, .. } => {
                    if !is_primary {
                        return Err(ParseASTError::Unimplemented("unique columns that are not primary key are not supported".to_string()))
                    }

                    is_col_primary = is_primary;
                },
                _ => return Err(ParseASTError::Unimplemented(format!("Option {} is not supported ", option.option))),
            }
        }

        Ok(is_col_primary)
    }
}
