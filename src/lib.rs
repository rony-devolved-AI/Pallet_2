#![cfg_attr(not(feature = "std"), no_std)]


use pallet::*;

#[cfg(test)]
mod tests;

use frame_support::PalletId;


#[cfg(test)]
mod mock;

#[frame_support::pallet]
pub mod pallet{
    
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config:frame_system::Config{
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;



        #[pallet::constant]
        type PalletId:Get<PalletId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when an ID is set.
        IdSet(T::AccountId, u32),
        /// Event emitted when an ID is retrieved.
        IdRetrieved(T::AccountId, Option<u32>),
    }


    #[pallet::error]
    pub enum Error<T>{
        IdNotFound
    }


    #[pallet::storage]
    #[pallet::getter(fn some_storage_item)]
    pub type NameToId<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;


    #[pallet::call]
    impl <T:Config>Pallet<T>{
        #[pallet::call_index(1)]
        #[pallet::weight({70_000})]
        pub fn set_Id(origin:OriginFor<T>,id:u32)->DispatchResult{
            let sender=ensure_signed(origin)?;
            NameToId::<T>::insert(&sender, id);
            Self::deposit_event(Event::IdSet(sender,id));
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight({70_000})]
        pub fn get_id(origin:OriginFor<T>)->DispatchResult{
            let sender=ensure_signed(origin)?;
            let id=NameToId::<T>::get(&sender);
            Self::deposit_event(Event::IdRetrieved(sender,id));
            Ok(())
        }
    }
    

}
