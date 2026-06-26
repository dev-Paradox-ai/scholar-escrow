# ScholarEscrow

> Milestone-gated USDC grant disbursement for students and NGOs on Stellar.

---

## The Idea in Plain English

An NGO locks scholarship money into a smart contract. The money only releases
to the student when they complete a milestone AND the NGO confirms it on-chain.
No bank. No waiting weeks. No paperwork.

---

## Problem
Filipino college students receiving NGO scholarships wait 6–8 weeks for manual
bank disbursements, forcing them into debt or dropping subjects.

## Solution
ScholarEscrow locks USDC in a Soroban escrow contract tied to student milestones.
Funds release instantly upon NGO approval — no bank, no delay.

---

## Stellar Features Used
- Soroban smart contracts (escrow logic)
- USDC transfers (stable disbursement)
- Trustlines (student wallet setup)
- XLM (near-zero gas fees)

---

## Vision
Replace paper-based NGO grant workflows across Southeast Asia with transparent,
auditable, instant on-chain disbursements that both students and donors can trust.

---

## Prerequisites
- Rust (stable, 1.74+): https://rustup.rs
- Soroban CLI v20+: cargo install --locked soroban-cli
- A Stellar testnet account (free at https://laboratory.stellar.org)

---

## How to Build

```bash
soroban contract build
```

Output: target/wasm32-unknown-unknown/release/scholar_escrow.wasm

---

## How to Test

```bash
cargo test
```

---

## How to Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/scholar_escrow.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet
```

---

## Sample Commands

### Step 1 — NGO initializes escrow with 2 milestones (300 + 700 USDC)
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source NGO_SECRET \
  --network testnet \
  -- initialize \
  --ngo NGO_ADDRESS \
  --student STUDENT_ADDRESS \
  --token USDC_CONTRACT_ADDRESS \
  --milestone_amounts '[300, 700]'
```

### Step 2 — Student submits milestone 0
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source STUDENT_SECRET \
  --network testnet \
  -- submit_milestone \
  --student STUDENT_ADDRESS \
  --milestone_index 0
```

### Step 3 — NGO approves milestone 0 (releases 300 USDC instantly)
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source NGO_SECRET \
  --network testnet \
  -- approve_milestone \
  --ngo NGO_ADDRESS \
  --milestone_index 0
```

---

## License
MIT

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CD77QB3LOH2NPPY2KI7FGZYLLVALUSPHG6HKXE7CGRFDXXU7AVF2QTTT` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CD77QB3LOH2NPPY2KI7FGZYLLVALUSPHG6HKXE7CGRFDXXU7AVF2QTTT) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/17ae708654a13c0b2892c6550e523aed9ce872b41c01fef1cf0f78acdc3e3a2c) |
| Deployed | 2026-06-26 06:53:22 UTC |
| Wallet | freighter (`GBP5…5UFC`) |
