# Remittance-Service
# 💸 Remittance Service on Stellar (Soroban Smart Contract)

## 📌 Project Description

This project is a basic remittance service built using the Soroban smart contract platform on the Stellar blockchain. It enables users to securely send and receive funds across borders in a decentralized manner without relying on traditional intermediaries such as banks or money transfer services.

The smart contract ensures that only authorized users can send and claim remittances, providing transparency, security, and efficiency.

---

## 🚀 What It Does

This smart contract allows:

- A sender to initiate a remittance transaction
- A receiver to securely claim the remitted amount
- Anyone to verify if a remittance exists between two parties

The contract acts as a simple escrow system where funds are recorded and released upon claim.

---

## ✨ Features

### 🔐 Secure Transactions
- Uses Stellar’s Soroban authentication (`require_auth`) to ensure only authorized users can perform actions

### 🌍 Cross-Border Remittance
- Enables global money transfers without intermediaries

### 📦 Persistent Storage
- Stores remittance data securely on-chain

### 🔍 Transparency
- Users can check remittance status anytime

### ⚡ Fast & Low Cost
- Built on Stellar’s efficient and low-fee network

### 🔄 One-Time Claim
- Once claimed, the remittance is removed from storage to prevent double spending

---

## 🧠 How It Works

1. **Send Remittance**
   - Sender calls `send_remittance`
   - Stores amount mapped to sender + receiver

2. **Check Remittance**
   - Anyone can call `check_remittance` to view pending funds

3. **Claim Remittance**
   - Receiver calls `claim_remittance`
   - Funds are released and removed from storage

---

## 🛠️ Tech Stack

- **Blockchain:** Stellar
- **Smart Contracts:** Soroban
- **Language:** Rust

---

## 🔗 Deployed Smart Contract Link

> ⚠️ Replace this with your actual deployed contract link:

