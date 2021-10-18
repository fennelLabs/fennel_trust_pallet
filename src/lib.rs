#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_system::pallet::*;
pub use frame_support::storage::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn value1)]
	pub type CurrentIssued<T: Config> = StorageValue<_, u32>;
	#[pallet::storage]
	#[pallet::getter(fn key1)]
	pub type TrustIssuance<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, T::AccountId>;
	#[pallet::storage]
	#[pallet::getter(fn value2)]
	pub type CurrentRevoked<T: Config> = StorageValue<_, u32>;
	#[pallet::storage]
	#[pallet::getter(fn key2)]
	pub type TrustRevocation<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, T::AccountId>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TrustIssued(T::AccountId, T::AccountId),
		TrustRevoked(T::AccountId, T::AccountId),
		TrustIssuanceRemoved(T::AccountId, T::AccountId),
		TrustRevocationRemoved(T::AccountId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Fully give your trust to an account 
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn issue_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// WARN: THIS ITERATION IS VERY INEFFICIENT
			// NOT SUITABLE FOR PRODUCTION
			let mut do_insert = true;
			let mut i = 0;
			for (_index, issued) in <TrustIssuance<T>>::iter_prefix(&who) {
				if issued == address { do_insert = false; }
				i += 1;
			}

			if do_insert {
				<TrustIssuance<T>>::insert(&who, i, &address);
				let total: u32 = <CurrentIssued<T>>::get().unwrap();
				<CurrentIssued<T>>::put(total + 1);
				Self::deposit_event(Event::TrustIssued(who, address));
			}
			
			Ok(())
		}

		/// Remove issued trust from an account, making their trust status 'Unknown'
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut do_remove: Option<u32> = None;
			for (index, issued) in <TrustIssuance<T>>::iter_prefix(&who) {
				if issued == address { do_remove = Some(index); }
			}

			if let Some(index) = do_remove {
				<TrustIssuance<T>>::remove(&who, index);
				let key = <CurrentIssued<T>>::get().unwrap();
				<CurrentIssued<T>>::put(key - 1);
				Self::deposit_event(Event::TrustIssuanceRemoved(address, who));
			}

			Ok(())
		}

		/// Revoke trust from an account
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn revoke_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut do_insert = true;
			let mut i = 0;
			for (_index, revoked) in <TrustRevocation<T>>::iter_prefix(&who) {
				if revoked == address { do_insert = false }
				i += 1;
			}

			if do_insert {
				<TrustRevocation<T>>::insert(&who, i, &address);
				let key: u32 = <CurrentRevoked<T>>::get().unwrap();	
				<CurrentRevoked<T>>::put(key + 1);
				Self::deposit_event(Event::TrustRevoked(address, who));
			}
		
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_revoked_trust(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut do_remove: Option<u32> = None;

			for (index, revoked) in <TrustRevocation<T>>::iter_prefix(&who) {
				if revoked == address { do_remove = Some(index) }
			}
			
			if let Some(index) = do_remove {
				<TrustRevocation<T>>::remove(&who, index);	
				let key: u32 = <CurrentRevoked<T>>::get().unwrap();
				<CurrentRevoked<T>>::put(key - 1);
				Self::deposit_event(Event::TrustRevocationRemoved(address, who));
			}

			Ok(())
		}
	}
}
