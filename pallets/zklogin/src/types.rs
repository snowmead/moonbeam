use sp_core::ConstU32;
use sp_runtime::BoundedVec;

pub type ImageId = BoundedVec<u32, ConstU32<8>>;
