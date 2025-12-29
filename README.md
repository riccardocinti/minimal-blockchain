# Minimal Blockchain – Learning-Oriented Specification

## 1. Purpose

This project defines a **minimal blockchain** designed purely for **hands-on exploration of core blockchain concepts**.

The goal is understanding, not production readiness.

You should be able to:
- Read all data structures
- Reason about every rule
- Inspect and modify behavior easily

---

## 2. Goals

The blockchain must demonstrate:

- Immutability via hash chaining
- Ordered transaction history
- Merkle trees
- Block validation
- Forks and reorganization
- Eventual consistency
- State derived from history

---

## 3. Non-Goals

This blockchain explicitly does **not** aim to:

- Be secure against adversaries
- Be decentralized over a real network
- Support smart contracts
- Handle high throughput
- Provide economic incentives
- Be compatible with existing chains

---

## 4. Core Concepts Implemented

| Concept            | Included |
|--------------------|----------|
| Hashing            | Yes      |
| Merkle Trees       | Yes      |
| Proof of Work      | Toy only |
| Forks              | Yes      |
| Finality           | Probabilistic |
| State Replay       | Yes      |
| Networking         | No       |

---

## 5. Data Structures

### 5.1 Transaction

A transaction is an opaque payload with deterministic hashing.


Rules:
- `tx_id = SHA256(payload)`
- No signatures
- No balances or accounts in MVP

---

### 5.2 Block


Rules:
- `block_hash = SHA256(all other fields)`
- Blocks are immutable once created
- Genesis block is hardcoded

---

### 5.3 Blockchain


Rules:
- Ordered by height
- Only one canonical chain at a time
- Forks may exist temporarily

---

## 6. Hashing & Merkle Tree

### 6.1 Hash Function

- SHA-256
- Used for:
    - Transaction IDs
    - Merkle tree nodes
    - Block hashes

---

### 6.2 Merkle Tree Rules

- Leaf hash: `SHA256(tx_id)`
- Binary Merkle tree
- If odd number of nodes → duplicate last node
- Merkle root stored in block header

Purpose:
- Efficient inclusion proofs
- Tamper detection

---

## 7. Block Creation (Mining)

Block creation is local and sequential.

### Steps

1. Collect pending transactions
2. Build Merkle tree
3. Construct block header
4. Increment nonce until difficulty rule is satisfied
5. Append block to chain

---

## 8. Proof-of-Work (Toy)

### Difficulty Rule


- N is small (e.g. 3–5)
- Difficulty is static
- Exists only to demonstrate:
    - Cost of immutability
    - Block creation delay
    - Fork probability

---

## 9. Validation Rules

A block is valid if:

1. `previous_hash` matches parent block hash
2. `height = parent.height + 1`
3. Merkle root matches transactions
4. Block hash is correctly computed
5. Difficulty rule is satisfied
6. Transactions are deterministic

Invalid blocks are rejected.

---

## 10. State Model

### Definition

State is derived by replaying all transactions from genesis.


Properties:
- No stored state snapshots
- Full replay on startup
- Deterministic application required

This highlights the difference between **history** and **state**.

---

## 11. Forks & Chain Selection

Forks occur when multiple blocks reference the same parent.

### Chain Selection Rule

- Canonical chain = longest valid chain
- If equal length:
    - Choose chain with lowest tip block hash (deterministic tie-break)

### Reorganization

- Roll back blocks to fork point
- Reapply transactions from new branch

---

## 12. Finality Model

- No absolute finality
- Blocks are probabilistically final


A block is considered “stable” after `K` confirmations.

---

## 13. Persistence

Minimum requirements:
- Blocks written to disk
- On startup:
    - Load blocks
    - Validate chain
    - Recompute state

File-based storage is sufficient.

---

## 14. Interface (CLI Suggested)

init-chain
add-tx <payload>
mine-block
print-chain
verify-chain


---

## 15. Metrics (Optional)

- Block time
- Transactions per block
- Fork count
- Reorg depth

---

## 16. Extension Paths

After MVP, possible extensions:

- Multiple nodes
- Gossip-based networking
- Account or UTXO model
- Transaction signatures
- Difficulty adjustment
- Light-client proofs

---

## 17. Summary

> A minimal blockchain that makes immutability, ordering, Merkle trees, proof-of-work, and fork resolution explicit and inspectable for learning purposes.
