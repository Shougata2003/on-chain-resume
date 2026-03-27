# 📄 On-Chain Resume — Soroban Smart Contract

> A decentralised, tamper-proof professional resume living entirely on the **Stellar blockchain**, powered by **Soroban** smart contracts.

---

## 📌 Project Description

On-Chain Resume lets any Stellar wallet holder publish, manage, and share their professional profile without relying on a centralised platform. Because every write is an authorised transaction signed by the owner's private key, the data is **self-sovereign** — no company can delete it, alter it, or gate access to it.

Peers and colleagues can endorse a resume directly on-chain, creating a publicly auditable trust layer that replaces closed recommendation systems.

---

## 🔍 What It Does

| Action | Who can call it | Description |
|---|---|---|
| `set_resume` | Owner | Publish or replace a full resume |
| `get_resume` | Anyone | Read any resume by Stellar address |
| `endorse` | Any wallet (not self) | Add one tamper-proof endorsement |
| `has_endorsed` | Anyone | Check if an address has endorsed |
| `update_skills` | Owner | Overwrite the skills list |
| `add_experience` | Owner | Append a work-experience entry |
| `add_education` | Owner | Append an education entry |
| `delete_resume` | Owner | Remove resume from storage |

All mutating calls require the owner's **signature** (`require_auth`), so no one can forge or edit another person's profile.

---

## ✨ Features

### 🔐 Self-Sovereign Identity
Your resume is tied to your Stellar wallet address. Only you — the keyholder — can write to it. No password resets, no account recovery via a third party.

### 🧱 Structured On-Chain Data
Stores rich, typed data directly on Stellar's ledger:
- **Bio & headline** — name, professional title, short bio
- **Skills** — updatable array of skill strings
- **Work Experience** — company, role, years, description
- **Education** — institution, degree, field, graduation year

### 👍 Peer Endorsements
Any wallet can endorse any other wallet's resume (except their own). Each endorsement is:
- **Idempotent** — one address can only endorse once
- **Auditable** — stored on-chain and queryable
- **Counted** — cumulative total tracked in the Resume struct

### 📡 On-Chain Events
Key actions emit Soroban events (`RESUME/SET`, `RESUME/DEL`, `ENDORSE/ADD`) that indexers and front-ends can subscribe to in real time.
<img width="1827" height="901" alt="image" src="https://github.com/user-attachments/assets/72462df7-ccd7-49dd-a61d-c1f60b62949a" />

### 🔄 Granular Updates
Owners don't need to resubmit their entire resume for small changes — dedicated functions let you append a new job, update skills, or add a degree independently.

### 🗑️ Right to Delete
Owners can permanently remove their resume from persistent storage at any time.

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)

```bash
rustup target add wasm32-unknown-unknown
```

### Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests
```bash
cargo test
```

### Deploy to Testnet
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/on_chain_resume.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```

### Deployed Contract ID
```
CD7LAGGRBACLP4N75H4TUQPHZKR7O5VPATWRRDIAWPV7FCKMH7WI6U4P
```
> Network: **Stellar Testnet**

### Invoke (example)
```bash
stellar contract invoke \
  --id CD7LAGGRBACLP4N75H4TUQPHZKR7O5VPATWRRDIAWPV7FCKMH7WI6U4P \
  --source <YOUR_SECRET_KEY> \
  --network testnet \
  -- get_resume \
  --owner <STELLAR_ADDRESS>
```

---

## 🗂️ Project Structure

```
on-chain-resume/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs          # Contract logic
```

---

## 🛣️ Roadmap

- [ ] Frontend dApp (React + Stellar Wallets Kit)
- [ ] Skill verification via third-party oracle
- [ ] Resume NFT mint (share as a unique token)
- [ ] Multi-sig endorsements for organisations

---

## 📜 License

MIT © 2026
