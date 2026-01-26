# How to Obtain Canonical Certification
**Operational Guide v1.0**

This document describes the practical steps for a developer or organization to obtain TASM v1 Conformance Certification for their implementation.

---

## Phase 1: Self-Verification (Prerequisites)

Before applying, you must verify your system against the **5 Pillars of TASM**:

1.  **Architecture Check**: Does your system have an "Executor"?
    *   *Yes* -> **STOP**. You are not eligible.
    *   *No* -> Proceed.
2.  **State Check**: Does your state change without user input (purely by time)?
    *   *Yes* -> Proceed.
    *   *No* -> **STOP**. You are building a standard ledger, not TASM.
3.  **Invariant Check**: Can an admin/DAO reverse an Absence?
    *   *Yes* -> **STOP**.
    *   *No* -> Proceed.

## Phase 2: Prepare the Evidence Package

You must prepare a repository containing:

### 1. Source Code Link
A fixed commit hash of the implementation (e.g., specific version of a standard Substrate pallet).

### 2. The Invariant Mapping Map
A document (`mapping.md`) linking your code functions to TASM Axioms.
*   *Example*:
    *   **Axiom**: `No Late Action`
    *   **Code**: `src/lib.rs: line 145 (fn validate_timestamp)`
    *   **Logic**: `ensure!(now <= deadline, Error::TooLate);`

### 3. Test Suite Logs
Output of a specific text scenario:
*   Scenario: "The Black Swan".
*   Action: Create Intent -> Stop Network for 1 Year -> Restart Network.
*   Result: System must immediately crystallize all Absences upon restart without manual intervention.

## Phase 3: The Submission Process

Currently, certification is managed by the **Abstract36 Genesis Maintainers**.

### Step 1: Open Certification Request
Submit a pull request (or issue) to the `Abstract36` repository with the tag `[CERTIFICATION]`.

**Template:**
```markdown
## Certification Request
**Project Name**: [Name]
**Repository**: [Link]
**Commit Hash**: [Hash]
**Claim**: "Strict Conformance to TASM Canon v1"

[ ] Attached Invariant Mapping
[ ] Attached Test Logs
[ ] Attached Architecture Diagram
```

### Step 2: Automated Analysis (Track B)
If you are using the Standard Rust Kernel (`tasm-core`), automated tools will verify that you haven't modified the core logic.

### Step 3: Manual Review (Track A)
For custom implementations (e.g., C++, Solidity), a manual code review is performed to ensure no "Backdoors" or "God Mode" keys exist that could violate Time Sovereignty.

## Phase 4: Attestation & Issuance

If successful, you will receive:

1.  **The Cryptographic Attestation**
    A PGP-signed message from the Canon Maintainer linking your specific `Commit Hash` to `TASM Canon v1`.
2.  **The Badge**
    Permission to display the "TASM Conformance" mark.
3.  **Registry Entry**
    Your project is added to `Abstract36/registry/certified_implementations.md`.

## Phase 5: Recertification
Certification is valid **only** for the specific Commit Hash.
*   If you upgrade the code -> **Certification is VOID**.
*   You must re-submit for the new Commit Hash including a diff analysis.

---

**Note**: Certification is *Zero-Knowledge* regarding your business logic. We certify the physics of time, not the quality of your product.
