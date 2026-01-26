# Invariant Mapping: Phoenix Kernel v0.1

**Target System**: `tasm-core` (Rust Crate)
**Canon Version**: v1.0
**Status**: Self-Verified

This document maps the implementation logic in `tasm-kernel` to the 6 Conformance Criteria of the TASM Canon.

---

## 1. Time-Driven Evolution
**Canon**: *State changes only as a function of time progression.*

*   **Implementation**: `AbsenceEngine::advance_time(new_slot)`
*   **File**: `src/engine.rs`
*   **Logic**:
    *   The *only* public method that mutates state (crystallizes absences) is `advance_time`.
    *   No external "admin" key can invoke `resolve_intent`.
    *   Resolution is triggered strictly by `new_slot > current_slot`.

**Status**: ✅ COMPLIANT

## 2. Absence Irreversibility
**Canon**: *Once an absence is true, it cannot become false.*

*   **Implementation**: `MemoryRegistry::mark_absence`
*   **File**: `src/engine.rs`
*   **Logic**:
    ```rust
    self.intents.remove(intent_id);
    ```
    *   The Intent is physically deleted from the `active` set.
    *   It serves as the "source" for potential fulfillment. Once removed, it is impossible for `prove_fulfillment` (hypothetical) to find the intent.
    *   The Absence event is emitted and the collateral is slashed immediately.

**Status**: ✅ COMPLIANT

## 3. No Late Action
**Canon**: *Observations after deadlines are impossible.*

*   **Implementation**: `AbsenceEngine::advance_time` loop
*   **File**: `src/engine.rs`
*   **Logic**:
    ```rust
    if intent.deadline < self.current_slot { ... }
    ```
    *   If the clock ticks past the deadline, the intent is processed *before* any new user action can theoretically be ingested in that slot (in a block context).
    *   The check is strict strict inequality (`<`).

**Status**: ✅ COMPLIANT

## 4. Monotonic State
**Canon**: *State(t1) ⊆ State(t2)*

*   **Implementation**: `current_slot` progression
*   **File**: `src/engine.rs`
*   **Logic**:
    ```rust
    if new_slot <= self.current_slot { return vec![]; }
    ```
    *   Time can only move forward.
    *   Absences accumulate (are emitted). They are never "un-emitted".

**Status**: ✅ COMPLIANT

## 5. Actor Independence
**Canon**: *Correctness does not depend on liveness or participation.*

*   **Implementation**: Pure Functionality
*   **File**: `src/engine.rs`
*   **Logic**:
    *   The `advance_time` function requires no arguments other than the Slot number.
    *   It does not require signatures, "keepers", or external API calls.
    *   If the network halts for 100 years and restarts, `advance_time(now)` will instantly crystallize 100 years of absences in one atomic step.

**Status**: ✅ COMPLIANT

## 6. No Execution Semantics
**Canon**: *The kernel does not execute actions.*

*   **Implementation**: `Intent` struct
*   **File**: `src/model.rs`
*   **Logic**:
    *   The `Intent` struct contains `description: String`. It does not contain code, bytecode, or syscalls.
    *   The Kernel "watches" (by time), it does not "do".
    *   Slashing is an *internal* state transition, not an external execution.

**Status**: ✅ COMPLIANT

---

## Conclusion
The **Phoenix Kernel (`tasm-kernel`)** strictly adheres to the TASM v1 Canon.
ready for "Black Swan" stress test.
