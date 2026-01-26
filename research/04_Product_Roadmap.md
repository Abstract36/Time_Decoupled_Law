# Time-Absence Protocol (TAP) — Product Roadmap

## Complexity Scale: From Concept to Product
The question of "first product ready" is about abstraction layers.

| Level | Description | Work Volume | Output |
| :--- | :--- | :--- | :--- |
| **1. Formal Specification** | Mathematical model & proofs (TASM). | 50-150 pages | Abstract, logically flawless model. |
| **2. Kernel** | Working implementation of the model (Rust/Python) without network. | 1000-2000 LOC | Library/Engine simulating the system. |
| **3. Network Prototype Node** | Distributed system with P2P, Consensus (e.g., Tendermint-BFT) and RPC. | 10k-20k LOC | First 3-5 test nodes. Product-ready for internal tests. |
| **4. Economically Full Node** | Added value layer: Token standard, Collateral, Slashing. | +5k-10k LOC | Public Testnet ready. |
| **5. End-User Product** | Wallet, Explorer, dApp interface. | 5k+ frontend LOC | Product-ready for niche users. |

## Strategic Paths
1.  **Researcher Path (3-6 months)**
    *   *Goal*: Prove viability.
    *   *Product*: Isolated Kernel + Whitepaper.
    *   *Node*: Single machine simulation.
2.  **Protocol Path (1-2 years)**
    *   *Goal*: Live minimal testnet.
    *   *Product*: P2P Node + 3-5 validators.
    *   *Value*: Demonstrating "Time -> Absence" principle in distributed environment.
3.  **Application Path (Fastest to Market)**
    *   *Goal*: Use existing L1 (Solana) to implement logic.
    *   *Product*: Smart Contract (TDSM) + Web Interface.
    *   *Volume*: ~1k LOC contract + ~1k LOC frontend.

## Roadmap: Time-Absence Protocol (TAP)

### Phase 0: Phoenix (MVP)
*   **Timeline**: 6-9 Months
*   **Goal**: Prove Kernel viability.
*   **Deliverables**:
    *   Isolated Kernel (Rust).
    *   **Atomic Vault** Smart Contract on Solana (Stub implementation).
    *   Formal Spec v1.

### Phase 1: Chronos (Testnet)
*   **Timeline**: 12-18 Months
*   **Goal**: Autonomous Blockchain.
*   **Deliverables**:
    *   P2P TAP Node.
    *   Public Testnet (5-10 validators).
    *   Consensus design centered on Time implementation.

### Phase 2: Aion (Mainnet)
*   **Timeline**: 24+ Months
*   **Goal**: Sustainable Public Protocol.
*   **Deliverables**:
    *   Stable Mainnet.
    *   DAO for parameter management (if meant to be governed).
    *   Ecosystem of apps (Lending, Insurance).

## Practical Plan (First 3 Months - Protocol Path)
1.  **Month 1**: Deep dive into TASM specs. Detailed Technical Design Doc (20-30 pages). Data schema.
2.  **Month 2**: Development of Isolated Kernel (Rust). Unit tests against spec.
3.  **Month 3**: Basic P2P network (libp2p) and state sync. First 2 nodes exchanging messages.

## Conclusion
To get "first nodes formed", the path ranges from strict math to 15k+ lines of network code.
**Recommendation**: The **Application Path** (Solana Contract) is the most efficient way to validate the *economic* thesis before building the *infrastructure*.
