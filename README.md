# 🧮 Solana Calculator (Anchor Example)

A simple **Solana smart contract** built using the **Anchor framework** — created to understand the basics of Solana accounts, serialization, and program execution.

---

## 🚀 Overview
This program demonstrates how to:
- Initialize a new **on-chain account**
- Store and update data in that account
- Work with **mutable accounts**, **signers**, and the **system program**

---

## 🧱 Account Structure
```rust
#[account]
pub struct accountData {
    value: u32,
    Owner: Pubkey,
}
