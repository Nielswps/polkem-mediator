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
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    pub use crate::pallet;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Storage Map for Trades by Hash to a Trade
    #[pallet::storage]
    #[pallet::getter(fn trades)]
    pub(super) type Trades<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Trade<T>>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        TradeAdded(u16, u8, T::AccountId, T::AccountId, T::Hash),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100000)]
        #[pallet::call_index(1)]
        pub fn organize_trades(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // ToDo: Fetch supply and demand from micro grids

            // let trade = Trade { amount: 0, price: 100, seller: None, buyer: None };
            // let trade_id = T::Hashing::hash_of(&trade);
            // <Trades<T>>::insert(trade_id, trade.clone());
            //
            // // Emit an event.
            // Self::deposit_event(Event::TradeAdded(trade.amount.clone(), trade.price.clone(), trade.seller.clone(), trade.buyer.clone(), trade_id));

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Trade<T: Config> {
        pub amount: u16,
        pub price: u8,
        pub seller: <T as frame_system::Config>::AccountId,
        pub buyer: <T as frame_system::Config>::AccountId,
    }
}
