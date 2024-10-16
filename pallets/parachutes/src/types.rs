use crate::Config;
use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::{BoundedVec, MaxEncodedLen, RuntimeDebug},
    traits::Get,
};

use scale_info::TypeInfo;

pub type MainId = u128;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, Clone, MaxEncodedLen)]
#[scale_info(skip_type_params(S))]
pub struct MainDetails<S: Get<u32>> {
    pub name: BoundedVec<u8, S>,
    pub repairs: BoundedVec<u8, S>,
    pub jumps: u8,
    pub rigs: u8,
}

impl<S: Get<u32>> MainDetails<S> {
    pub fn new(id: MainId, name: BoundedVec<u8, S>) -> Self {
        MainDetails {
            name,
            repairs: BoundedVec::new(),
            jumps: 0,
            rigs: 0,
        }
    }

}
