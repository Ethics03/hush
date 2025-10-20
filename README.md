# Hush

A secure, encrypted password manager built with Rust.


![Hush Demo](public/hushdemo.gif)


## Features

- **Military-grade encryption** - ChaCha20-Poly1305 AEAD
- **Secure key derivation** - Argon2 password hashing
- **Multiple vaults** - Organize secrets by project
- **Clipboard integration** - Auto-copy secrets (never visible on screen)
- **Zero dependencies runtime** - Standalone binary

## Installation



```bash
git clone https://github.com/Ethics03/hush.git
cd hush
cargo build --release
sudo cp target/release/hush /usr/local/bin/
```
### Requires Rust ≥1.70 and Cargo.


## Quick Start


Create a new vault

```bash
hush create work
```


Add a secret (prompts for hidden value)

```bash
hush set --vault work --key github_token
```


Get a secret (copies to clipboard)

```bash
hush get --vault work --key github_token

```

List all vaults

```bash
hush list
```

Delete a secret

```bash
hush delete --vault work --key github_token
```


Delete an entire vault

```bash
hush delete-vault work
```


## Security

- Encryption: ChaCha20-Poly1305 (used by Google, TLS 1.3)

- Key Derivation: Argon2 (winner of the Password Hashing Competition)

- Salt & Nonce: Unique per vault and encryption operation

- No Plaintext Storage: Secrets are stored only in encrypted form at ~/.vaults/

- Without your master password, vault files are mathematically impossible to decrypt.


## Storage Structure

Vaults are stored in your home directory:

```bash
~/.vaults/
├── work/
│   ├── vault.json       # Metadata (salt, nonce)
│   └── secrets.enc      # Encrypted secrets
└── personal/
    ├── vault.json
    └── secrets.enc
```


## License

MIT License
Free to use, modify, and distribute.

<p align="center"> Built with ❤️ and 🦀 Rust </p>
