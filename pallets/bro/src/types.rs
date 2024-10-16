use crate::Config;
use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::{BoundedVec, MaxEncodedLen, RuntimeDebug},
    traits::Get,
};
use sp_std::prelude::*;
pub use sp_runtime::traits::{StaticLookup};

use scale_info::TypeInfo;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, Clone, MaxEncodedLen)]
#[scale_info(skip_type_params(S))]
pub struct BroDetails<T: Config, S: Get<u32>, Bool> {
    pub name: BoundedVec<u8, S>,
    pub rigger: bool,
}

impl<T: Config, S: Get<u32>, Bool> BroDetails<T, S, Bool> {
    pub fn new(id: T::AccountId, name: BoundedVec<u8,S>) -> Self {
        BroDetails {
            name,
            rigger: false,
        }
    }
}
