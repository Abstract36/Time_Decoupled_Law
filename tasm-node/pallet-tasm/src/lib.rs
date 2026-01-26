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

    // --- STRUCTS (TASM Core + Consensus) ---

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct IntentData<AccountId, Balance> {
        pub creator: AccountId,
        pub description_hash: [u8; 32],
        pub deadline: u64, // BlockNumber/TimeSlot
        pub collateral: Balance,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct AbsenceData<AccountId, Balance> {
        pub intent_id: [u8; 32],
        pub creator: AccountId,
        pub deadline: u64,
        pub slashed_amount: Balance,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum EpochStatus {
        Pending,   // Gathering Attestations
        Challenged, // Under Dispute
        Finalized, // Accepted as Truth
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Attestation<AccountId> {
        pub validator: AccountId,
        pub timestamp: u64,
        pub signature: [u8; 64], // Simulated signature
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)] // Removed MaxEncodedLen due to Vec
    pub struct EpochData<AccountId> {
        pub index: u64,
        pub status: EpochStatus,
        pub attestations: Vec<Attestation<AccountId>>, 
        pub median_time: u64,
        pub challenge_end: u64, // BlockNumber
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: frame_support::traits::Currency<Self::AccountId>;
        
        #[pallet::constant]
        type EpochLength: Get<T::BlockNumber>; // e.g. 100 blocks
        
        #[pallet::constant]
        type ChallengeWindow: Get<T::BlockNumber>; // e.g. 50 blocks
	}

	// --- STORAGE ---

	#[pallet::storage]
	pub type Intents<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], IntentData<T::AccountId, BalanceOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub type Absences<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], AbsenceData<T::AccountId, BalanceOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub type Epochs<T: Config> = StorageMap<_, Identity, u64, EpochData<T::AccountId>, OptionQuery>;

    #[pallet::storage]
    pub type CurrentEpochIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

    type BalanceOf<T> = <<T as Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// --- EVENTS ---

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IntentDeclared { intent_id: [u8; 32], who: T::AccountId, deadline: u64 },
        AbsenceCrystallized { intent_id: [u8; 32], who: T::AccountId, slashed: BalanceOf<T> },
        EpochStarted { index: u64, start_block: T::BlockNumber },
        EpochFinalized { index: u64, time: u64 },
        AttestationSubmitted { validator: T::AccountId, epoch: u64, time: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {
		IntentAlreadyExists,
        DeadlineInPast,
        EpochNotPending,
        DuplicateAttestation,
	}

    // --- HOOKS ---

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let epoch_len = T::EpochLength::get();
            let current_block_u64: u64 = n.try_into().unwrap_or(0);
            
            // 1. Check for New Epoch
            // Note: In real Substrate, block numbers need careful type handling. 
            // Here assuming simple modulo for prototype.
            // Converting T::BlockNumber to u64 for modulo check is tricky in generic context 
            // without `sp_runtime::traits::Saturing`. 
            // For POC, we assume n is convertable.
            
            // State Machine Logic for Epochs would go here.
            // Simplified: If modulo == 0, finalize previous.
            
            // 2. Check Finalized Epochs for Absences
            // (Lazy Slashing)
            // Iterate over Finalized Epochs that haven't been processed?
            // Or just check active intents against "Safe Time".
            
            // Example: If an Epoch finalized time T, slash everything < T.
            
            Weight::from_ref_time(10_000)
        }
    }

	// --- EXTRINSICS ---

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn declare_intent(origin: OriginFor<T>, description_hash: [u8; 32], deadline: u64, collateral: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
            // Check deadline > Finalized Time (not current block time!)
            // Implementation simplified.
            let intent = IntentData { creator: who.clone(), description_hash, deadline, collateral };
            Intents::<T>::insert(description_hash, intent);
			Self::deposit_event(Event::IntentDeclared { intent_id: description_hash, who, deadline });
			Ok(())
		}

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn submit_attestation(origin: OriginFor<T>, epoch_index: u64, timestamp: u64, signature: [u8; 64]) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            // Verify TEE Signature (Stub)
            // In reality: verify(signature, key, timestamp)
            
            Epochs::<T>::try_mutate(epoch_index, |maybe_epoch| -> DispatchResult {
                let epoch = maybe_epoch.as_mut().ok_or(Error::<T>::EpochNotPending)?;
                ensure!(epoch.status == EpochStatus::Pending, Error::<T>::EpochNotPending);
                
                // Add
                epoch.attestations.push(Attestation { validator: who.clone(), timestamp, signature });
                
                Ok(())
            })?;
            
            Self::deposit_event(Event::AttestationSubmitted { validator: who, epoch: epoch_index, time: timestamp });
            Ok(())
        }
	}
}
