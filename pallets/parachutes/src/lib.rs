// All pallets must be configured for `no_std`.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
use frame_support::{ BoundedVec, ensure };
use frame_system::pallet_prelude::OriginFor;
use scale_info::prelude::vec::Vec;

use pallet_bro;

use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo + pallet_bro::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo;

        // #[pallet::constant]
        // type MaxLength: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        InService { bro: T::AccountId, chute_name: BoundedVec<u8, T::MaxLength> },
        Rigged { bro: T::AccountId, chute_name: BoundedVec<u8, T::MaxLength> },
        Jumped { bro: T::AccountId, chute_name: BoundedVec<u8, T::MaxLength> },
        Repaired { bro: T::AccountId, chute_name: BoundedVec<u8, T::MaxLength> },
        ParachuteRetired { bro: T::AccountId, chute_name: BoundedVec<u8, T::MaxLength> },
	}

	#[pallet::error]
	pub enum Error<T> {
        AlreadyInService,
        NotARigger,
        NotABro,
        ParachuteDoesNotExist,
        TooLong,
        TooShort,
	}

	#[pallet::storage]
    #[pallet::getter(fn main_store)]
	pub(super) type MainStore<T: Config> = StorageMap<_, Blake2_128Concat, MainId, BoundedVec<u8, T::MaxLength>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(Weight::default())]
		#[pallet::call_index(0)]
        pub fn in_service(origin: OriginFor<T>, id: MainId, name: Vec<u8>) -> DispatchResult {
            let bro = ensure_signed(origin)?;
            ensure!(pallet_bro::Pallet::<T>::bro_store(bro.clone()).unwrap().1 == true, Error::<T>::NotARigger);
            ensure!(!MainStore::<T>::contains_key(id.clone()), Error::<T>::AlreadyInService);
            
            let bounded_name: BoundedVec<_,_> =
                name.try_into().map_err(|_| Error::<T>::TooLong)?;
            ensure!(bounded_name.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);
            
            MainStore::<T>::insert(id, bounded_name.clone());

            Self::deposit_event(Event::InService { bro, chute_name: bounded_name });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(1)]
        pub fn rig(origin: OriginFor<T>, id: MainId) -> DispatchResult {
            let bro = ensure_signed(origin)?;
            ensure!(MainStore::<T>::contains_key(id.clone()), Error::<T>::ParachuteDoesNotExist);
            let chute_name = MainStore::<T>::get(&id).ok_or(Error::<T>::ParachuteDoesNotExist).unwrap();
            Self::deposit_event(Event::Rigged { bro, chute_name });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(2)]
        pub fn jump(origin: OriginFor<T>, id: MainId) -> DispatchResult {
            let bro = ensure_signed(origin)?;
            ensure!(MainStore::<T>::contains_key(id.clone()), Error::<T>::ParachuteDoesNotExist);
            let chute_name = MainStore::<T>::get(&id).ok_or(Error::<T>::ParachuteDoesNotExist).unwrap();
            Self::deposit_event(Event::Jumped { bro, chute_name });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(3)]
        pub fn repair(origin: OriginFor<T>, id: MainId) -> DispatchResult {
            let bro = ensure_signed(origin)?;
            ensure!(pallet_bro::Pallet::<T>::bro_store(bro.clone()).unwrap().1 == true, Error::<T>::NotARigger);
            ensure!(MainStore::<T>::contains_key(id.clone()), Error::<T>::ParachuteDoesNotExist);
            let chute_name = MainStore::<T>::get(&id).ok_or(Error::<T>::ParachuteDoesNotExist).unwrap();
            Self::deposit_event(Event::Repaired { bro, chute_name });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(4)]
        pub fn retire(origin: OriginFor<T>, id: MainId) -> DispatchResult {
            let bro = ensure_signed(origin)?;
            ensure!(pallet_bro::Pallet::<T>::bro_store(bro.clone()).unwrap().1 == true, Error::<T>::NotARigger);
            ensure!(MainStore::<T>::contains_key(id.clone()), Error::<T>::ParachuteDoesNotExist);
            let chute_name = MainStore::<T>::get(&id.clone()).ok_or(Error::<T>::ParachuteDoesNotExist).unwrap();
            MainStore::<T>::remove(&id);
            Self::deposit_event(Event::ParachuteRetired { bro, chute_name });
            Ok(())
        }
	}
}

pub mod weights {
	// Placeholder struct for the pallet weights
	pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
}
