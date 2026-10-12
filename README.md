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

### Alias policy

Canonical grammar (shared with Blink clients): `^[a-z0-9-]{3,32}$` — lowercase ASCII letters, digits, and hyphens only.

### Destination policy

`create` and `update` accept only `http://` or `https://` destinations up to 2048 characters.

### Error codes

| Code | Name | Meaning |
|---|---|---|
| 1 | `InvalidAlias` | Alias fails length/charset policy |
| 2 | `InvalidDestination` | Destination fails scheme/length policy |
| 3 | `AliasTaken` | Alias already stored |
| 4 | `LinkNotFound` | Alias missing |
| 5 | `Unauthorized` | Authenticated address is not the stored owner |

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

