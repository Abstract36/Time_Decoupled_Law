# Time Oracle: Hybrid Architecture (TEE + Guardians)

**Status**: Proposed Architecture (Phase 1)
**Problem**: How to inject physical time into the blockchain trustlessly?
**Solution**: A multi-layered hybrid model combining Hardware Root of Trust and Decentralized Verification, reinforced by a **Time Finality Gadget**.

---

## 🏗️ Architecture Overview

The system utilizes a **"Physical Anchor + Decentralized Verification"** approach. It acknowledges that perfect decentralization of *physical* time is impossible, so it relies on cryptographic hardware verification rooted in economic security.

### Level 1: The Physical Anchor (TEE Validators)
**Role**: Source of Truth (Cryptographic Anchor).
**Composition**: 30-40% of Validator Stake.
**Technology**: TEE-enabled hardware (Intel SGX, AMD SEV, Keystone RISC-V).

**Mechanism**:
*   **Attestation**: The TEE enclave signs a Time Certificate with a fused hardware key.
*   **Payload**: It validates "Current UTC Time" derived from the secure hardware clock.
*   **Property**: Authenticity is guaranteed by the manufacturer's root of trust, but correctness is verifiable by the network.

### Level 2: The Guardian Network (Decentralized Verification)
**Role**: Auditing and Consensus Filtering.
**Composition**: 60-70% of Validators (Standard Hardware).

**Mechanism**:
*   **Consensus Clock**: Guardians calculate the **median time** of all TEE certificates, discarding outliers (e.g., > 2s deviation).
*   **Anomaly Detection**: If a specific TEE node persistently deviates from the median or from external reality (NTP cross-checks), Guardians initiate a **Slashing Challenge**.
*   **Protection**: Prevents individual TEE failures from polluting the network time.

### Level 3: Slashing Consensus (Economic Defense)
**Role**: Dispute Resolution.
**Mechanism**:
1.  **Challenge**: Guardian stakes tokens against a TEE Certificate.
2.  **Peer Review**: A random committee of *other* TEE nodes determines correctness.
3.  **Outcome**: If the challenged node lied, it is **Slashed**. If the challenge was false, the Guardian is slashed.

---

## 🌟 The Critical component: Time Finality Gadget

**Problem**: What if >40% of TEE nodes collude to shift time simultaneously?
**Solution**: **Epoch-Based Finality**. We shift from "Real-Time" to "Judicial Time".

### Mechanism
1.  **Discrete Epochs**: Time does not flow continuously. It advances in epochs (e.g., 24 hours).
2.  **Retrospective Signing**: TEE nodes do not sign "It is now 12:00". They sign **"The Epoch 12:00-12:00 has started"**.
3.  **Verification Period**: This statement is not accepted immediately. It enters a **Challenge Window** (e.g., 1 Epoch).
4.  **Evidence of Lies**: During this window, any evidence that the epoch *had not* started (e.g., blocks from Bitcoin with older timestamps, NTP server signatures) can be submitted to prove the TEE lied.
5.  **Finality**: Only after the window closes without challenge does the Time Stamp become **Canonical**.

### Philosophy
We refuse "Perfect Real Time" in favor of a **"Judicial Protocol"** regarding the passage of time.
*   **Benefit**: This allows the system to survive massive TEE collusion, as long as *one* honest actor provides proof of reality during the challenge window.
*   **Fit**: Perfect for Abstract36, where "Absence" is determined post-factum anyway.

---

## 🛡️ Security Analysis (Attack Cost)
To successfully spoof time in this model, an attacker must:
1.  **Compromise Hardware**: Break TEE encryption on >30% of nodes.
2.  **Compromise Guardians**: Bribe >51% of Guardians to ignore the mismatch.
3.  **Survive Finality**: Suppress *all* external evidence of real time for the duration of the Challenge Window.

**Result**: An attack is not just expensive; it is information-theoretically practically impossible to sustain.

## 🎯 Implementation Strategy (Phase 1: Chronos)

### 1. Hardware Partnerships
*   Prioritize open-source RISC-V TEE implementations (Keystone).
*   Form a consortium of trusted initial validators (Universities, Labs).

### 2. Research Tasks
*   Formalize the "Time Finality Gadget" logic in Lean 4 (Track B).
*   Implement the slasher logic in the Substrate Pallet (Track A).
