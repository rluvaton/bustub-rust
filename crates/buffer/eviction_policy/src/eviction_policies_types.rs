use crate::{EvictionPolicy, LRUKEvictionPolicy};
use crate::lru_k::LRUKOptions;
use crate::traits::EvictionPolicyCreator;

pub enum EvictionPoliciesTypes {
    #[allow(non_camel_case_types)]
    LRU_K(LRUKOptions)
}

impl EvictionPoliciesTypes {
    pub fn create_policy(self, number_of_frames: usize) -> Box<dyn EvictionPolicy> {
        Box::new(
            match self {
                EvictionPoliciesTypes::LRU_K(options) => LRUKEvictionPolicy::new(number_of_frames, options)
            }
        )
    }
}

impl Default for EvictionPoliciesTypes {
    fn default() -> Self {
        EvictionPoliciesTypes::LRU_K(LRUKOptions::default())
    }
}
