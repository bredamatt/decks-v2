#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use sp_runtime::{traits::AtLeast32BitUnsigned, traits::Bounded};
	use frame_support::traits::{Currency, ReservableCurrency};
	use frame_support::traits::tokens::fungibles::{Mutate, InspectMetadata, Inspect, Create};
	use frame_support::traits::tokens::fungibles::metadata::Mutate as MutateMetadata;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::PalletId;
	use codec::HasCompact;
	use sp_runtime::traits::AccountIdConversion;

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	type AssetIdOf<T> =
		<<T as Config>::Tokens as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

	type BalanceOf<T> =
		<<T as Config>::Tokens as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
		
	type NativeBalanceOf<T> =
		<<T as Config>::NativeCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type LpTokenMinimumBalance: Get<
			<Self::Tokens as Inspect<<Self as frame_system::Config>::AccountId>>::Balance>;

		type LpTokenDecimals: Get<u8>;

		type Tokens: Create<Self::AccountId>
			+ Mutate<
				Self::AccountId,
				AssetId = Self::AssetId,
				Balance = NativeBalanceOf<Self>,
			> + MutateMetadata<
				Self::AccountId,
				AssetId = Self::AssetId,
				Balance = NativeBalanceOf<Self>,
			> + StorageInfoTrait + InspectMetadata<Self::AccountId>;

		type AssetId: AtLeast32BitUnsigned
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ TypeInfo
			+ Parameter
			+ Default
			+ Copy
			+ MaxEncodedLen
			+ PartialOrd;

		type PalletId: Get<PalletId>;

		type NativeCurrency: ReservableCurrency<Self::AccountId>;

		fn exists(id: Self::AssetId) -> bool;
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct LiquidityPool<T: Config> {
		pub id: AssetIdOf<T>,
		pub pair: (AssetIdOf<T>, AssetIdOf<T>),
		pub account: AccountIdOf<T>,
	}

	impl<T: Config> LiquidityPool<T> {
		pub fn new_liquidity_pool(pair: (AssetIdOf<T>, AssetIdOf<T>)) -> Result<Self, DispatchError> {
			let lp_token_id = Self::create_liquidity_pool_token(pair)?;
			let account = T::PalletId::get().into_sub_account_truncating(lp_token_id);
			let pool = Self { id: lp_token_id, pair, account };
			Ok(pool)
		}

		fn create_liquidity_pool_token(pair: (AssetIdOf<T>, AssetIdOf<T>)) -> Result<AssetIdOf<T>, DispatchError> {
			let lp_token_id = <GetLpTokenId<T>>::get().unwrap_or_else(|| AssetIdOf::<T>::max_value());
			// println!("{:?}", lp_token_id);

			// Check if the liquidity pool token already exits
			ensure!(!T::exists(lp_token_id), Error::<T>::TokenAlreadyExists);
	
			// Create asset
			let dex_id: T::AccountId = T::PalletId::get().into_account_truncating();
			T::Tokens::create(lp_token_id, dex_id.clone(), true, T::LpTokenMinimumBalance::get())?;
	
			// Set asset metadata based on existing assets
			let mut asset_0 = T::Tokens::symbol(&pair.0);
			let asset_1 = T::Tokens::symbol(&pair.1);
			asset_0.extend(asset_1);
			T::Tokens::set(lp_token_id, &dex_id, asset_0.clone(), asset_0, T::LpTokenDecimals::get())?;
	
			// Set next value to be used
			<GetLpTokenId<T>>::set(Some(lp_token_id - 1u32.into()));
	
			Ok(lp_token_id)
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub(super) type GetLpTokenId<T: Config> = StorageValue<_, AssetIdOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The liquidity pool token already exists.
		TokenAlreadyExists,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}