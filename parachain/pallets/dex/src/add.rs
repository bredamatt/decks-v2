/// Add liquidity to the pool
    /// Note that this should adjust the value of the tokens
    pub(super) fn add(
        &self,
        amounts: (BalanceOf<T>, BalanceOf<T>),
        sender: &AccountIdOf<T>,
    ) -> Result<PriceOf<T>, DispatchError> {
        let issuance = T::Assets::total_issaunce(self.id);

        // Must check if the pool is empty
        if issuance == <BalanceOf<T>>::default() {
            T::Tokens::mint_into(self.id, sender, amounts.0)?;
            <Pallet<T>>::transfer(self.pair.0, sender, &self.account, amounts.0)?;
            <Pallet<T>>::transfer(self.pair.1, sender, &*self.account, amounts.1)?;
        } else {
            // If the pool is not initialised, have to calculate how to adjust balances
            let balances = (
                <Pallet<T>>::balance(self.pair.0, &self.account),
                <Pallet<T>>::balance(self.pair.1, &self.account),
            );
            let amount_1 = amounts.0 * (balances.1 / balances.0);
            let to_mint = amounts.0 * (issuance / balances.0);

            // Mint new tokens to distribute to the sender, and transfer to the pool
            T::Tokens::mint_into(self.id, sender, to_mint)?;
            <Pallet<T>>::transfer(self.pair.0, sender, &self.account, amounts.0)?;
            <Pallet<T>>::transfer(self.pair.1, sender, &self.account, amounts.1)?;
        }

    }