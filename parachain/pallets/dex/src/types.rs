use super::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct LiquidityPool<T: Config> {
    /// The ID of the liquidity pool asset
    pub(super) id: AssetIdOf<T>,

    /// The ids of the asset pairs (can also be used to identify the LP)
    pub(super) pair: (AssetIdOf<T>, AssetIdOf<T>),

    /// The account holding the assets
    pub(super) account: AccountIdOf<T>,
}

impl <T:Config> LiquidityPool <T> {
    /// Create a new LP from two assets
    pub(super) fn new_lp(pair: (AssetIdOf<T>, AssetIdOf<T>)) -> Result<Self, DispatchError> {
        let id = Self::create_lp_token(pair)?;
        let account = T::PalletId::get().into_sub_account(id);
        Ok(Self { id, pair, account })
    }

    /// Create a liquidity pool token from two assets
    fn create_lp_token(pair: (AssetIdOf<T>, AssetIdOf<T>)) -> Result<AssetIdOf<T>, DispatchError> {
        let id = <LiquidityPoolTokenAssetId<T>>::get().unwrap();

        // Make sure the id has not been used
        ensure!(!T::exists(id), Error::<T>::LiquidityPoolTokenAlreadyExists);

        // Create the asset
        let dex: T::AccountId = T::PalletId::get().into_account_truncating();
        T::Tokens::create(id, dex.clone(), true, T::TokenMinimumBalance::get())?;

        Ok(id)
    }

}