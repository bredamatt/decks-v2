#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, Randomness};

    // The basis which we buil
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Your Pallet's configuration trait, representing custom external types and interfaces.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the kitties pallet.
        type Currency: Currency<Self::AccountId>;

        /// The maximum amount of kitties a single account can own.
        #[pallet::constant]
        type MaxKittiesOwned: Get<u32>;

        /// The type of Randomness we want to specify for this pallet.
        type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    // The Gender type used in the `Kitty` struct
    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Gender {
        Male,
        Female,
    }

    // Struct for holding kitty information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        // Using 16 bytes to represent a kitty DNA
        pub dna: [u8; 16],
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub gender: Gender,
        pub owner: T::AccountId,
    }

    /// Keeps track of the number of kitties in existence.
    #[pallet::storage]
    pub(super) type CountForKitties<T: Config> = StorageValue<
        _, 
        u64, 
        ValueQuery
    >;

    /// Maps the kitty struct to the kitty DNA.
    #[pallet::storage]
    pub(super) type Kitties<T: Config> = StorageMap<
        _,
        Twox64Concat,
        [u8; 16],
        Kitty<T>
    >;

    /// Track the kitties owned by each account.
    #[pallet::storage]
    pub(super) type KittiesOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<[u8; 16], T::MaxKittiesOwned>,
        ValueQuery,
    >;

    // Your Pallet's events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new kitty was successfully created.
        Created { kitty: [u8; 16], owner: T::AccountId },

        /// A kitty was transferred
        Transferred { from: T::AccountId, to: T::AccountId, kitty: [u8;16] },

        /// The price was successfully set
        PriceSet { kitty: [u8; 16], price: Option<BalanceOf<T>> },

        /// Sold event
        Sold { seller: T::AccountId, buyer: T::AccountId, kitty: [u8; 16], price: BalanceOf<T> },
    }

    // Your Pallet's error messages.
    #[pallet::error]
    pub enum Error<T> {
        /// An account may only own `MaxKittiesOwned` kitties.
        TooManyOwned,
        /// This kitty already exists!
        DuplicateKitty,
        /// An overflow has occured!
        Overflow,
        /// Doesn't exist
        NoKitty,
        /// Not the owner
        NotOwner,
        /// Can't transfer or buy from oneself
        TransferToSelf,
        /// Ensures that the buying price is greater than the asking price.
        BidPriceTooLow,
        /// This kitty is not for sale.
        NotForSale,
    }

    // Your Pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new unique kitty.
        ///
        /// The actual kitty creation is done in the `mint()` function.
        #[pallet::weight(0)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Generate unique DNA and Gender using a helper function
            let (kitty_gen_dna, gender) = Self::gen_dna();

            // Write new kitty to storage by calling helper function
            Self::mint(&sender, kitty_gen_dna, gender)?;

            Ok(())
        }

        /// Directly transfer a kitty to another participant
        /// 
        /// Any account with a kitty can send it to another Account. 
        /// Note that this will reset the asking price of the kitty, marking it not for sale.
        #[pallet::weight(0)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            kitty_id: [u8; 16],
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            ensure!(kitty.owner == from, Error::<T>::NotOwner);
            Self::do_transfer(kitty_id, to)?;
            Ok(())
        }

        /// Set the price
        /// 
        /// Update kitty price and storage
        #[pallet::weight(0)]
        pub fn set_price(
            origin: OriginFor<T>,
            kitty_id: [u8; 16],
            new_price: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Make sure the owner is the sender
            let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            ensure!(kitty.owner == sender, Error::<T>::NotOwner);

            // Set the price in storage
            kitty.price = new_price;
            Kitties::<T>::insert(&kitty_id, kitty);

            // Deposit a "PriceSet" event
            Self::deposit_event(Event::PriceSet { kitty: kitty_id, price: new_price });

            Ok(())
        }
    }

    // Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {
        // Generates and returns DNA and Gender
        fn gen_dna() -> ([u8; 16], Gender) {
            // Create randomness
            let random = T::KittyRandomness::random(&b"dna"[..]).0;

            // Create randomness payload. Multiple kitties can be generated in the same block,
            // retaining uniqueness.
            let unique_payload = (
                random,
                frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
                frame_system::Pallet::<T>::block_number(),
            );

            // Turns into a byte array
            let encoded_payload = unique_payload.encode();
            let hash = frame_support::Hashable::blake2_128(&encoded_payload);

            // Generate Gender
            if hash[0] % 2 == 0 {
                (hash, Gender::Male)
            } else {
                (hash, Gender::Female)
            }
        }

        // Helper to mint a kitty
        pub fn mint(
            owner: &T::AccountId,
            dna: [u8; 16],
            gender: Gender,
        ) -> Result<[u8; 16], DispatchError> {
            // Create a new object
            let kitty = Kitty::<T> { dna, price: None, gender, owner: owner.clone() };

            // Check if the kitty does not already exist in our storage map
            ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

            // Performs this operation first as it may fail
            let count = CountForKitties::<T>::get();
            let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

            // Append kitty to KittiesOwned
            KittiesOwned::<T>::try_append(&owner, kitty.dna)
                .map_err(|_| Error::<T>::TooManyOwned)?;

            // Write new kitty to storage
            Kitties::<T>::insert(kitty.dna, kitty);
            CountForKitties::<T>::put(new_count);

            // Deposit our "Created" event.
            Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

            // Returns the DNA of the new kitty if this succeeds
            Ok(dna)
        }
        
        // Update storage to transfer
        pub fn do_transfer(
            kitty_id: [u8; 16],
            to: T::AccountId,
        ) -> DispatchResult {
            let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            let from = kitty.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = KittiesOwned::<T>::get(&from);

            // Remove kitty from list of owned kitties for the person sending the kitty
            if let Some(i) = from_owned.iter().position(|&id| id == kitty_id) {
                from_owned.swap_remove(i);
            } else {
                return Err(Error::<T>::NoKitty.into())
            }

            // Add kitty to the list of owned kitties for the person receiving the kitty
            let mut to_owned = KittiesOwned::<T>::get(&to);
            to_owned.try_push(kitty_id).map_err(|()| Error::<T>::TooManyOwned)?;

            // Transfer succeed, update owner and reset price to `None`
            kitty.owner = to.clone();
            kitty.price = None;
            
            // Write to storage
            Kitties::<>::insert(&kitty_id, kitty);
            KittiesOwned::<T>::insert(&to, to_owned);
            KittiesOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::Transferred { from, to, kitty: kitty_id });

            Ok(())

        }

        pub fn do_buy_kitty(
            kitty_id: [u8; 16],
            to: T::AccountId,
            bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            let from = kitty.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = KittiesOwned::<T>::get(&from);

            if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
                from_owned.swap_remove(ind);
            } else {
                return Err(Error::<T>::NoKitty.into())
            }

            let mut to_owned = KittiesOwned::<T>::get(&to);
            to_owned.try_push(kitty_id).map_err(|()| Error::<T>::TooManyOwned)?;

            if let Some(price) = kitty.price {
                ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);
                T::Currency::transfer(&to, &from, price, frame_support::traits::ExistenceRequirement::KeepAlive)?;
                // Deposit sold event
                Self::deposit_event(Event::Sold {
                    seller: from.clone(),
                    buyer: to.clone(),
                    kitty: kitty_id,
                    price,
                });
            } else {
                return Err(Error::<T>::NotForSale.into())
            }

            // Transfer succeeded
            kitty.owner = to.clone();
            kitty.price = None;

            Kitties::<T>::insert(&kitty_id, kitty);
            KittiesOwned::<T>::insert(&to, to_owned);
            KittiesOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::Transferred { from, to, kitty: kitty_id });

            Ok(())

        }
    }
}