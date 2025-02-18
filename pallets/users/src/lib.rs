//! This is part of Chocolate Project for Encode Club Polkadot Hackathon
//! Consider all of this part is a work in progress

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
// for vectors and all else. In v4.0, all the vec and other imports haave been swept under prelude
// use sp_std::prelude::*;
// this uses vec from prelude
// use sp_std::vec::Vec;
// this isn't accessible in the child modl pallet.
// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
		UserCreated(T::AccountId),
	}

	#[pallet::storage]
	// users store
	pub type Users<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, User>;

	use v1::User;
	pub mod v1 {
		use codec::{Decode, Encode};
		use sp_std::vec::Vec;

		#[derive(Encode, Decode, Default, Clone)]
		pub struct User {
			pub rank_points: u32,
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No Value
		NoneValue,
		/// Storage is overflow
		StorageOverflow,
		/// User already exists
		UserAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// use base weight then add on any additional operations
		// strings are u8 arrays. utf-8
		/// Signed transaction to create user
		#[pallet::weight(0 + T::DbWeight::get().writes(1))]
		pub fn make_user(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Users::<T>::contains_key(&who), Error::<T>::UserAlreadyExists);
			<Users<T>>::insert(&who, User { rank_points: 0 });

			Self::deposit_event(Event::UserCreated(who));

			Ok(())
		}
	}
}
