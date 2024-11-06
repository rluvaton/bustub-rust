use data_types::{DBTypeId, Value};

#[derive(Clone, PartialEq, Debug)]
pub enum ColumnDefault {
    // No default
    None,

    // Specific value
    Value(Value),

    // TODO - can add in the future functions like now()
}

impl Default for ColumnDefault {
    fn default() -> Self {
        ColumnDefault::None
    }
}

#[derive(Clone, PartialEq, Debug, derive_builder::Builder)]
#[builder(pattern = "mutable", build_fn(skip))]
pub struct ColumnOptions {
    db_type: DBTypeId,
    nullable: bool,
    default: ColumnDefault,
}


impl ColumnOptionsBuilder {
    pub fn build(&mut self) -> Result<ColumnOptions, String> {
        if self.db_type.is_none() {
            return Err("ColumnOptions must have db type".to_string())
        }

        let db_type = self.db_type.unwrap();
        let nullable = self.nullable.unwrap_or(true);
        let mut default = self.default.clone().unwrap_or_default();

        if nullable && default == ColumnDefault::None {
            default = ColumnDefault::Value(Value::null(db_type));
        }

        // Validate default
        match default {
            ColumnDefault::None => {}
            ColumnDefault::Value(ref d) => {
                if d.get_db_type_id() != db_type {
                    return Err(format!("default value type mismatch, expected {db_type}, got {}", d.get_db_type_id()))
                }
            }
        }

        Ok(ColumnOptions {
            db_type,
            nullable,
            default,
        })
    }
}

impl ColumnOptions {
    pub fn builder() -> ColumnOptionsBuilder {
        ColumnOptionsBuilder::create_empty()
    }

    pub fn default_for_type(db_type: DBTypeId) -> Self {
        Self::builder().db_type(db_type).build().expect("Must be able to create default column options")
    }

    pub fn get_db_type(&self) -> DBTypeId {
        self.db_type
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
