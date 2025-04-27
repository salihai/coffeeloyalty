# Coffee Loyalty

![output](assets/coffee_star.jpg)

Welcome to CoffeeLoyalty, a Stellar-based loyalty program for a coffee shop. Built on the Stellar Testnet, this Soroban smart contract rewards customers with 1 point per coffee purchase. After 10 points, customers earn a free coffee, redeemable through the contract. The contract also supports token management, account freezing, and admin controls, ensuring a secure and flexible loyalty system.

## About Me

I’m Saliha İpteş, a 2023 Computer Engineering graduate from Necmettin Erbakan University. Passionate about blockchain, I’m learning Soroban, Rust, and Stellar SDK through Stellar Bootcamp. With skills in data science and AI, I aim to build innovative decentralized applications that drive real-world impact. My journey in blockchain fuels my vision to create secure, transparent solutions for the future. Connect with me on [LinkedIn](https://shorturl.at/rAJA2) .

## Project Details

CoffeeLoyalty is a Stellar-based loyalty program for coffee shops, built on the Stellar Testnet using a Soroban smart contract. Customers earn 1 point for each coffee purchased, tracked securely on the blockchain. After collecting 10 points, they receive a free coffee voucher, redeemable through the contract. The system supports token management (mint, burn, transfer), allowance mechanisms for third-party spending, and admin controls for secure operations. Accounts can be frozen to prevent unauthorized actions. With a transparent and decentralized approach, CoffeeLoyalty ensures trust and efficiency, rewarding loyal customers while leveraging Stellar’s fast, low-cost transactions. The contract, written in Rust, offers a scalable solution for modernizing retail loyalty programs.

## Vision

CoffeeLoyalty envisions a future where blockchain transforms customer loyalty programs. By using Stellar’s fast, low-cost transactions and Soroban smart contracts, it creates a transparent, secure, and decentralized system for coffee shops to reward customers. This project promotes trust, eliminates intermediaries, and ensures fair point tracking and redemption. Its impact extends beyond coffee shops, inspiring retailers to adopt blockchain for loyalty systems, enhancing customer engagement, and fostering financial inclusion. CoffeeLoyalty aims to set a standard for decentralized rewards, making loyalty programs more efficient, accessible, and trustworthy, ultimately reshaping how businesses connect with customers globally.

## Software Development Plan

1. Smart Contract Design:

   - Functions: mint, burn, transfer, add_coffee_point, check_free_coffee, redeem_free_coffee, freeze_account, unfreeze_account.
   - Variables: Balance(Address), CoffeePoints(Address), FreeCoffee(Address), Frozen(Address).
   - Features: Token management, loyalty program, account freezing.

2. Contract Development:

   - Write contract in Rust using Soroban SDK.
   - Implement admin-only controls and error handling.

3. Testing:

   - Deploy to Stellar Testnet.
   - Test token operations, point accumulation, and free coffee redemption.

4. Front-End (Optional):

   - Develop a JavaScript interface for customers to view points and balances.

5. Documentation:

   - Create Readme.md with setup and usage instructions.

6. Deployment:
   - Deploy contract to Testnet and share contract ID and transaction hashes.

## Features

- Token Management: Mint, burn, and transfer operations for the CoffeeToken (COFFEE).
- Loyalty Program: Earn 1 point per coffee purchase; 10 points grant a free coffee.
- Authorization: Allowance mechanism for third-party spending of tokens.
- Admin Control: Setting admin and operations requiring administrator permission (e.g., minting, freezing accounts, managing points).
- Account Freezing: Freeze or unfreeze accounts to restrict token operations.
- Metadata: Token name ("CoffeeToken"), symbol ("COFFEE"), and decimal information (0 decimals).

## Contract Structure

The project consists of the following modules:

- admin: Administrator functions and authorization (e.g., set_admin, initialize).
- allowance: Token spending permission management (e.g., approve, transfer_from).
- balance: Balance management operations (e.g., balance, transfer).
- coffee: Loyalty program operations (e.g., add_coffee_point, check_free_coffee).
- contract: Main contract implementation and token interface.
- metadata: Token metadata (name, symbol, decimals).
- storage_types: Storage data structures (e.g., CoffeePoints, FreeCoffee).

## Technical Details

The contract is written in Rust using the Soroban SDK and compiled without standard library dependencies via the #![no_std] directive. This ensures a smaller contract size and efficient execution on Stellar’s Testnet. The contract uses persistent storage to track token balances, coffee points, free coffee vouchers, and frozen account statuses, with TTL extensions for data durability.

### Key Functions

Token Management

- mint: Create new tokens (admin only).
- burn: Burn tokens to remove them from circulation.
- transfer: Transfer tokens between addresses.
- balance: View the token balance of an address.

Allowance Management

- approve: Grant spending permission to another address.
- allowance: View the granted permission amount.
- transfer_from: Transfer tokens using granted permission.
- burn_from: Burn tokens using granted permission.

Admin Operations

- initialize: Initialize the contract with admin, name, symbol, and decimals.
- set_admin: Change the administrator address.
- freeze_account: Freeze an account to prevent token transfers or burns (admin only).
- unfreeze_account: Unfreeze an account to restore token operations (admin only).

Loyalty Program

- add_coffee_point: Add 1 point to a customer’s account for a coffee purchase (admin only).
- check_free_coffee: Check if a customer has 10 points and grant a free coffee voucher (admin only).
- redeem_free_coffee: Redeem a free coffee voucher (admin only).

## Installation and Setup

### Prerequisites

- Rust:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Soroban CLI:
  ```bash
  cargo install --locked soroban-cli
  ```
- Node.js:
  ```bash
  sudo apt install nodejs npm
  ```
- Stellar SDK:
  ```bash
  npm install stellar-sdk
  ```

### Steps

1. Clone the repository:

   ```bash
    git clone https://github.com/[YourUsername]/CoffeeLoyalty.git
    cd CoffeeLoyalty
   ```

2. Build the smart contract:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Configure Testnet:

   ```bash
   soroban config network add --global testnet \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase "Test SDF Network ; September 2015"
   ```

4. Deploy the contract:

   ```bash
   stellar contract deploy \
   --wasm target/wasm32-unknown-unknown/release/coffeeloyalty.wasm \
   --source coffeeadmin \
   --network testnet \
   --alias coffeetoken
   ```

5. Initialize the contract:
   ```bash
   stellar contract invoke \
   --id <CONTRACT_ID> \
   --source coffeeadmin \
   --network testnet \
   -- \
   initialize \
   --admin <ADMIN_ADDRESS> \
   --decimal 0 \
   --name "CoffeeToken" \
   --symbol "COFFEE"
   ```

## Testnet Details

- _Contract ID_: [Your Contract ID, e.g., CBXXY33COLFCIEH2DJBKVSD456CFE6F3PMJRP2WX3TUX3CSIJMNVCMTF]
- _Transaction Hash_: [Example Transaction Hash: 4d7ffb09aba979673a6a3af4bf827570c65e9c7e1e4ed99f2b20b3bcb4f5a04a]
- _Testnet Explorer_: Stellar Laboratory

## Example Usage

- Add a coffee point:

  ```bash
  stellar contract invoke \
  --id <CONTRACT_ID> \
  --source coffeeadmin \
  --network testnet \
  -- \
  add_coffee_point \
  --account <USER_ADDRESS>
  ```

- Grant a free coffee (after 10 points):

  ```bash
  stellar contract invoke \
  --id <CONTRACT_ID> \
  --source coffeeadmin \
  --network testnet \
  -- \
  check_free_coffee \
  --account <USER_ADDRESS>
  ```

- Redeem a free coffee:

  ```bash
  stellar contract invoke \
  --id <CONTRACT_ID> \
  --source coffeeadmin \
  --network testnet \
  -- \
  redeem_free_coffee \
  --account <USER_ADDRESS>
  ```

- Freeze an account:
  ```bash
  stellar contract invoke \
  --id <CONTRACT_ID> \
  --source coffeeadmin \
  --network testnet \
  -- \
  freeze_account \
  --account <USER_ADDRESS>
  ```

License

MIT License
