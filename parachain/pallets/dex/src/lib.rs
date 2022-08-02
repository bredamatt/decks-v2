#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::tokens::fungibles::{Inspect, Mutate, Transfer};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// To deal with Assets
		type Tokens: Inspect<Self::AccountId> + Mutate<Self::AccountId> + Transfer<Self::AccountId>;
	}

	type AnAssetId<T: Config> = <T::Tokens as Inspect<T::AccountId>>::AssetId;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	pub type LiquidityPool<T> = StorageValue<_, [u8; 16]>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Liquidty Pool created
		LiquidityPoolCreated { id: [u8; 16] }
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The Liquidity pool exists
		LiquidityPoolExists
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::weight(0)]
		pub fn create_pool(
			origin: OriginFor<T>,
			pair: (AnAssetId<T>, AnAssetId<T>),
		) -> DispatchResult {
			Ok(())	
		}
	}

	// Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {}

}
