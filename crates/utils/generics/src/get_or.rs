use std::collections::HashMap;
use std::hash::Hash;

pub trait GetOr<K, V> {
    fn get_or<'a>(&'a self, key: &K, default: &'a V) -> &'a V;
}


impl<K: Eq + Hash, V> GetOr<K, V> for HashMap<K, V> {
    fn get_or<'a>(&'a self, key: &K, default: &'a V) -> &'a V {
        self.get(key).unwrap_or(default)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::GetOr;

    #[test]
    fn should_return_default_if_missing() {
        let map: HashMap<u32, u8> = HashMap::new();

        assert_eq!(map.get_or(&1, &0), &0);
        assert_eq!(map.get_or(&2, &0), &0);
        assert_eq!(map.get_or(&1, &3), &3);
    }

    #[test]
    fn should_return_existing_if_exists() {
        let mut map: HashMap<u32, u8> = HashMap::new();
        map.insert(1, 101);
        map.insert(2, 102);
        map.insert(3, 103);

        assert_eq!(map.get_or(&1, &0), &101);
        assert_eq!(map.get_or(&2, &0), &102);
        assert_eq!(map.get_or(&1, &3), &101);
        assert_eq!(map.get_or(&4, &6), &6);
    }
}
