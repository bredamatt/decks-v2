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

	/// Needed to inspect an token`
	type AssetIdOf<T> =
		<<T as Config>::Tokens as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

	/// Needed to insepct a token
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

		type NativeTokenId: Get<Self::AssetId>;

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
			// Check if we have a liquidity pool token id
			let lp_token_id = <GetLpTokenId<T>>::get().unwrap_or_else(|| AssetIdOf::<T>::max_value());

			// Check if the liquidity pool token already exits
			ensure!(!T::exists(lp_token_id), Error::<T>::TokenAlreadyExists);

			// Create token for liquidity pool
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

		pub fn add_liquidity(
			&self,
			amounts: (BalanceOf<T>, BalanceOf<T>),
			sender: &AccountIdOf<T>,
		) -> DispatchResult {
			let issuance = T::Tokens::total_issuance(self.id);
			if issuance == <BalanceOf<T>>::default() {
				T::Tokens::mint_into(self.id, sender, amounts.0)?;
				T::Tokens::teleport(self.pair.0, sender, &self.account, amounts.0)?;
				T::Tokens::teleport(self.pair.1, sender, &self.account, amounts.1)?;
			} else {
				let balances = (
					T::Tokens::balance(self.pair.0, &self.account),
					T::Tokens::balance(self.pair.1, &self.account),
				);
				let amount_1 = amounts.0 * (balances.1 / balances.0);
				let to_mint = amounts.0 * (issuance / balances.0);
				T::Tokens::mint_into(self.id, sender, to_mint)?;
				T::Tokens::teleport(self.pair.0, &sender, &self.account, amounts.0)?;
				T::Tokens::teleport(self.pair.1, &sender, &self.account, amounts.1)?;
			}
			Ok(())
		}
	}

	/// Used to make sure pools of two tokens can only exist once
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Pair<T: Config>(PhantomData<T>);

	/// Can use inspect ()
	impl<T: Config> Pair<T> {
		pub fn new_pair(
			token_0: AssetIdOf<T>,
			token_1: AssetIdOf<T>,
		) -> (AssetIdOf<T>, AssetIdOf<T>) {
			if token_1 < token_0 {
				(token_1, token_0)
			} else {
				(token_0, token_1)
			}
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Can be used to get the asset id of a liquidity poo ltoken
	#[pallet::storage]
	pub(super) type GetLpTokenId<T: Config> = StorageValue<_, AssetIdOf<T>>;

	#[pallet::storage]
	pub(super) type LiquidityPools<T: Config> = StorageMap<_, Twox64Concat, (AssetIdOf<T>, AssetIdOf<T>), LiquidityPool<T>>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// For when an LP is created (token_0, token_1)
		LiquidityPoolCreated(AssetIdOf<T>, AssetIdOf<T>),
		/// For when liquidity is added to pre-existing pool (token_0, amt_0, token_1, amt_1)
		LiquidityAdded(AssetIdOf<T>,  BalanceOf<T>, AssetIdOf<T>, BalanceOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The liquidity pool token already exists.
		TokenAlreadyExists,
		/// Input the same token twice when ading liquidity
		IdenticalTokens,
		/// Sent a Zero value
		AmountZero,
		/// Sent invalid amount
		InvalidAmount,
		/// Sender has too few funds
		InsufficientBalance,
		/// Sent a non-existent token
		NonExistentToken,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			amount_0: BalanceOf<T>,
			token_0: AssetIdOf<T>,
			amount_1: BalanceOf<T>,
			token_1: AssetIdOf<T>,
		) -> DispatchResult {

			// Make sure extrinsic is signed
			let sender = ensure_signed(origin)?;

			// Input check
			ensure!(token_0 != token_1, Error::<T>::IdenticalTokens); // Make sure we don't input the same token twice
			ensure!(amount_0 > <BalanceOf<T>>::default(), Error::<T>::AmountZero); // Make sure we don't input zero amount
			ensure!(amount_1 > <BalanceOf<T>>::default(), Error::<T>::AmountZero); // Make sure we don't input zero amount
			ensure!(amount_0 != <BalanceOf<T>>::default() && amount_1 != <BalanceOf<T>>::default(), Error::<T>::InvalidAmount); // Check if either amount valid
			ensure!(T::exists(token_0) && T::exists(token_1), Error::<T>::NonExistentToken); // Ensure token exists
			
			// Ensure sender has sufficient balance
			ensure!(Self::balance(token_0, &sender) >= amount_0
				&& Self::balance(token_1, &sender) >= amount_1,
				Error::<T>::InsufficientBalance
			); 

			// Create pair from supplied tokens
			let pair = Pair::<T>::new_pair(token_0, token_1);

			// Get/create liquidity pool
			let lp_key = (pair.0, pair.1);

			let pool = match <LiquidityPools<T>>::get(lp_key) {
				Some(pool) => Result::<LiquidityPool<T>, DispatchError>::Ok(pool),
				None => {
					// Create new pool, save and emit event
					let pool = <LiquidityPool<T>>::new_liquidity_pool(lp_key)?;
					<LiquidityPools<T>>::set(lp_key, Some(pool.clone()));
					Self::deposit_event(Event::LiquidityPoolCreated(pair.0, pair.1));
					Ok(pool)
				},
			}?;

			// Add liquidity
			pool.add_liquidity((amount_0, amount_1), &sender)?;
			Self::deposit_event(Event::LiquidityAdded(
				token_0,
				amount_0,
				token_1,
				amount_1,
			));
			Ok(())
		}
	}

	// Internal functions to be used by this pallet
	impl<T: Config> Pallet<T> {
		/// Get the balance of a token given an account
		fn balance(id: AssetIdOf<T>, who: &AccountIdOf<T>) -> BalanceOf<T> {
			if id == T::NativeTokenId::get() {
				T::NativeCurrency::total_balance(who)
			} else {
				// Otherwise use asset balance
				T::Tokens::balance(id, &who)
			}
		}
	}

	// Configuration of the DEX state at genesis
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// Genesis liquidity pools: ((amount, asset), (amount, asset), liquidity provider)
		pub liquidity_pools: Vec<((BalanceOf<T>, AssetIdOf<T>), (BalanceOf<T>, AssetIdOf<T>), AccountIdOf<T>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { liquidity_pools: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
				for (token_0, token_1, sender) in &self.liquidity_pools {
					let pair = Pair::<T>::new_pair(token_0.1,token_1.1,);

					let new_pool = LiquidityPool::<T>::new_liquidity_pool(pair)
						.expect("Should be able to create new LiquidityPool during genesis");
					
					let pallet_id = T::PalletId::get();

					new_pool.add_liquidity((token_0.0, token_1.0), &pallet_id.into_account_truncating())
						.expect("Should be able to add liquidity during genesis");

					LiquidityPools::<T>::insert(pair, new_pool);
				}
			}
		}
	}
