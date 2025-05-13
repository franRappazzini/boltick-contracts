# 🎟️ Boltick Smart Contract

This repository contains the smart contract for **Boltick**, a platform designed to create and manage digital access credentials such as memberships and event tickets, powered by the Solana blockchain.

## 📌 Description

Boltick transforms digital access into unique, non-falsifiable credentials using NFTs as tickets or memberships. This enables:

- Automatic, serverless validation.
- Flexible benefit management based on user roles.
- Persistent, auditable participation history.

All with a user-friendly experience that requires zero blockchain knowledge from end users.

## ⚙️ Technologies

- [Solana](https://solana.com/)
- [Anchor Framework](https://book.anchor-lang.com/)
- [Rust](https://www.rust-lang.org)

## 📦 Project Structure

- `/programs/boltick`: Smart contract source code (Anchor program).
- `/tests`: Smart contract tests using Anchor.
- `Anchor.toml`: Anchor project configuration.
- `Cargo.toml`: Rust dependencies.

## Auditware Radar audit

<img src="https://img.shields.io/github/actions/workflow/status/franRappazzini/boltick-contracts/radar.yaml">

## 🚀 Deployment

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/en/download/) (for testing)

### Run locally

1. Build the program:

   ```bash
   anchor build
   ```

2. Set the Solana cluster to local:

   ```bash
   solana config set --url localhost
   ```

3. Start a local Solana cluster (using the Metaplex metadata program):

   ```bash
    solana-test-validator --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so --reset
   ```

4. Deploy to localnet:

   ```bash
   anchor deploy
   ```

5. Run tests (optional):

   ```bash
   anchor test --skip-local-validator
   ```

🧪 Testing

The tests are written in TypeScript and cover core features such as ticket minting, validation, and membership management.

📄 License

This project is licensed under the MIT License.

⸻

For more information about Boltick’s mission and ecosystem, check out the whitepaper.
