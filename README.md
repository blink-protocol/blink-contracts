# Blink Contracts

Soroban contracts for wallet-owned short links on Stellar.

## LinkRegistry

The registry stores a unique alias with its owner, destination, activation status, and timestamps. Mutations require authorization from the stored owner, while resolution is public.

| Method | Authorization | Purpose |
|---|---|---|
| `create` | Owner | Claim a unique alias |
| `resolve` | Public | Read an active alias |
| `update` | Owner | Change and reactivate a destination |
| `deactivate` | Owner | Disable redirects for an alias |

## Development

```bash
cargo test -p link-registry
stellar contract build
```

## Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/link_registry.wasm \
  --source blink-deployer \
  --network testnet
```

This contract has not received an independent security audit. Do not use it in production without review.

