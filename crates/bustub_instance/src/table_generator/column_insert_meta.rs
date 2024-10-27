use data_types::{BigIntUnderlyingType, DBTypeId, DecimalUnderlyingType, IntUnderlyingType, SmallIntUnderlyingType, TinyIntUnderlyingType};
use crate::table_generator::dist::Dist;

#[derive(Copy, Clone)]
pub struct GenerateMeta<T> {
    /// Min value of the column
    pub(super) min: T,

    /// Max value of the column
    pub(super) max: T,
}

#[derive(Copy, Clone)]
pub enum GenerateType {
    TinyInt(GenerateMeta<TinyIntUnderlyingType>),
    SmallInt(GenerateMeta<SmallIntUnderlyingType>),
    Int(GenerateMeta<IntUnderlyingType>),
    BigInt(GenerateMeta<BigIntUnderlyingType>),
    Decimal(GenerateMeta<DecimalUnderlyingType>),
}

impl From<GenerateType> for DBTypeId {
    fn from(value: GenerateType) -> Self {
        match value {
            GenerateType::TinyInt(_) => DBTypeId::TINYINT,
            GenerateType::SmallInt(_) => DBTypeId::SMALLINT,
            GenerateType::Int(_) => DBTypeId::INT,
            GenerateType::BigInt(_) => DBTypeId::BIGINT,
            GenerateType::Decimal(_) => DBTypeId::DECIMAL,
        }
    }
}

/// Metadata about the data for a given column. Specifically, the type of the
/// column, the distribution of values, a min and max if appropriate.
#[derive(Clone)]
pub(super) struct ColumnInsertMeta {
    /// Name of the column
    pub(super) name: String,
    
    /// Whether the column is nullable
    pub(super) nullable: bool,
    
    /// Distribution of values
    pub(super) dist: Dist,

    // Type of the column with data
    pub(super) generate_type: GenerateType,
    
    /// Counter to generate serial data
    pub(super) serial_counter: i64,
}
