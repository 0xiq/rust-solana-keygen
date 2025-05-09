# keygen

A tiny Solana wallet generator in Rust.

## What it does

This script uses the `solana-sdk` to generate a new Solana `Keypair`, prints the public key, and outputs the private key in base58 format. 
## Usage

1. Clone or copy this project.

2. Build it:
```bash
    cargo build --release
```

3. Run it
```bash
    cargo run --release
```

You’ll see output like:
```bash
    Pubkey: 7FZ...
    Secret: 3mrYxW...
```

Use the public key to receive tokens, and store the secret securely.
