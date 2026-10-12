# Blink Contracts

Soroban smart contracts for **Blink** — wallet-owned short links on Stellar.

The primary contract, **LinkRegistry**, stores a unique alias with its owner, destination, activation flag, and timestamps. Ownership mutations require the stored owner’s authorization. Resolution is public and returns only **active** links.

| | |
| --- | --- |
| **Package** | `link-registry` |
| **SDK** | `soroban-sdk` 26.x |
| **Related** | [blink-frontend](https://github.com/blink-protocol/blink-frontend) · [blink-backend](https://github.com/blink-protocol/blink-backend) |

> **Security:** This contract has **not** received an independent security audit. Do not treat it as production-safe on public mainnet without review.

---

## LinkRegistry

### Methods

| Method | Auth | Purpose |
| --- | --- | --- |
| `create(owner, alias, destination)` | `owner` | Claim a unique alias |
| `resolve(alias)` | public | Read an **active** link, or `None` |
| `update(owner, alias, destination)` | stored owner | Change destination and reactivate |
| `deactivate(owner, alias)` | stored owner | Set `active = false` (history retained) |

### Storage model

- Key: `DataKey::Link(alias)`
- Value: `Link { owner, destination, active, created_at, updated_at }`
- Persistent TTL is extended on write and on successful resolve reads (`EXTEND_AT` / `EXTEND_TO`)

### Events

`LinkEvent` topics include action + owner; data includes alias.

| Action symbol (`symbol_short`) | Emitted by |
| --- | --- |
| `created` | `create` |
| `updated` | `update` |
| `disabled` | `deactivate` |

Integrators should treat these symbols as part of the public surface — do not rename lightly.

---

## Policies (canonical)

Clients (frontend + backend) **must** match these rules. Divergent client validation creates unresolvable on-chain state.

### Alias

```text
^[a-z0-9-]{3,32}$
```

- Length 3–32
- Lowercase ASCII `a-z`, digits `0-9`, hyphen `-` only
- Enforced in `validate_alias`

### Destination

- Non-empty, max **2048** characters
- Must start with `https://` or `http://`
- Shared by `create` and `update` via `validate_destination` (update cannot weaken create)

---

## Error codes

Stable `repr(u32)` discriminants — **append only**; do not renumber.

| Code | Name | Meaning |
| ---: | --- | --- |
| 1 | `InvalidAlias` | Alias fails length/charset policy |
| 2 | `InvalidDestination` | Destination fails scheme/length policy |
| 3 | `AliasTaken` | Alias already stored |
| 4 | `LinkNotFound` | Alias missing |
| 5 | `Unauthorized` | Authenticated address ≠ stored owner |

Wallet UIs should map these codes to actionable copy (switch wallet vs pick another alias vs fix URL).

---

## Repository layout

```text
Cargo.toml                       Workspace
contracts/link-registry/
  Cargo.toml
  src/lib.rs                     Contract implementation
  src/test.rs                    Unit tests
  test_snapshots/                Soroban test snapshots
```

---

## Prerequisites

- Rust toolchain (edition 2021)
- [`stellar` CLI](https://developers.stellar.org/docs/tools/cli) (build + deploy)
- Target: `wasm32v1-none` (or the current Soroban WASM target for your CLI version)

```bash
rustup target add wasm32v1-none
```

---

## Development

```bash
# from repo root
cargo test -p link-registry
```

Current coverage includes:

- Happy-path create → resolve → update → deactivate
- Invalid alias / destination rejection (charset + scheme)
- Wrong-owner update/deactivate → `Unauthorized`

```bash
stellar contract build
```

Artifact (typical path):

```text
target/wasm32v1-none/release/link_registry.wasm
```

Confirm the exact path with your CLI/toolchain output.

---

## Deploy (testnet)

1. Build the WASM release artifact.
2. Fund a deployer identity on testnet.
3. Deploy and record the contract id for frontend/backend env.

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/link_registry.wasm \
  --source <deployer-identity> \
  --network testnet
```

Then set:

- Frontend: `NEXT_PUBLIC_CONTRACT_ID`
- Backend: `CONTRACT_ID`
- Matching `STELLAR_NETWORK` / `NEXT_PUBLIC_STELLAR_NETWORK` + RPC URLs

### Public network

Same flow with `--network public` (or your CLI’s public network profile). Double-check passphrase alignment in both clients before announcing a public alias namespace.

---

## Client integration

### Create (wallet)

1. Validate alias/destination locally (same grammar as above).
2. Simulate `create` with the owner address requiring auth.
3. Sign with Freighter (or another adapter) on the **same** network passphrase.
4. Submit and wait for SUCCESS before showing a receipt.

### Resolve (read API / redirect)

1. Prefer [blink-backend](https://github.com/blink-protocol/blink-backend) `GET /v1/links/:alias`.
2. Treat inactive/missing as not found.
3. Never redirect to non-http(s) destinations even if older chain data were hostile — backend returns `422 unsafe_destination`.

### Update / deactivate

Require the **stored** owner. Mismatch returns `Unauthorized` (code 5), not `LinkNotFound`.

---

## Threat model (summary)

| Risk | Mitigation in contract |
| --- | --- |
| Alias squatting without auth | `owner.require_auth()` on create |
| Unauthorized destination change | Owner check + `Unauthorized` |
| Non-web redirect schemes | http(s) prefix validation |
| Client/contract policy drift | Documented canonical grammar + shared checks |
| TTL expiry / archival | Persistent extend on write/resolve |

Remaining risks (phishing destinations that are valid https URLs, RPC lying, UI footguns) need product/ops controls outside this crate.

---

## Contributing

1. Wait for maintainer assignment on Wave issues.
2. Keep behavior changes covered by `cargo test -p link-registry`.
3. If you add an error variant, **append** a new discriminant and update this README table.
4. PR must include `Closes #<n>` when resolving an issue.
5. Do not claim audit readiness in PR text unless an audit actually landed.

Issues: [github.com/blink-protocol/blink-contracts/issues](https://github.com/blink-protocol/blink-contracts/issues)

---

## Versioning & compatibility

- Error codes and event action symbols are part of the external contract surface.
- Breaking storage layout changes require a migration plan or a new contract id — never silently reshape `Link` in place on a live deployment without a version story.

---

## License & disclaimer

Provided as-is for Stellar Wave / ecosystem development. No warranty. Mainnet use without audit is at your own risk.
