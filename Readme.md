# 🚀 Solana AMM - Constant Product Automated Market Maker

A decentralized **Constant Product Automated Market Maker (AMM)** built using **Rust**, **Anchor Framework**, and the **Solana Blockchain**.

This project implements the core mechanics of decentralized exchanges such as **Uniswap V2**, enabling users to create liquidity pools, provide liquidity, swap assets, and withdraw liquidity while preserving the invariant:

> **x × y = k**

---

# 📌 Overview

Traditional exchanges require buyers and sellers to match orders.

An Automated Market Maker (AMM) eliminates the need for an order book by allowing users to trade directly against a liquidity pool.

Liquidity providers deposit two assets into the pool and receive LP Tokens representing their ownership share. Traders interact with the pool, paying a small fee that is distributed to liquidity providers.

---

# ✨ Features

* ✅ Initialize Liquidity Pool
* ✅ Add Liquidity
* ✅ Remove Liquidity
* ✅ Token Swaps
* ✅ LP Token Minting & Burning
* ✅ Constant Product Pricing
* ✅ Swap Fee Support
* ✅ Slippage Protection
* ✅ PDA-based Authorities
* ✅ Secure CPI Calls
* ✅ Unit & Integration Tests

---

# 🛠 Tech Stack

| Technology        | Purpose                      |
| ----------------- | ---------------------------- |
| Rust              | Smart Contract Development   |
| Anchor            | Solana Development Framework |
| Solana            | Blockchain Platform          |
| Anchor SPL        | Token Operations             |
| SPL Token Program | Token Transfers & Minting    |

---

# 📂 Project Structure

```text id="wczmjlwm"
programs/
└── amm/
    └── src/
        ├── lib.rs
        ├── constants.rs
        ├── error.rs
        ├── math/
        │   ├── mod.rs
        │   └── constant_product.rs
        ├── state/
        │   ├── mod.rs
        │   └── pool_state.rs
        └── instructions/
            ├── mod.rs
            ├── initialize_pool.rs
            ├── add_liquidity.rs
            ├── remove_liquidity.rs
            └── swap.rs
```

---

# 🏗 Architecture

```text id="jlq9iy3y"
                  User Wallet
                       │
                       ▼
                Solana Transaction
                       │
                       ▼
                 AMM Program (Anchor)
                       │
      ┌────────────────┼────────────────┐
      ▼                ▼                ▼
 PoolState PDA     Vault A         Vault B
   (Metadata)    (Token A)       (Token B)
                       │
                       ▼
                  LP Token Mint
```

---

# ⚙️ Program Instructions

## 1. Initialize Pool

Creates the complete liquidity pool infrastructure.

Creates:

* Pool State PDA
* Token Vault A
* Token Vault B
* LP Token Mint

---

## 2. Add Liquidity

Allows liquidity providers to deposit Token A and Token B into the pool.

The protocol:

* Transfers both assets into the vaults
* Calculates LP shares
* Mints LP Tokens
* Updates pool reserves

---

## 3. Swap

Allows users to exchange one token for another.

The protocol:

* Calculates output amount
* Applies swap fee
* Performs CPI token transfers
* Preserves the constant product invariant

---

## 4. Remove Liquidity

Allows liquidity providers to redeem their LP Tokens.

The protocol:

* Burns LP Tokens
* Calculates proportional withdrawal
* Transfers assets back to the user
* Updates reserves

---

# 📐 Constant Product Formula

The AMM uses the Constant Product Market Maker model:

```text id="3pmb7d5d"
x × y = k
```

Where:

* **x** = Reserve of Token A
* **y** = Reserve of Token B
* **k** = Constant product

Every swap preserves this invariant while charging a small trading fee.

---

# 🔒 Security Features

The implementation includes:

* Program Derived Addresses (PDAs)
* Canonical PDA Bumps
* Signer Verification
* Account Ownership Validation
* Checked Arithmetic
* Slippage Protection
* Authority Verification
* Anchor Account Constraints
* Safe Cross Program Invocations (CPIs)

---

# 🧪 Testing

Run all tests:

```bash id="3n0fpxd9"
anchor test
```

or

```bash id="kc9z6om9"
cargo test
```

The test suite covers:

* Pool Initialization
* Adding Liquidity
* Token Swaps
* Removing Liquidity
* Mathematical Invariants

---

# 🚀 Build & Deploy

Build the program:

```bash id="z4r1k04n"
anchor build
```

Deploy to Devnet:

```bash id="22msi4vl"
solana config set --url devnet

anchor deploy
```

---

# 📚 Key Concepts Demonstrated

This project demonstrates:

* Solana Account Model
* Program Derived Addresses (PDAs)
* Cross Program Invocation (CPI)
* SPL Token Program
* Anchor Framework
* Liquidity Pools
* LP Token Accounting
* Constant Product AMM
* On-chain State Management

---

# 🔮 Future Enhancements

Potential improvements include:

* Multiple Liquidity Pools
* Protocol Fee Collection
* Concentrated Liquidity
* TWAP Oracle
* Zero-Copy Accounts
* Property-Based Testing
* Compute Unit Optimization
* Multi-Hop Swaps

---

# 👨‍💻 Author

**Ardhendu Sekhar Mishra**

This project was developed as part of a learning journey into Solana smart contract development, decentralized finance (DeFi), and Automated Market Maker protocols using the Anchor Framework.
