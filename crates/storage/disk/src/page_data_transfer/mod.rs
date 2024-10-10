mod traits;
mod unsafe_single_ref_data;
mod unsafe_single_ref_mut_data;

pub use traits::AsPtr;

// TODO - change file names and remove pub
pub use unsafe_single_ref_data::PageDataTransfer;
pub use unsafe_single_ref_mut_data::MutPageDataTransfer;
