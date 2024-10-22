use rand::rngs::ThreadRng;
use crate::table_generator::column_insert_meta::{GenerateMeta, GenerateType};
use crate::table_generator::dist::Dist;
use rand::distributions::{Uniform, Distribution};
use data_types::{BigIntUnderlyingType, DecimalUnderlyingType, IntUnderlyingType, SmallIntUnderlyingType, TinyIntUnderlyingType, Value};

pub(crate) trait GenerateValues {
    fn gen_numeric_values(&self, dist: Dist, serial_counter: &mut i64, count: usize, rng: &mut ThreadRng) -> Vec<Value>;
}

macro_rules! generate_values_impl {
    ($($t:ty)+) => ($(
impl GenerateValues for GenerateMeta<$t> {
    fn gen_numeric_values(&self, dist: Dist, serial_counter: &mut i64, count: usize, rng: &mut ThreadRng) -> Vec<Value> {
        let count = count as i64;
        match dist {
            Dist::Uniform => {
                let uniform = Uniform::from(self.min..self.max);

                (0..count)
                    .map(|_| uniform.sample(rng))
                    .map(|value| Value::from(value))
                    .collect::<Vec<Value>>()
            }
            Dist::Serial => {
                let values = (0..count)
                    .map(|i| (i + *serial_counter) + self.min as i64)
                    .map(|value| Value::from(value as $t))
                    .collect::<Vec<Value>>();

                *serial_counter += count;

                values
            }
            Dist::Cyclic => {
                let values = (0..count)
                    .map(|i| ((i + *serial_counter) % self.max as i64))
                    .map(|value| Value::from(value as $t))
                    .collect::<Vec<Value>>();

                *serial_counter += count;
                *serial_counter %= (self.max as i64);

                values
            },
            _ => unimplemented!()
        }
    }
}
    )+)
}

generate_values_impl! { 
    TinyIntUnderlyingType
    SmallIntUnderlyingType
    IntUnderlyingType
    BigIntUnderlyingType
    DecimalUnderlyingType
}

impl GenerateValues for GenerateType {
    fn gen_numeric_values(&self, dist: Dist, serial_counter: &mut i64, count: usize, rng: &mut ThreadRng) -> Vec<Value> {
        match self {
            GenerateType::TinyInt(t) => t.gen_numeric_values(dist, serial_counter, count, rng),
            GenerateType::SmallInt(t) => t.gen_numeric_values(dist, serial_counter, count, rng),
            GenerateType::Int(t) => t.gen_numeric_values(dist, serial_counter, count, rng),
            GenerateType::BigInt(t) => t.gen_numeric_values(dist, serial_counter, count, rng),
            GenerateType::Decimal(t) => t.gen_numeric_values(dist, serial_counter, count, rng),
        }
    }
}