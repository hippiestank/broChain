// All pallets must be configured for `no_std`.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
pub use scale_info::prelude::vec::Vec;
// pub use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
// pub mod types;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo;

        #[pallet::constant]
        type MaxLength: Get<u32>;

        #[pallet::constant]
        type MinLength: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BroCreated { bro: T::AccountId },
        RiggerSet { bro: T::AccountId, rigger: bool }
	}

	#[pallet::error]
	pub enum Error<T> {
        AlreadyARigger,
		BroAlreadyCreated,
        BroDoesNotExist,
        NotABro,
        TooLong,
        TooShort,
	}

	#[pallet::storage]
    #[pallet::getter(fn bro_store)]
	pub(super) type BroStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat, 
        T::AccountId, 
        (BoundedVec<u8, T::MaxLength>, bool),
    >;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(Weight::default())]
		#[pallet::call_index(0)]
		pub fn create_bro(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
			let bro = ensure_signed(origin)?;
            ensure!(!BroStore::<T>::contains_key(bro.clone()), Error::<T>::BroAlreadyCreated);

            let bounded_name: BoundedVec<_,_> =
                name.try_into().map_err(|_| Error::<T>::TooLong)?;
            ensure!(bounded_name.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);

            let rigger = false;
            let bro_details = ( bounded_name, rigger );

			BroStore::<T>::insert(bro.clone(), bro_details);
			Self::deposit_event(Event::<T>::BroCreated { bro });
			Ok(())
		}

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(1)]
        pub fn set_rigger(origin: OriginFor<T>, rigger: bool) -> DispatchResult {
            let bro = ensure_signed(origin)?;
            
            let (bounded_name, maybe_rigger) = BroStore::<T>::get(bro.clone()).ok_or(Error::<T>::NotABro)?;
            ensure!(maybe_rigger == false, Error::<T>::AlreadyARigger);

            let bro_details = (bounded_name, rigger);
            BroStore::<T>::set(bro.clone(), Some(bro_details));
            Self::deposit_event(Event::<T>::RiggerSet { bro, rigger });
            Ok(())
        }
	}
}

pub mod weights {
	// Placeholder struct for the pallet weights
	pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
}
