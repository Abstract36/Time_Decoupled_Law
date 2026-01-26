# TASM Substrate Pallet Design (v1.0)

**Status**: Draft Architecture
**Target**: Phase 1 (Chronos) - Networked Module
**Framework**: Substrate (Polkadot SDK)

## 1. Overview
The `pallet-tasm` acts as the bridge between the **TASM Kernel** (Pure Rust) and the **Substrate Runtime** (Blockchain).
It ensures that the "Time as Authority" philosophy is enforced at the consensus level.

## 2. Pallet Architecture

### 2.1 Dependencies
*   `tasm-core`: The isolated kernel crate (logic source).
*   `frame-support`: Substrate traits (Storage, Events).
*   `frame-system`: Block execution.

### 2.2 Storage Layout
Unlike standard pallets that store "active data", TASM stores "commitments" and "impossibilities".

```rust
#[pallet::storage]
/// Registry of active Intent Commitments.
/// Key: IntentId (SHA-256), Value: IntentData
pub type Intents<T: Config> = StorageMap<_, Blake2_128Concat, IntentId, Intent<T>, OptionQuery>;

#[pallet::storage]
/// Registry of crystallized Absences.
/// The "Graveyard" of broken promises.
pub type Absences<T: Config> = StorageMap<_, Blake2_128Concat, IntentId, Absence<T>, OptionQuery>;
```

### 2.3 Hooks (The Heartbeat)
The critical innovation is using the `on_initialize` hook to enforce Time logic **before** any user transaction.

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_initialize(n: BlockNumberFor<T>) -> Weight {
        // 1. Convert BlockNumber to TASM Slot
        let current_slot = Self::convert_to_slot(n);
        
        // 2. Feed time to the Absence Engine
        // (This triggers the "No Late Action" rule)
        let newly_dead = Self::engine_advance(current_slot);
        
        // 3. Crystallize Absences on-chain
        for absence in newly_dead {
            // Write to Storage
            Absences::<T>::insert(absence.id, absence);
            
            // Slash Collateral (Economic Consequence)
            T::Currency::slash(&absence.creator, absence.collateral);
            
            // Emit Event
            Self::deposit_event(Event::AbsenceCrystallized(absence.id));
        }
        
        Weight::from_ref_time(10_000)
    }
}
```

## 3. Extrinsics (User Actions)

The pallet exposes limited actions. You cannot "execute" an intent here, you can only "declare" it.

### 3.1 `declare_intent`
*   **Input**: `description_hash`, `deadline`, `collateral_amount`.
*   **Logic**:
    1.  Lock Collateral (Reserve balance).
    2.  Calculate SHA-256 Intent ID.
    3.  Store in `Intents`.
    4.  Emit `IntentDeclared`.

### 3.2 `prove_fulfillment` (Optional)
*   **Input**: `intent_id`, `proof_of_action`.
*   **Logic**:
    1.  Verify proof (e.g., check signature or external oracle data).
    2.  **Verify Time**: `current_block <= deadline`.
    3.  If valid:
        *   Unlock Collateral.
        *   Remove from `Intents`.
        *   Emit `IntentFulfilled`.
    4.  If invalid (too late):
        *   **Reject**. The `on_initialize` hook will eventually slash it.

## 4. Integration with Cosmos SDK (CosmWasm)
A similar logic applies to a Cosmos module:
*   **BeginBlocker**: Runs the `AbsenceEngine::advance_time`.
*   **Keeper**: Manages the `KVStore` for Intents.
*   **MsgDeclareIntent**: The transaction type.

## 5. Security Model
*   **Timestamp Manipulation**: Substrate's `pallet-timestamp` provides a median time. We rely on Block Number for deterministic strictness.
*   **Shutdown Loop**: If the chain halts, "Time" halts. This is acceptable for Phase 1. In Phase 2, we need an Oracle Time that continues even if blocks stop.

## 6. Next Steps
*   Create a new Substrate Node Template.
*   Implement `pallet-tasm` importing the logic from `tasm-core`.
