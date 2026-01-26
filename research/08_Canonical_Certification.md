# TASM Canon v1: Certification and Authority

**Classification**: Legal / Protocol Law
**Status**: Immutable
**Canon Version**: v1.0

---

## Part A: Canonical Certification Rules

### 1. Purpose
These rules define the process by which an external system may be certified as **Conformant** to the TASM Canon v1.
Certification establishes technical correspondence, not endorsement, quality, or suitability.

### 2. Scope of Certification
Certification applies exclusively to:
*   the core kernel
*   the state evolution logic
*   the absence derivation rules
*   the time semantics

Certification does not apply to:
*   UX
*   networking
*   governance
*   economics
*   compliance layers
*   deployment environments

### 3. Canon Definition
The Canon consists of the following immutable artifacts:
1.  Mathematical Model (TASM)
2.  Formal Specification
3.  Reference Kernel
4.  Formal Proofs (Lean / Coq)
5.  Canonical Hash / Timestamp

These artifacts are fixed and non-upgradable.

### 4. Eligibility
An implementation is eligible for certification if and only if:
*   it claims full correspondence to the Canon
*   it does not introduce additional state transitions
*   it does not weaken any invariant
*   it does not add discretionary overrides

Partial or “inspired by” implementations are ineligible.

### 5. Conformance Criteria
An implementation MUST satisfy all of the following:

1.  **Time-Driven Evolution**
    State changes only as a function of time progression.
2.  **Absence Irreversibility**
    Once an absence is true, it cannot become false.
3.  **No Late Action**
    Observations after deadlines are impossible.
4.  **Monotonic State**
    $State(t_1) \subseteq State(t_2)$ for all $t_2 \ge t_1$.
5.  **Actor Independence**
    Correctness does not depend on liveness or participation.
6.  **No Execution Semantics**
    The kernel does not execute actions.

**Violation of any single criterion results in non-conformance.**

### 6. Evaluation Process
The evaluation process is:
1.  Submission of:
    *   source code
    *   formal description
    *   invariant mapping
2.  Deterministic analysis against Canon artifacts
3.  Binary outcome:
    *   Conformant
    *   Non-Conformant

There are no conditional approvals.

### 7. Certification Outcome
If conformant, the following statement may be issued:
> “This system conforms to TASM Canon v1 as defined by the Canonical Specification.”

No other claims are implied or permitted.

### 8. Authority and Finality
*   Certification authority derives solely from the Canon.
*   Decisions are final.
*   There is no appeal process.
*   The Canon itself is not subject to certification.

### 9. Amendments
*   These rules may not alter the Canon.
*   If these rules conflict with the Canon, the Canon prevails.

### 10. Closure
Certification exists to preserve clarity, not adoption.
Conformance is optional.
Deviation is permitted but must not claim equivalence.

---

## Part B: Authoritative Origin Declaration

### 1. Declaration
This document declares the authoritative origin of the **Time–Absence State Machine (TASM) Canon v1**.
The Canon is defined as a closed formal artifact.

### 2. Scope of Authority
Authority applies to:
*   the mathematical definition of TASM
*   the formal specification
*   the reference kernel
*   the formal proofs
*   the canonical hash and timestamp

Authority does not imply control over implementations.

### 3. Canonical Status
The Canon:
*   is complete
*   is immutable
*   admits no upgrades
*   admits no extensions
*   admits no governance

Any system deviating from the Canon defines a different system.

### 4. Role of the Author
The author:
*   establishes the Canon
*   maintains its availability
*   verifies correspondence claims

The author does not:
*   certify themselves
*   modify the Canon
*   arbitrate interpretations
*   participate in implementations

### 5. Relation to Implementations
Implementations may:
*   reference the Canon
*   claim correspondence
*   request certification

Implementations may not:
*   alter Canon semantics
*   redefine invariants
*   claim equivalence if deviating

### 6. Finality
This declaration is final.
The Canon exists independently of:
*   adoption
*   usage
*   market demand
*   institutional acceptance

### 7. Closing Statement
The TASM Canon is not proposed. **It is declared.**
All correspondence flows from this point.
**The Canon is fixed at the moment of publication.**
