# Polkadot SDK → Pezkuwi SDK Rebranding Handoff

**Repository:** `/home/mamostehp/polkadot-sdk-fresh`
**Date:** 2025-11-26
**Status:** Clean repository verified and ready for rebranding

---

## Executive Summary

This document outlines the complete rebranding strategy for transforming the Polkadot SDK into the Pezkuwi SDK. The rebranding is structured in 6 phases to ensure systematic, safe, and reversible changes.

**Timeline:** Phased approach with validation gates between each phase
**Risk Mitigation:** Git commits after each phase, comprehensive testing before proceeding

---

## Phase 0: Discovery & Documentation

**Objective:** Identify all URLs, security keys, cryptographic parameters, and Polkadot-specific configurations that need to be preserved, replaced, or removed.

### 0.1 URL Discovery & Documentation

**Task:** Catalog all URLs in the codebase
- External URLs (documentation, block explorers, public endpoints)
- Internal URLs (localhost, testing endpoints)
- Repository URLs (git remotes, dependency sources)
- API endpoints (RPC, WebSocket, REST)

**Commands:**
```bash
# Find all URLs
grep -rn "https\?://" . --include="*.rs" --include="*.toml" --include="*.json" --include="*.md" > /tmp/urls-discovery.txt

# Find specific Polkadot URLs
grep -rn "polkadot\.network\|polkadot\.io\|parity\.io\|substrate\.io" . --include="*.rs" --include="*.toml" --include="*.json" --include="*.md" > /tmp/polkadot-urls.txt

# Find RPC/WS endpoints
grep -rn "wss\?://\|127\.0\.0\.1\|localhost" . --include="*.rs" --include="*.toml" --include="*.json" > /tmp/endpoints.txt
```

**Categorization:**
1. **Preserve:** External dependencies (GitHub, crates.io, official docs that must remain)
2. **Replace:** Branding URLs (polkadot.network → pezkuwi.network)
3. **Remove:** Zagros-specific URLs (if removing Zagros components)

**Deliverable:** `URL_INVENTORY.md` with categorized list

---

### 0.2 Cryptographic Parameters & Security Audit

**Task:** Document all cryptographic keys, genesis parameters, and security-critical constants

**Key Areas:**

#### Network IDs & Chain Specifications
```bash
# Find chain specs and genesis configs
find . -name "*.json" -path "*/chain-specs/*" -o -name "*genesis*.rs"

# Find network/protocol IDs
grep -rn "protocol_id\|chain_id\|network_id\|para_id" . --include="*.rs" --include="*.toml"
```

**Critical Parameters:**
- **Protocol ID:** Must be unique for Pezkuwi
- **Genesis Hash:** Will change after rebranding
- **SS58 Prefix:** Polkadot=0, Kusama=2, Pezkuwichain=42 → Define Pezkuwi prefix
- **Teyrchain IDs:** System teyrchains (1000, 1001, etc.)
- **Token Decimals:** HEZ=10, TYR=12, ZGR=12 → Define HEZ, ZGR, TYR decimals

#### Cryptographic Keys & Seeds
```bash
# Find hardcoded keys (DANGER: these must be changed!)
grep -rn "//Alice\|//Bob\|//Charlie\|0x[0-9a-fA-F]\{64\}" . --include="*.rs" --include="*.toml" | head -100

# Find seed phrases and mnemonics
grep -rn "seed\|mnemonic\|secret" . --include="*.rs" | grep -v "test\|example" | head -50
```

**Security Checklist:**
- [ ] Change all hardcoded development keys
- [ ] Replace bootnode keys
- [ ] Update session keys structure (if needed)
- [ ] Regenerate authority discovery keys
- [ ] Update telemetry endpoints
- [ ] Replace GRANDPA/BABE genesis authorities

#### Node Keys & Bootnodes
```bash
# Find bootnode configurations
grep -rn "bootnodes\|boot-nodes\|12D3KooW" . --include="*.rs" --include="*.toml" --include="*.json"

# Find node key references
grep -rn "node-key\|node_key\|NodeKey" . --include="*.rs"
```

**Action Items:**
- Generate new bootnode libp2p keys for Pezkuwi networks
- Document current Polkadot bootnode addresses (to preserve for dependencies)
- Create new bootnodes for PezkuwiChain, Zagros, TeyrChain

#### Runtime Version & Metadata
```bash
# Find runtime versions
grep -rn "spec_name\|impl_name\|spec_version\|impl_version" . --include="*.rs"

# Find runtime APIs
grep -rn "RuntimeApi\|runtime_apis" . --include="*.rs" | head -50
```

**Version Strategy:**
- Maintain compatibility with Substrate APIs
- Update spec_name to "pezkuwi", "zagros", "teyrchain"
- Start with spec_version = 1.0.0 for each chain

**Deliverable:** `CRYPTO_SECURITY_AUDIT.md` with:
- Complete list of keys to replace
- SS58 prefix allocation
- Token decimal definitions
- Genesis parameter specifications

---

### 0.3 Polkadot-Specific Constants & Magic Numbers

**Task:** Identify Polkadot protocol constants that may need adjustment

```bash
# Find timing constants (block time, epoch duration, etc.)
grep -rn "MILLISECS_PER_BLOCK\|SLOT_DURATION\|EPOCH_DURATION\|MINUTES\|HOURS\|DAYS" . --include="*.rs" --path="*/runtime/*"

# Find staking/governance constants
grep -rn "BondingDuration\|SessionsPerEra\|SlashDeferDuration" . --include="*.rs" --path="*/runtime/*"

# Find economic constants
grep -rn "EXISTENTIAL_DEPOSIT\|CENTS\|DOLLARS\|MILLICENTS" . --include="*.rs" --path="*/runtime/*"
```

**Categories:**
1. **Protocol Timing:** Block time (6s), epoch duration, session length
2. **Economic Parameters:** Existential deposit, transaction fees, inflation
3. **Governance:** Voting period, enactment delay, cooloff period
4. **Staking:** Bonding duration, slash defer duration, max nominators

**Decision Matrix:**
- Keep Polkadot values: Maintains compatibility, proven parameters
- Customize for Pezkuwi: Enables network differentiation, testing flexibility

**Deliverable:** `PROTOCOL_CONSTANTS.md` with recommendations

---

## Phase 1: URL & Endpoint Replacement

**Objective:** Replace all Pezkuwi-specific URLs while preserving external dependencies

### 1.1 URL Mapping Strategy

**Preserve (External Dependencies):**
```
✓ https://github.com/paritytech/*
✓ https://crates.io/*
✓ https://github.com/pezkuwichain/docs.pezkuwichain.io/*
✓ https://wiki.polkadot.network/* (for reference documentation)
✓ wss://rpc.polkadot.io (if used as reference)
```

**Replace (Branding):**
```
polkadot.network → pezkuwi.network
polkadot.io → pezkuwi.io
telemetry.polkadot.io → telemetry.pezkuwi.network
```

**Remove (Zagros-specific):**
```
zagros-rpc.polkadot.io → (remove or replace with zagros-rpc.pezkuwi.network)
zagros.subscan.io → (remove)
```

### 1.2 Execution Plan

```bash
# Step 1: Create URL replacement script
cat > /tmp/url-replacement-phase1.sh << 'SCRIPT'
#!/bin/bash

# Backup strategy
git checkout -b rebranding-phase1-urls

# Replace branding URLs (preserving external deps)
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's|polkadot\.network|pezkuwi.network|g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's|telemetry\.polkadot\.io|telemetry.pezkuwi.network|g' {} \;

# Preserve external repository URLs
echo "External dependencies preserved: GitHub, crates.io, docs.substrate.io"

# Commit changes
git add -A
git commit -m "Phase 1: Replace Polkadot branding URLs with Pezkuwi equivalents

- Changed polkadot.network → pezkuwi.network
- Changed telemetry.polkadot.io → telemetry.pezkuwi.network
- Preserved external dependencies (GitHub, crates.io, Substrate docs)
"
SCRIPT

chmod +x /tmp/url-replacement-phase1.sh
```

### 1.3 Validation

**Tests:**
- [ ] `cargo check` passes
- [ ] `cargo test` (unit tests) passes
- [ ] No external dependency URLs were modified
- [ ] Git diff review shows only intended changes

**Rollback:** `git checkout main` (phase1 branch preserved)

---

## Phase 2: Polkadot → Pezkuwi Core Rebranding

**Objective:** Rename Polkadot references to Pezkuwi in code, comments, and documentation

### 2.1 Scope

**In Scope:**
- Package names containing "polkadot"
- Code identifiers (structs, enums, functions with "Polkadot")
- Comments and documentation
- Cargo.toml package metadata
- Chain specification names

**Out of Scope (Preserve):**
- External dependencies: `pallet-*`, `sp-*`, `frame-*` crates from Parity
- Protocol-level substrate code
- Upstream git history references

### 2.2 High-Impact Changes

**Workspace Root:**
```toml
# Cargo.toml
[workspace.package]
authors = ["Pezkuwi Technologies"]
edition = "2024"
repository = "https://github.com/pezkuwi/pezkuwi-sdk"
license = "GPL-3.0-or-later"
```

**Binary Names:**
```
polkadot → pezkuwi
polkadot-teyrchain → pezkuwi-teyrchain
polkadot-prepare-worker → pezkuwi-prepare-worker
polkadot-execute-worker → pezkuwi-execute-worker
```

**Crate Names:**
```
polkadot-cli → pezkuwi-cli
polkadot-service → pezkuwi-service
polkadot-runtime → pezkuwi-runtime
polkadot-runtime-common → pezkuwi-runtime-common
polkadot-primitives → pezkuwi-primitives
```

### 2.3 Execution Script

```bash
# Step 1: Automated bulk rename
cat > /tmp/polkadot-pezkuwi-rename.sh << 'SCRIPT'
#!/bin/bash

git checkout -b rebranding-phase2-polkadot-pezkuwi

# Rename in file contents
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/Polkadot/Pezkuwi/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/polkadot/pezkuwi/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/POLKADOT/PEZKUWI/g' {} \;

# Rename directories
find . -depth -type d -name "*polkadot*" -not -path "*/target/*" -not -path "*/.git/*" | while read dir; do
  newdir=$(echo "$dir" | sed 's/polkadot/pezkuwi/g')
  if [ "$dir" != "$newdir" ]; then
    mv "$dir" "$newdir"
  fi
done

# Rename files
find . -type f -name "*polkadot*" -not -path "*/target/*" -not -path "*/.git/*" | while read file; do
  newfile=$(echo "$file" | sed 's/polkadot/pezkuwi/g')
  if [ "$file" != "$newfile" ]; then
    mv "$file" "$newfile"
  fi
done

git add -A
git commit -m "Phase 2: Polkadot → Pezkuwi core rebranding

- Renamed all Polkadot references to Pezkuwi
- Updated package names in Cargo.toml
- Renamed directories and files
- Updated code identifiers and comments
"
SCRIPT

chmod +x /tmp/polkadot-pezkuwi-rename.sh
```

### 2.4 Manual Adjustments

**After automated rename, manually review:**

1. **Cargo.toml dependencies:**
   - Ensure external `polkadot-*` crates from crates.io are NOT renamed
   - Only rename local workspace members

2. **Import paths:**
   ```rust
   // Preserve upstream imports
   use sp_runtime::traits::Block;  // ✓ Keep
   use polkadot_primitives::v7::*; // ✗ Should be: use pezkuwi_primitives::v7::*;
   ```

3. **Documentation links:**
   - Update internal docs to point to Pezkuwi
   - Keep external Polkadot wiki links for reference

### 2.5 Validation

**Build Test:**
```bash
cargo clean
cargo check --workspace
cargo test --workspace --lib
cargo build --release -p pezkuwi # formerly polkadot binary
```

**Checklist:**
- [ ] All workspace members renamed correctly
- [ ] External dependencies unchanged
- [ ] Binary builds successfully
- [ ] Runtime compiles
- [ ] Tests pass

---

## Phase 3: Pezkuwichain → PezkuwiChain Rebranding

**Objective:** Transform Pezkuwichain testnet to PezkuwiChain relay chain

### 3.1 Renaming Scope

**Identifiers:**
```
Pezkuwichain → PezkuwiChain
pezkuwichain → pezkuwichain
TYR → ZGR (Zagros token - see note below)
PEZKUWICHAIN → PEZKUWICHAIN
```

**Note:** We're mapping Pezkuwichain → PezkuwiChain but its token will be ZGR (Zagros), defined in Phase 4.

### 3.2 Key Files

**Runtime:**
```
polkadot/runtime/pezkuwichain/ → pezkuwi/runtime/pezkuwichain/
polkadot/runtime/pezkuwichain/src/lib.rs → pezkuwi/runtime/pezkuwichain/src/lib.rs
polkadot/runtime/pezkuwichain/Cargo.toml
```

**Chain Specs:**
```
polkadot/chain-specs/pezkuwichain*.json → pezkuwi/chain-specs/pezkuwichain*.json
```

**Primitives:**
```
polkadot/primitives/src/pezkuwichain.rs (if exists)
```

### 3.3 Execution Script

```bash
cat > /tmp/pezkuwichain-pezkuwichain-rename.sh << 'SCRIPT'
#!/bin/bash

git checkout -b rebranding-phase3-pezkuwichain-pezkuwichain

# Rename in contents
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/Pezkuwichain/PezkuwiChain/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/pezkuwichain/pezkuwichain/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/PEZKUWICHAIN/PEZKUWICHAIN/g' {} \;

# Rename directories
find . -depth -type d -name "*pezkuwichain*" -not -path "*/target/*" -not -path "*/.git/*" | while read dir; do
  newdir=$(echo "$dir" | sed 's/pezkuwichain/pezkuwichain/g')
  if [ "$dir" != "$newdir" ]; then
    mv "$dir" "$newdir"
  fi
done

# Rename files
find . -type f -name "*pezkuwichain*" -not -path "*/target/*" -not -path "*/.git/*" | while read file; do
  newfile=$(echo "$file" | sed 's/pezkuwichain/pezkuwichain/g')
  if [ "$file" != "$newfile" ]; then
    mv "$file" "$newfile"
  fi
done

git add -A
git commit -m "Phase 3: Pezkuwichain → PezkuwiChain relay chain rebranding

- Renamed Pezkuwichain to PezkuwiChain throughout codebase
- Updated runtime package names
- Renamed chain specifications
- Updated all code references and comments
"
SCRIPT

chmod +x /tmp/pezkuwichain-pezkuwichain-rename.sh
```

### 3.4 Runtime-Specific Changes

**Update Runtime Spec:**
```rust
// pezkuwi/runtime/pezkuwichain/src/lib.rs

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("pezkuwichain"),
    impl_name: create_runtime_str!("pezkuwi-pezkuwichain"),
    authoring_version: 1,
    spec_version: 1_000_000, // Start fresh versioning
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
```

**Update Token Symbol (ZGR will be set in Phase 4):**
```rust
// Temporarily keep as "UNIT" or "ZGR"
pub const UNITS: Balance = 1_000_000_000_000; // 12 decimals
```

### 3.5 Chain Spec Generation

```bash
# Generate new PezkuwiChain chain spec
./target/release/pezkuwi build-spec --chain pezkuwichain-local > chain-specs/pezkuwichain-local.json

# Generate raw spec
./target/release/pezkuwi build-spec --chain pezkuwichain-local --raw > chain-specs/pezkuwichain-local-raw.json
```

### 3.6 Validation

**Tests:**
- [ ] Runtime compiles: `cargo build --release -p pezkuwichain-runtime`
- [ ] Chain spec generates successfully
- [ ] Node starts: `./target/release/pezkuwi --chain pezkuwichain-local --alice`
- [ ] Produces blocks in local testnet

---

## Phase 4: Zagros → Zagros Rebranding

**Objective:** Transform Zagros testnet to Zagros network

**Decision Point:** Before starting this phase, confirm whether to:
1. Keep Zagros infrastructure (recommended for Polkadot bridge compatibility)
2. Rebrand to Zagros (full independence)
3. Remove Zagros entirely (simplification)

**Recommended:** Option 1 or 2 (keep infrastructure for bridge testing)

### 4.1 Renaming Scope

**Identifiers:**
```
Zagros → Zagros
zagros → zagros
ZGR → TYR (Tyr token)
ZAGROS → ZAGROS
```

### 4.2 Execution Script

```bash
cat > /tmp/zagros-zagros-rename.sh << 'SCRIPT'
#!/bin/bash

git checkout -b rebranding-phase4-zagros-zagros

# Rename in contents
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/Zagros/Zagros/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/zagros/zagros/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -exec sed -i 's/ZAGROS/ZAGROS/g' {} \;

# Rename directories
find . -depth -type d -name "*zagros*" -not -path "*/target/*" -not -path "*/.git/*" | while read dir; do
  newdir=$(echo "$dir" | sed 's/zagros/zagros/g')
  if [ "$dir" != "$newdir" ]; then
    mv "$dir" "$newdir"
  fi
done

# Rename files
find . -type f -name "*zagros*" -not -path "*/target/*" -not -path "*/.git/*" | while read file; do
  newfile=$(echo "$file" | sed 's/zagros/zagros/g')
  if [ "$file" != "$newfile" ]; then
    mv "$file" "$newfile"
  fi
done

git add -A
git commit -m "Phase 4: Zagros → Zagros network rebranding

- Renamed Zagros to Zagros throughout codebase
- Updated runtime package names
- Renamed chain specifications
- Prepared for TYR token integration (Phase 6)
"
SCRIPT

chmod +x /tmp/zagros-zagros-rename.sh
```

### 4.3 Runtime Configuration

**Update Runtime Spec:**
```rust
// pezkuwi/runtime/zagros/src/lib.rs

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("zagros"),
    impl_name: create_runtime_str!("pezkuwi-zagros"),
    authoring_version: 1,
    spec_version: 1_000_000,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
```

### 4.4 Validation

- [ ] Runtime builds: `cargo build --release -p zagros-runtime`
- [ ] Chain spec generates
- [ ] Node starts with Zagros chain

---

## Phase 5: Teyrchain → TeyrChain Rebranding

**Objective:** Rename template/example teyrchains to TeyrChain

### 5.1 Scope

**Target Teyrchains:**
- Generic teyrchain template → TeyrChain
- Asset Hub Pezkuwichain → Asset Hub PezkuwiChain (or rename to custom name)
- Bridge Hub Pezkuwichain → Bridge Hub PezkuwiChain
- Collectives Pezkuwichain → Collectives PezkuwiChain

**Primary Focus:** Template teyrchain → TeyrChain

### 5.2 Execution Script

```bash
cat > /tmp/teyrchain-teyrchain-rename.sh << 'SCRIPT'
#!/bin/bash

git checkout -b rebranding-phase5-teyrchain-teyrchain

# Rename teyrchain-template to teyrchain
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -path "*/teyrchain-template/*" \
  -exec sed -i 's/teyrchain-template/teyrchain/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.json" \) \
  -path "*/teyrchain-template/*" \
  -exec sed -i 's/TeyrchainTemplate/TeyrChain/g' {} \;

# Rename system teyrchains
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" \) \
  -exec sed -i 's/asset-hub-pezkuwichain/asset-hub-pezkuwichain/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" \) \
  -exec sed -i 's/bridge-hub-pezkuwichain/bridge-hub-pezkuwichain/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" \) \
  -exec sed -i 's/collectives-pezkuwichain/collectives-pezkuwichain/g' {} \;

# Rename directories
find . -depth -type d -name "*teyrchain-template*" -not -path "*/target/*" | while read dir; do
  newdir=$(echo "$dir" | sed 's/teyrchain-template/teyrchain/g')
  mv "$dir" "$newdir" 2>/dev/null
done

git add -A
git commit -m "Phase 5: Teyrchain → TeyrChain rebranding

- Renamed teyrchain-template to teyrchain
- Updated system teyrchains (Asset Hub, Bridge Hub, Collectives)
- Updated all teyrchain references to PezkuwiChain ecosystem
"
SCRIPT

chmod +x /tmp/teyrchain-teyrchain-rename.sh
```

### 5.3 TeyrChain Runtime Configuration

```rust
// cumulus/teyrchains/runtimes/teyrchain/src/lib.rs

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("teyrchain"),
    impl_name: create_runtime_str!("teyrchain-node"),
    authoring_version: 1,
    spec_version: 1_000_000,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

// Teyrchain Info
parameter_types! {
    pub const TeyrchainId: ParaId = ParaId::new(2000); // Default para ID
}
```

### 5.4 Validation

- [ ] TeyrChain runtime builds: `cargo build --release -p teyrchain-runtime`
- [ ] Collator binary builds: `cargo build --release -p teyrchain-collator`
- [ ] Can register on PezkuwiChain local testnet

---

## Phase 6: Token Symbol Rebranding

**Objective:** Replace HEZ, TYR, ZGR with HEZ, ZGR, TYR throughout the codebase

### 6.1 Token Mapping

| Original | New | Network | Decimals |
|----------|-----|---------|----------|
| HEZ | HEZ | Pezkuwi (mainnet) | 10 |
| TYR | ZGR | PezkuwiChain (testnet relay) | 12 |
| ZGR | TYR | Zagros (testnet) | 12 |

### 6.2 Execution Script

```bash
cat > /tmp/token-rebranding.sh << 'SCRIPT'
#!/bin/bash

git checkout -b rebranding-phase6-tokens

# HEZ → HEZ (Pezkuwi mainnet)
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" -o -name "*.md" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -path "*/pezkuwi/runtime/pezkuwi/*" \
  -exec sed -i 's/"HEZ"/"HEZ"/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -path "*/pezkuwi/runtime/pezkuwi/*" \
  -exec sed -i 's/\bDOT\b/HEZ/g' {} \;

# TYR → ZGR (PezkuwiChain testnet)
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" -o -name "*.md" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -path "*/runtime/pezkuwichain/*" \
  -exec sed -i 's/"TYR"/"ZGR"/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -path "*/runtime/pezkuwichain/*" \
  -exec sed -i 's/\bTYR\b/ZGR/g' {} \;

# ZGR → TYR (Zagros testnet)
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" -o -name "*.md" \) \
  -not -path "*/target/*" \
  -not -path "*/.git/*" \
  -path "*/runtime/zagros/*" \
  -exec sed -i 's/"ZGR"/"TYR"/g' {} \;

find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.json" \) \
  -not -path "*/target/*" \
  -path "*/runtime/zagros/*" \
  -exec sed -i 's/\bWND\b/TYR/g' {} \;

# Update token properties in chain specs
find . -name "*pezkuwi*.json" -path "*/chain-specs/*" -exec sed -i 's/"tokenSymbol": "HEZ"/"tokenSymbol": "HEZ"/g' {} \;
find . -name "*pezkuwichain*.json" -path "*/chain-specs/*" -exec sed -i 's/"tokenSymbol": "TYR"/"tokenSymbol": "ZGR"/g' {} \;
find . -name "*zagros*.json" -path "*/chain-specs/*" -exec sed -i 's/"tokenSymbol": "ZGR"/"tokenSymbol": "TYR"/g' {} \;
find . -name "*teyrchain*.json" -path "*/chain-specs/*" -exec sed -i 's/"tokenSymbol": "TYR"/"tokenSymbol": "TEYR"/g' {} \;

git add -A
git commit -m "Phase 6: Token symbol rebranding (HEZ→HEZ, TYR→ZGR, ZGR→TYR)

- Replaced HEZ with HEZ in Pezkuwi mainnet runtime
- Replaced TYR with ZGR in PezkuwiChain testnet
- Replaced ZGR with TYR in Zagros testnet
- Updated chain specification token metadata
- Set correct decimal places for each token
"
SCRIPT

chmod +x /tmp/token-rebranding.sh
```

### 6.3 Decimal Configuration

**Update in runtime constants:**

```rust
// pezkuwi/runtime/pezkuwi/src/lib.rs (Mainnet)
pub const DECIMALS: u8 = 10;
pub const UNITS: Balance = 10_000_000_000; // 10^10

// pezkuwi/runtime/pezkuwichain/src/lib.rs (Testnet Relay)
pub const DECIMALS: u8 = 12;
pub const UNITS: Balance = 1_000_000_000_000; // 10^12

// pezkuwi/runtime/zagros/src/lib.rs (Testnet)
pub const DECIMALS: u8 = 12;
pub const UNITS: Balance = 1_000_000_000_000; // 10^12

// cumulus/teyrchains/runtimes/teyrchain/src/lib.rs
pub const DECIMALS: u8 = 12;
pub const UNITS: Balance = 1_000_000_000_000; // 10^12
```

### 6.4 Validation

**Test Token Display:**
```bash
# Check chain spec
jq '.properties' chain-specs/pezkuwichain-local.json
# Expected: {"tokenSymbol": "ZGR", "tokenDecimals": 12}

# Start node and check
./target/release/pezkuwi --chain pezkuwichain-local --alice
# Query system.properties via RPC, should show ZGR
```

**Checklist:**
- [ ] All token symbols updated in runtime
- [ ] Chain specs reflect new tokens
- [ ] Decimal places configured correctly
- [ ] UI displays correct symbols (if applicable)

---

## Integration & Testing Plan

### Post-Phase 6 Validation

**Full Build Test:**
```bash
# Clean build
cargo clean

# Build all workspace members
cargo build --release --workspace

# Build specific binaries
cargo build --release -p pezkuwi
cargo build --release -p teyrchain-collator
cargo build --release -p pezkuwi-omni-node

# Run tests
cargo test --workspace --lib
```

**Local Testnet Deployment:**

1. **Start PezkuwiChain Relay Chain:**
```bash
# Alice (Validator)
./target/release/pezkuwi \
  --chain pezkuwichain-local \
  --alice \
  --base-path /tmp/relay-alice \
  --port 30333 \
  --rpc-port 9944

# Bob (Validator)
./target/release/pezkuwi \
  --chain pezkuwichain-local \
  --bob \
  --base-path /tmp/relay-bob \
  --port 30334 \
  --rpc-port 9945 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<ALICE_PEER_ID>
```

2. **Register TeyrChain Teyrchain:**
```bash
# Generate chain spec for TeyrChain
./target/release/teyrchain-collator build-spec \
  --chain teyrchain-local \
  > teyrchain-local.json

# Generate WASM runtime
./target/release/teyrchain-collator export-genesis-wasm \
  --chain teyrchain-local \
  > teyrchain-wasm

# Generate genesis state
./target/release/teyrchain-collator export-genesis-state \
  --chain teyrchain-local \
  > teyrchain-genesis

# Start collator
./target/release/teyrchain-collator \
  --alice \
  --collator \
  --chain teyrchain-local \
  --base-path /tmp/teyr-alice \
  --port 40333 \
  --rpc-port 8844 \
  -- \
  --chain pezkuwichain-local \
  --port 40343 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<ALICE_PEER_ID>
```

3. **Verify Network:**
- [ ] Relay chain produces blocks (6s interval)
- [ ] TeyrChain collator connects to relay chain
- [ ] Teyrchain produces blocks after registration
- [ ] Token symbols display correctly (ZGR on relay, TEYR on para)

---

## Risk Mitigation & Rollback Strategy

### Git Branch Strategy

Each phase has its own branch:
```
main (clean starting point)
├── rebranding-phase0-discovery (documentation only)
├── rebranding-phase1-urls
├── rebranding-phase2-polkadot-pezkuwi
├── rebranding-phase3-pezkuwichain-pezkuwichain
├── rebranding-phase4-zagros-zagros
├── rebranding-phase5-teyrchain-teyrchain
└── rebranding-phase6-tokens
```

**Merge Strategy:**
- Only merge to main after full validation
- Keep phase branches for reference
- Tag each successful phase: `git tag phase-N-complete`

### Rollback Procedures

**Immediate Rollback (if build fails):**
```bash
git checkout main
# Investigate failure, fix script, retry phase
```

**Partial Rollback (specific file):**
```bash
git checkout main -- path/to/file.rs
```

**Complete Reset:**
```bash
git reset --hard main
```

---

## Documentation Updates

### Files to Create/Update

1. **README.md** - Update project description to Pezkuwi
2. **CONTRIBUTING.md** - Update contribution guidelines
3. **LICENSE** - Verify GPL-3.0 license (no change needed)
4. **docs/** - Update all documentation
5. **Chain Spec Documentation** - Create guides for PezkuwiChain, Zagros, TeyrChain

### New Documentation

**Create:**
- `PEZKUWI_GENESIS.md` - Genesis configuration for mainnet
- `TESTNET_GUIDE.md` - How to connect to PezkuwiChain/Zagros testnets
- `TEYRCHAIN_DEPLOYMENT.md` - TeyrChain deployment guide
- `TOKEN_ECONOMICS.md` - HEZ/ZGR/TYR tokenomics

---

## Timeline Estimate

| Phase | Estimated Time | Dependencies |
|-------|---------------|--------------|
| Phase 0 | 2-3 days | None |
| Phase 1 | 1 day | Phase 0 complete |
| Phase 2 | 2-3 days | Phase 1 complete |
| Phase 3 | 1-2 days | Phase 2 complete |
| Phase 4 | 1-2 days | Phase 2 complete (can run parallel with Phase 3) |
| Phase 5 | 2-3 days | Phases 3,4 complete |
| Phase 6 | 1 day | Phase 5 complete |
| Integration Testing | 3-5 days | All phases complete |
| **Total** | **13-22 days** | Sequential execution |

**Parallelization Opportunities:**
- Phase 3 and Phase 4 can run in parallel (separate branches)
- Phase 0 discovery can inform all subsequent phases

---

## Success Criteria

### Phase Completion Checklist

**Each phase must meet:**
- [ ] `cargo check --workspace` passes without errors
- [ ] `cargo test --workspace --lib` passes
- [ ] Git commit created with clear message
- [ ] Manual code review completed
- [ ] No external dependencies broken

**Final Success Criteria:**
- [ ] Full workspace builds successfully
- [ ] All unit tests pass
- [ ] Local testnet (relay + teyrchain) operational
- [ ] Token symbols display correctly
- [ ] Chain specs generate without errors
- [ ] Documentation updated
- [ ] No Polkadot branding visible in user-facing code
- [ ] External dependencies (Substrate, Parity crates) preserved

---

## Appendix A: Critical Files Inventory

### Must-Review Files (High Priority)

**Workspace Root:**
- `Cargo.toml` - Workspace definition, metadata
- `.github/workflows/*.yml` - CI/CD configurations
- `README.md` - Project introduction

**Runtime Directories:**
- `pezkuwi/runtime/pezkuwi/` - Mainnet runtime (HEZ token)
- `pezkuwi/runtime/pezkuwichain/` - Testnet relay runtime (ZGR token)
- `pezkuwi/runtime/zagros/` - Alternative testnet runtime (TYR token)
- `cumulus/teyrchains/runtimes/teyrchain/` - Teyrchain runtime (TEYR token)

**Chain Specifications:**
- `chain-specs/*.json` - All chain spec files

**Binary Crates:**
- `pezkuwi/cli/` - Main CLI interface
- `pezkuwi/node/service/` - Node service implementation

---

## Appendix B: External Dependency Preservation

### DO NOT Modify These Patterns

**Preserve Exact:**
```
# Crates from crates.io
sp-*
frame-*
pallet-*
sc-*
substrate-*

# GitHub URLs
https://github.com/paritytech/*
https://github.com/substrate-developer-hub/*

# Documentation
https://github.com/pezkuwichain/docs.pezkuwichain.io/*
https://wiki.polkadot.network/* (reference docs only)

# Crate registry
https://crates.io/*
```

**Reasoning:** These are upstream dependencies maintained by Parity Technologies. Modifying them would break builds and prevent upstream merges.

---

## Appendix C: Contact & Support

**Questions During Rebranding:**
- Review this handoff document first
- Check git history for context: `git log --oneline --graph`
- Consult Substrate/Polkadot docs for protocol questions

**Emergency Rollback:**
```bash
# Nuclear option - return to clean state
git stash  # Save any local work
git checkout main
git branch -D rebranding-phase-*  # Delete all rebranding branches
```

---

## Status Tracking

**Current Status:** Phase 0 - Ready to Begin
**Last Updated:** 2025-11-26
**Repository State:** Clean, Rust 1.91.1, Cargo 1.91.1
**Verified Working:** Benchmark builds passing

**Next Step:** Execute Phase 0 Discovery & Documentation

---

**END OF REBRANDING HANDOFF DOCUMENT**
