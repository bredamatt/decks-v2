#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use pallet_assets as assets;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_kitties::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Tokens: Create<Self::AccountId> + Mutate<Self::AccountId, AssetId = Self::AssetId, Balance = BalanceOf<Self>>;
		
		type TokenMinimalBalance: Get<<Self::Tokens as Inspect<<Self as frame_system::Config>::AccountId>>::Balance>;
	}

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type AssetIdOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
	type BalanceOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Get all liquidity pools
	#[pallet::storage]
	pub(super) type LiquidtyPools<T: Config> =
		StorageMap<_, Twox64Concat, (AssetIdOf<T>, AssetIdOf<T>), LiquidityPool<T>>;

	/// Get LiquidityPool token asset identifier
	#[pallet::storage]
	pub(super) type LiquidityPoolTokenAssetId<T: Config> = StorageValue<_, AssetIdOf<T>>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// LiquidityPool was created
		LiquidityPoolCreated(AssetIdOf<T>, AssetIdOf<T>),
		/// Liquidity was added to a pool
		LiquidityAdded(BalanceOf<T>, AssetIdOf<T>, BalanceOf<T>),
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
			amts: (BalanceOf<T>, BalanceOf<T>),
		) -> DispatchResult {
			Ok(())	
		}
	}

	// Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {}
}