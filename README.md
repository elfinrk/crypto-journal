# Stellar Crypto Journal 🚀

A decentralized trading journal application built on the **Stellar Soroban** network. This smart contract allows traders to securely log, manage, and audit their crypto trading strategies directly on the blockchain.

## 📝 Description
**Stellar Crypto Journal** is a Web3-based journaling tool designed for the modern trader. Unlike centralized note-taking apps, this application stores your trading history on the Stellar Testnet, ensuring that your data is immutable and accessible anywhere. It helps traders maintain discipline by recording the technical reasons behind every "BUY" or "SELL" decision.

## ✨ Key Features
- **Decentralized Storage:** No central database; all journal entries are stored as instance data on the Stellar ledger.
- **Trade Logging:** Record the specific token pair (e.g., BTC/USDT), the type of trade, and detailed strategy notes.
- **Unique Identification:** Every entry is assigned a unique 64-bit ID generated via the contract's Pseudo-Random Number Generator (PRNG).
- **History Management:** Easily retrieve all past journals or remove entries once they are no longer needed.

## 🛠 Tech Stack
- **Language:** Rust
- **Platform:** Soroban Smart Contracts (Stellar Network)
- **SDK:** `soroban-sdk`

## 🚀 Smart Contract Functions
| Function | Parameters | Description |
| :--- | :--- | :--- |
| `add_entry` | `token_pair`, `trade_type`, `notes` | Creates and saves a new trading journal entry. |
| `get_entries` | *None* | Returns a list of all saved journal entries. |
| `delete_entry` | `id` | Removes a specific entry from the ledger using its ID. |

## 🌐 Deployment Information
- **Network:** Stellar Testnet
- **Contract ID:** `CBBIMTLJRJARPB2JZL5ZLHPQSYFRHXSBF7R7NQHJ6K6OITYSUJ2EJSDP`

## 📥 How to Use
1. **Clone the repository:**
   ```bash
   git clone https://github.com/elfinrk/crypto-journal.git