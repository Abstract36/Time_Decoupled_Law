#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    // Import TASM Core Logic
    // Note: In a real node, we would import the crate. 
    // Here we define the Structs locally to match tasm-core for the prototype.
    // Ideally, tasm-core should implement `scale_info::TypeInfo`.

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct IntentData<AccountId, Balance> {
        pub creator: AccountId,
        pub description_hash: [u8; 32],
        pub deadline: u64, // BlockNumber
        pub collateral: Balance,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct AbsenceData<AccountId, Balance> {
        pub intent_id: [u8; 32],
        pub creator: AccountId,
        pub deadline: u64,
        pub slashed_amount: Balance,
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// Currency for Collateral
        type Currency: frame_support::traits::Currency<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn intents)]
	pub type Intents<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], IntentData<T::AccountId, BalanceOf<T>>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn absences)]
    pub type Absences<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], AbsenceData<T::AccountId, BalanceOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub type LastProcessedBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    type BalanceOf<T> = <<T as Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IntentDeclared { intent_id: [u8; 32], who: T::AccountId, deadline: T::BlockNumber },
        AbsenceCrystallized { intent_id: [u8; 32], who: T::AccountId, slashed: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		IntentAlreadyExists,
        DeadlineInPast,
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // Simplified Logic: 
            // In a real implementation, we would likely use an ordered set (BTreeMap)
            // or a secondary index "Deadline -> Vec<IntentId>" to avoid iterating everything.
            // For Phase 0/1 Prototype, checking all active intents is O(N) but acceptable for logical proof.
            
            // 1. Iterate all active intents
            let mut absences_found = Vec::new();
            
            // Note: Storage iteration is expensive. 
            // Optimally: Intents::<T>::iter_values() check deadline < n
            
            // NOTE: Substrate BlockNumbers need conversion to u64
            let current_block_u64: u64 = n.try_into().unwrap_or(0);

            for (intent_id, intent) in Intents::<T>::iter() {
                if intent.deadline < current_block_u64 {
                    absences_found.push((intent_id, intent));
                }
            }

            // 2. Crystallize Absences
            for (id, intent) in absences_found {
                // Remove from Active
                Intents::<T>::remove(id);

                // Slash Collateral
                // T::Currency::slash(&intent.creator, intent.collateral); 
                // (Assuming slash impl)
                
                // Record Absence
                let absence = AbsenceData {
                    intent_id: id,
                    creator: intent.creator.clone(),
                    deadline: intent.deadline,
                    slashed_amount: intent.collateral,
                };
                Absences::<T>::insert(id, absence);

                Self::deposit_event(Event::AbsenceCrystallized { 
                    intent_id: id, 
                    who: intent.creator, 
                    slashed: intent.collateral 
                });
            }

            Weight::from_ref_time(10_000)
        }
    }

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn declare_intent(origin: OriginFor<T>, description_hash: [u8; 32], deadline: u64, collateral: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

            // Check deadline
            let current_block = <frame_system::Pallet<T>>::block_number();
            let current_u64: u64 = current_block.try_into().unwrap_or(0);
            ensure!(deadline > current_u64, Error::<T>::DeadlineInPast);

            // Generate ID (Simulated)
            // In reality, hash(who, description, deadline, nonce)
            let intent_id = description_hash; // Simplified for POC

            ensure!(!Intents::<T>::contains_key(intent_id), Error::<T>::IntentAlreadyExists);

            // Lock Funds (TODO: Implement T::Currency::reserve)
            
            // Store
            let intent = IntentData {
                creator: who.clone(),
                description_hash,
                deadline,
                collateral,
            };
            Intents::<T>::insert(intent_id, intent);

			Self::deposit_event(Event::IntentDeclared { intent_id, who, deadline: deadline.try_into().unwrap_or_default() });
			Ok(())
		}
	}
}
