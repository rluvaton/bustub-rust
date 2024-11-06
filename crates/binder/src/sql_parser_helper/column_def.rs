use sqlparser::ast::{CharLengthUnits, CharacterLength, ColumnOption, DataType};
use catalog_schema::{Column, ColumnDefault, ColumnOptions, ColumnOptionsBuilder};
use common::config::VARCHAR_DEFAULT_LENGTH;
use data_types::DBTypeId;
use crate::{Binder, Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTResult, ParseASTError};

pub(crate) trait ColumnDefExt {
    fn try_convert_into_column(&self, binder: &Binder) -> ParseASTResult<Column>;

    fn try_parse_options(&self, binder: &Binder) -> ParseASTResult<ColumnOptionsBuilder>;

    fn is_primary_column(&self) -> bool;
}

// ColumnDef
impl ColumnDefExt for sqlparser::ast::ColumnDef {
    fn try_convert_into_column(&self, binder: &Binder) -> ParseASTResult<Column> {
        let mut column_options_builder = self.try_parse_options(binder)?;
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

                column_options_builder.db_type(DBTypeId::VARCHAR);

                return Ok(
                    Column::new_variable_size(name.clone(), DBTypeId::VARCHAR, length as u32).with_options(column_options_builder.build().unwrap())
                );
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

        Ok(Column::new_fixed_size(name.clone(), fixed_size_data_type).with_options(column_options_builder.db_type(fixed_size_data_type).build().unwrap()))
    }

    fn try_parse_options(&self, binder: &Binder) -> ParseASTResult<ColumnOptionsBuilder> {
        let mut column_options_builder = ColumnOptions::builder();

        // Nullable by default
        column_options_builder.nullable(true);

        for option in &self.options {
            match &option.option {
                ColumnOption::Null => {
                    column_options_builder.nullable(true);
                }
                ColumnOption::NotNull => {
                    column_options_builder.nullable(false);
                }
                ColumnOption::Default(expr) => {
                    let default_value = ExpressionTypeImpl::try_parse_from_expr(&expr, binder)?;

                    let default = match default_value {
                        ExpressionTypeImpl::Constant(c) => ColumnDefault::Value(c.value),
                        _ => return Err(ParseASTError::Unimplemented(format!("Default value {:?} is not supported ", default_value))),
                    };

                    column_options_builder.default(default);
                }
                ColumnOption::Unique { is_primary, .. } => {
                    if !is_primary {
                        return Err(ParseASTError::Unimplemented("unique columns that are not primary key are not supported".to_string()))
                    }
                }
                _ => return Err(ParseASTError::Unimplemented(format!("Option {} is not supported ", option.option))),
            }
        }

        Ok(column_options_builder)
    }

    fn is_primary_column(&self) -> bool {
        let mut is_col_primary = false;

        for option in &self.options {
            match option.option {
                ColumnOption::Unique { is_primary, .. } => {
                    is_col_primary = is_primary;
                },
                _ => {},
            }
        }

       is_col_primary
    }
}
