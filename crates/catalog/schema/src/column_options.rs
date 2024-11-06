use data_types::Value;

#[derive(Clone, PartialEq, Debug)]
pub enum ColumnDefault {
    // No default
    None,

    // Specific value
    Value(Value),

    // TODO - can add in the future functions like now()
}

#[derive(Clone, PartialEq, Debug, derive_builder::Builder)]
#[builder(default, pattern = "mutable")]
pub struct ColumnOptions {
    nullable: bool,
    default: ColumnDefault,
}

impl ColumnOptions {
    pub fn builder() -> ColumnOptionsBuilder {
        ColumnOptionsBuilder::create_empty()
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn has_default(&self) -> bool {
        self.default == ColumnDefault::None
    }

    pub fn get_default(&self) -> &ColumnDefault {
        &self.default
    }
}

impl Default for ColumnOptions {
    fn default() -> Self {
        ColumnOptions {
            nullable: true,
            default: ColumnDefault::None,
        }
    }
}
