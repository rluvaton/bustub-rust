use crate::DBTypeId;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum InnerNumericConversionError {

    // The values are as string to avoid having generics
    #[error("value {value} is out of range of {min} and {max}")]
    OutOfRange {
        value: String,
        min: String,
        max: String
    },
}

pub type NumericConversionError = error_utils::Error<InnerNumericConversionError>;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum InnerFromStringConversionError {
    #[error("value {value} cannot be converted to {dest_type}")]
    UnableToConvert {
        value: String,
        dest_type: DBTypeId,
    },
}

pub type FromStringConversionError = error_utils::Error<InnerFromStringConversionError>;
