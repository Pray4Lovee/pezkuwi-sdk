# Phase 0.1: URL Discovery & Inventory

**Repository:** /home/mamostehp/polkadot-sdk-fresh
**Branch:** rebranding-phase0-discovery
**Analysis Date:** 2025-11-26

---

## Executive Summary

This document provides a comprehensive inventory of all URLs found in the Polkadot SDK codebase as part of the TeyrChain/Pezkuwi rebranding initiative. URLs have been categorized into three main groups: PRESERVE (external dependencies), REPLACE (Polkadot branding), and ANALYZE (endpoints).

### Summary Statistics

| Category | Count | Percentage |
|----------|-------|------------|
| **Total URL references** | 6,477 | 100% |
| **Total files with URLs** | 4,812 | - |
| **Polkadot branding URLs** | 266 | 4.1% |
| **External dependencies** | 967 | 14.9% |
| **WebSocket/RPC endpoints** | 176 | 2.7% |
| **Apache/GNU license URLs** | 3,944 | 60.9% |

### Files by Type

| File Type | Files with URLs | Purpose |
|-----------|----------------|---------|
| Rust (.rs) | 4,575 | Source code, tests, examples |
| Markdown (.md) | 155 | Documentation |
| TOML (.toml) | 79 | Configuration, dependencies |
| JSON (.json) | 3 | Chain specs, configs |

### Files Requiring Branding Updates

- **100 files** contain Polkadot/Parity branding URLs that need review
- **266 URL references** across these files need categorization

---

## Top 50 URL Domains by Frequency

| Count | Domain | Category |
|-------|--------|----------|
| 2,848 | http://www.apache.org | LICENSE (Preserve) |
| 1,096 | http://www.gnu.org | LICENSE (Preserve) |
| 857 | https://github.com | Dependencies (Preserve) |
| 627 | https://www.gnu.org | LICENSE (Preserve) |
| 240 | https://docs.rs | Rust docs (Preserve) |
| 116 | https://paritytech.github.io | Parity docs (Analyze) |
| 55 | https://docs.substrate.io | Substrate docs (Preserve) |
| 50 | https://img.shields.io | Badges (Preserve) |
| 42 | https://doc.rust-lang.org | Rust docs (Preserve) |
| 38 | https://crates.io | Dependencies (Preserve) |
| 37 | https://polkadot.js.org | Polkadot ecosystem (Analyze) |
| 35 | https://wiki.polkadot.network | Polkadot docs (Analyze) |
| 33 | https://research.web3.foundation | W3F research (Preserve) |
| 30 | http:// | Generic/incomplete |
| 25 | http://localhost | Local dev (Preserve) |
| 22 | https://polkadot.com | Polkadot branding (Replace) |
| 18 | https://substrate.stackexchange.com | Community (Preserve) |
| 18 | https://docs.polkadot.com | Polkadot docs (Analyze) |
| 16 | https://eips.ethereum.org | Ethereum specs (Preserve) |
| 15 | https://en.wikipedia.org | Reference (Preserve) |
| 15 | http://unlicense.org | License (Preserve) |
| 9 | https://raw.githubusercontent.com | GitHub raw (Preserve) |
| 9 | https://grafana.com | Monitoring (Preserve) |
| 8 | https://www.youtube.com | Media (Preserve) |
| 7 | https://tools.ietf.org | IETF specs (Preserve) |
| 7 | https://polkadot-fellows.github.io | Fellows docs (Analyze) |
| 6 | https://use.ink | Ink! docs (Preserve) |
| 6 | https://unique.network | Parachain (Preserve) |
| 6 | https://sora.org | Parachain (Preserve) |
| 5 | https://substrate.io | Substrate branding (Analyze) |
| 5 | https://storage.googleapis.com | Cloud storage (Preserve) |
| 5 | https://min-api.cryptocompare.com | Price API (Preserve) |
| 5 | https://crates.parity.io | Parity crates (Analyze) |
| 4 | https://zero.io | Parachain (Preserve) |
| 4 | https://www.shawntabrizi.com | Personal blog (Preserve) |
| 4 | https://www.rust-lang.org | Rust docs (Preserve) |
| 4 | https://www.peaq.network | Parachain (Preserve) |
| 4 | https://w3f.github.io | W3F docs (Preserve) |
| 4 | https://totemaccounting.com | Parachain (Preserve) |
| 4 | https://t.me | Telegram (Preserve) |
| 4 | https://subspace.network | Parachain (Preserve) |
| 4 | https://prometheus.io | Monitoring (Preserve) |
| 4 | https://polkafoundry.com | Parachain (Preserve) |
| 4 | https://polkadex.trade | Parachain (Preserve) |
| 4 | https://pendulumchain.org | Parachain (Preserve) |
| 4 | https://parallel.fi | Parachain (Preserve) |
| 4 | https://moonbeam.network | Parachain (Preserve) |
| 4 | https://mathwallet.org | Wallet (Preserve) |
| 4 | https://manta.network | Parachain (Preserve) |
| 4 | https://litentry.com | Parachain (Preserve) |

---

## Category 1: PRESERVE (External Dependencies)

### GitHub Repository References (Top 30)

External project dependencies that must be preserved:

| Count | Repository | Purpose |
|-------|------------|---------|
| 304 | https://github.com/paritytech/polkadot-sdk | This repository (self-reference) |
| 125 | https://github.com/paritytech/substrate | Substrate framework |
| 83 | https://github.com/paritytech/polkadot | Polkadot reference |
| 23 | https://github.com/rust-lang/rust | Rust language |
| 21 | https://github.com/polkadot-fellows/RFCs | Fellows RFCs |
| 20 | https://github.com/paritytech/parity-bridges-common | Bridge commons |
| 12 | https://github.com/Snowfork/snowbridge | Snowbridge |
| 10 | https://github.com/polkadot-fellows/runtimes | Fellows runtimes |
| 10 | https://github.com/paritytech/xcm-format | XCM format |
| 10 | https://github.com/ethereum/consensus-specs | Ethereum specs |
| 9 | https://github.com/paritytech/zombienet | Zombienet testing |
| 8 | https://github.com/libp2p/specs | Libp2p specs |
| 7 | https://github.com/tokio-rs/tracing | Tokio tracing |
| 6 | https://github.com/paritytech/cumulus | Cumulus |
| 6 | https://github.com/AcalaNetwork/chopsticks | Chopsticks |
| 5 | https://github.com/polkadot-js/apps | Polkadot.js apps |
| 5 | https://github.com/paritytech/try-runtime-cli | Try-runtime CLI |
| 5 | https://github.com/paritytech/release-registry | Release registry |
| 5 | https://github.com/paritytech/polkadot-sdk-parachain-template | Parachain template |
| 5 | https://github.com/paritytech/finality-grandpa | GRANDPA |
| 4 | https://github.com/w3f/consensus | W3F consensus |
| 4 | https://github.com/serde-rs/json | Serde JSON |
| 4 | https://github.com/polkadot-fellows/rfCs | Fellows RFCs |
| 4 | https://github.com/paritytech/substrate-developer-hub | Developer hub |
| 4 | https://github.com/paritytech/scripts | Parity scripts |
| 4 | https://github.com/paritytech/prdoc | PRDoc tool |
| 3 | https://github.com/polkadot-fellows/xcm-format | XCM format |
| 3 | https://github.com/paritytech/subxt | Subxt library |
| 3 | https://github.com/paritytech/polkadot-sdk-solochain-template | Solochain template |
| 3 | https://github.com/paritytech/parity-scale-codec | SCALE codec |

### Documentation & Crates

#### Rust Ecosystem (Preserve)
- https://crates.io - Rust crate registry (38 references)
- https://docs.rs - Rust documentation (240 references)
- https://doc.rust-lang.org - Rust language docs (42 references)
- https://www.rust-lang.org - Rust official site (4 references)

#### Top Crates.io References
- staging-chain-spec-builder (11 refs)
- polkadot-omni-node (7 refs)
- polkadot-parachain-bin (3 refs)
- frame-omni-bencher (3 refs)

#### Substrate Documentation (Preserve)
- https://docs.substrate.io (55 references)

#### Technical Specifications (Preserve)
- https://eips.ethereum.org - Ethereum Improvement Proposals (16 refs)
- https://tools.ietf.org - IETF specifications (7 refs)
- https://research.web3.foundation - Web3 Foundation research (33 refs)

#### License References (Preserve)
- http://www.apache.org (2,848 refs) - Apache License headers
- http://www.gnu.org (1,723 refs) - GNU License headers
- http://unlicense.org (15 refs) - Unlicense

### Development Tools (Preserve)

- https://grafana.com - Monitoring dashboards
- https://prometheus.io - Metrics collection
- https://substrate.stackexchange.com - Community support
- https://img.shields.io - Repository badges

---

## Category 2: REPLACE (Polkadot Branding)

### Polkadot & Parity Domains by Frequency

| Count | Domain | Replacement Strategy |
|-------|--------|---------------------|
| 116 | https://paritytech.github.io | Analyze context - mostly reference docs |
| 37 | https://polkadot.js.org | External tool - preserve |
| 35 | https://wiki.polkadot.network | Reference docs - preserve or replace context |
| 22 | https://polkadot.com | Replace with pezkuwi.com |
| 18 | https://docs.polkadot.com | Replace with docs.pezkuwi.network |
| 7 | https://polkadot-fellows.github.io | External org - preserve |
| 5 | https://crates.parity.io | Analyze - may be deprecated |
| 4 | https://forum.polkadot.network | Replace with forum.pezkuwi.network |
| 3 | https://www.parity.io | Context dependent - analyze |
| 3 | https://telemetry.polkadot.io | Replace with telemetry.pezkuwi.network |
| 3 | https://polkadot.network | Replace with pezkuwi.network |
| 3 | https://polkadot-discord.w3f.tools | External W3F tool - preserve |
| 2 | https://westend-asset-hub-eth-explorer.parity.io | Testnet explorer - replace |
| 2 | https://try-runtime.polkadot.io | Replace with try-runtime.pezkuwi.network |
| 2 | https://statement.polkadot.network | Replace if applicable |
| 2 | https://releases.parity.io | Parity releases - analyze |
| 2 | https://paritytech.io | Corporate site - context dependent |
| 2 | https://gitlab.parity.io | CI/CD reference - preserve |
| 1 | https://spec.polkadot.network | Replace with spec.pezkuwi.network |
| 1 | https://snapshots.polkadot.io | Replace if needed |
| 1 | https://security-submission.parity.io | Security reporting - preserve |
| 1 | https://rpc.polkadot.io | Replace with rpc.pezkuwi.network |
| 1 | https://polkadot.subscan.io | External explorer - preserve |
| 1 | https://polkadot-public.notion.site | External Notion - preserve |
| 1 | https://contracts.polkadot.io | Replace if deploying contracts site |
| 1 | http://spec.polkadot.network | Replace with spec.pezkuwi.network |

### Email Addresses Requiring Review

Found in Cargo.toml author fields and documentation:

- `admin@parity.io` - Found in multiple Cargo.toml files
- `security@parity.io` - Security reporting contact
- `bugbounty@parity.io` - Bug bounty program contact

**Recommendation:** Decide whether to:
1. Replace with pezkuwi.network domain emails
2. Preserve for historical/license purposes
3. Use neutral domain

### Documentation URLs (52 references)

References to official Polkadot documentation:

- https://wiki.polkadot.network/* - Wiki references (35 refs)
- https://docs.polkadot.com/* - Doc site references (18 refs)

**Context:** Many are educational references explaining concepts like parachains, consensus, account generation. Consider:
- **Preserve:** If referring to Polkadot ecosystem concepts
- **Replace:** If should point to TeyrChain/Pezkuwi docs
- **Supplement:** Add TeyrChain docs alongside Polkadot references

### Example File Locations

Key files containing Polkadot branding URLs:

```
./REBRANDING_HANDOFF.md - Rebranding documentation
./templates/parachain/README.md - Template documentation
./substrate/client/chain-spec/res/chain_spec.json - Chain specifications
./substrate/client/telemetry/src/endpoints.rs - Telemetry endpoints
./substrate/frame/people/src/mock.rs - Context constants
./substrate/bin/node/cli/src/chain_spec.rs - Chain spec constants
./polkadot/node/service/src/chain_spec.rs - Chain spec constants
./polkadot/node/service/chain-specs/*.json - Network chain specs
```

---

## Category 3: ANALYZE (Endpoints)

### WebSocket Endpoints

**Total WebSocket endpoint references:** 176

#### Polkadot Infrastructure Endpoints (Replace)

| Count | Endpoint | Purpose | Replacement |
|-------|----------|---------|-------------|
| 12 | wss://telemetry.polkadot.io/submit/ | Telemetry submission | wss://telemetry.pezkuwi.network/submit/ |
| 4 | wss://westend-rpc.polkadot.io:443 | Westend testnet RPC | wss://zagros-rpc.pezkuwi.network:443 |
| 1 | wss://westend-people-rpc.polkadot.io:443 | Westend People chain | Update testnet name |
| 1 | wss://westend-coretime-rpc.polkadot.io:443 | Westend Coretime | Update testnet name |
| 1 | wss://westend-collectives-rpc.polkadot.io:443 | Westend Collectives | Update testnet name |
| 1 | wss://westend-bridge-hub-rpc.polkadot.io:443 | Westend Bridge Hub | Update testnet name |
| 1 | wss://westend-asset-hub-rpc.polkadot.io:443 | Westend Asset Hub | Update testnet name |
| 1 | wss://westend-asset-hub-rpc.polkadot.io | Westend Asset Hub | Update testnet name |
| 1 | wss://try-runtime-westend.polkadot.io:443 | Try-runtime Westend | Update service |
| 1 | wss://try-runtime-rococo.polkadot.io:443 | Try-runtime Rococo | Update service |
| 1 | wss://rococo-rpc.polkadot.io:443 | Rococo testnet RPC | Update testnet name |

#### External RPC Endpoints (Preserve)

| Count | Endpoint | Provider |
|-------|----------|----------|
| 1 | wss://rpc.ibp.network/polkadot | IBP Network |
| 1 | wss://rpc.ibp.network/kusama | IBP Network |

#### Test/Example Endpoints (Preserve)

| Count | Endpoint | Purpose |
|-------|----------|---------|
| 7 | ws:// | Generic test placeholder |
| 4 | wss://something:100/path | Generic test endpoint |
| 2 | wss://something:9090/path | Generic test endpoint |
| 2 | wss://something:443/path | Generic test endpoint |
| 2 | wss://something/path | Generic test endpoint |

#### Named Test Nodes (Preserve)

| Count | Endpoint | Purpose |
|-------|----------|---------|
| 2 | ws://polkadot-alice:9944 | Test node name |
| 2 | ws://kusama-alice:9944 | Test node name |
| 2 | ws://bridge-hub-polkadot-collator1:9944 | Bridge test node |
| 2 | ws://bridge-hub-kusama-node-collator1:9944 | Bridge test node |

### Local Development Endpoints (Preserve)

| Count | Endpoint | Purpose |
|-------|----------|---------|
| 25 | http://localhost | Generic localhost |
| 7 | http://localhost:1234 | Test server |
| 6 | http://localhost:9933/ | Substrate RPC |
| 3 | ws://localhost:45789 | Test WebSocket |
| 3 | ws://127.0.0.1:9944 | Local node |
| 3 | http://localhost:8545 | Ethereum RPC (EVM) |
| 2 | ws://localhost:9944 | Local Substrate node |
| 2 | http://localhost:12345 | Test server |
| 2 | http://localhost: | Incomplete URL |

### Key Configuration Files with Endpoints

#### Telemetry Configuration
```
./substrate/client/telemetry/src/endpoints.rs:83
("wss://telemetry.polkadot.io/submit/".into(), 3)
```

#### Chain Spec Telemetry
```
./substrate/client/chain-spec/res/chain_spec.json:19
["wss://telemetry.polkadot.io/submit/", 0]

./substrate/client/chain-spec/res/chain_spec2.json:19
["wss://telemetry.polkadot.io/submit/", 0]
```

#### Chain Spec Constants
```
./substrate/bin/node/cli/src/chain_spec.rs:44
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

./polkadot/node/service/src/chain_spec.rs:31-35
const WESTEND_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ROCOCO_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const VERSI_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
```

#### Network Chain Specs (JSON)
```
./polkadot/node/service/chain-specs/kusama.json:45
./polkadot/node/service/chain-specs/westend.json:39
./polkadot/node/service/chain-specs/polkadot.json:46
```

#### Runtime Test Configurations
```
./substrate/frame/staking-async/runtimes/parachain/src/staking.rs:774
var("WS").unwrap_or("wss://westend-rpc.polkadot.io:443".to_string()).into();

./polkadot/runtime/rococo/src/lib.rs:2742
var("WS").unwrap_or("wss://rococo-rpc.polkadot.io:443".to_string()).into();
```

#### GitHub Workflows
```
./.github/workflows/runtimes-matrix.json
- wss://try-runtime-westend.polkadot.io:443
- wss://try-runtime-rococo.polkadot.io:443
- wss://westend-asset-hub-rpc.polkadot.io:443
- wss://rococo-asset-hub-rpc.polkadot.io:443
```

---

## Recommendations by Category

### PRESERVE - External Dependencies

**Action:** No changes required

These URLs reference external projects, specifications, licenses, and tools that are:
- Essential for technical functionality (crates.io, docs.rs, GitHub dependencies)
- Industry standards (IETF, W3C, Ethereum specs)
- License headers (Apache, GNU)
- External ecosystem tools (Polkadot.js, parachain projects)

**Total URLs to preserve:** ~5,800+ (89% of all URLs)

### REPLACE - Polkadot Branding URLs

**Action:** Context-dependent replacement

#### High Priority (Direct Branding)
Replace these with Pezkuwi equivalents:

1. **Domain Replacements:**
   - polkadot.com → pezkuwi.com (22 refs)
   - polkadot.network → pezkuwi.network (3 refs)
   - docs.polkadot.com → docs.pezkuwi.network (18 refs)
   - forum.polkadot.network → forum.pezkuwi.network (4 refs)
   - spec.polkadot.network → spec.pezkuwi.network (2 refs)

2. **Telemetry Infrastructure:**
   - telemetry.polkadot.io → telemetry.pezkuwi.network (15 refs)
   - Update constants in chain_spec.rs files
   - Update JSON chain specifications

3. **RPC Endpoints:**
   - westend-rpc.polkadot.io → zagros-rpc.pezkuwi.network
   - Update testnet-specific endpoints
   - Update try-runtime endpoints

#### Medium Priority (Documentation References)

Consider case-by-case:

- **wiki.polkadot.network (35 refs):** Educational references
  - Option 1: Preserve as external reference
  - Option 2: Create equivalent TeyrChain docs
  - Option 3: Add supplementary notes

- **paritytech.github.io (116 refs):** Technical documentation
  - Most are external reference docs
  - Preserve unless hosting own equivalent docs

#### Low Priority (Corporate/Historical)

- **Email addresses:** admin@parity.io, security@parity.io
  - Decision needed on organizational contacts
  - May preserve in license headers for historical accuracy

- **Parity infrastructure:** gitlab.parity.io, releases.parity.io
  - Likely CI/CD and historical references
  - Preserve unless migrating infrastructure

**Total URLs to review for replacement:** ~266 (4.1% of all URLs)

### ANALYZE - WebSocket/RPC Endpoints

**Action:** Strategic infrastructure planning

#### Immediate Action Required

**Telemetry Endpoints (15 locations):**
```rust
// Current
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

// Proposed
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.pezkuwi.network/submit/";
```

**Files to update:**
- ./substrate/bin/node/cli/src/chain_spec.rs
- ./polkadot/node/service/src/chain_spec.rs
- ./substrate/client/telemetry/src/endpoints.rs
- ./substrate/client/chain-spec/res/chain_spec.json
- ./substrate/client/chain-spec/res/chain_spec2.json
- ./polkadot/node/service/chain-specs/*.json

#### Testnet RPC Endpoints

**Westend → Zagros rebranding:**
- westend-rpc.polkadot.io → zagros-rpc.pezkuwi.network
- westend-asset-hub-rpc.polkadot.io → zagros-asset-hub-rpc.pezkuwi.network
- westend-*-rpc.polkadot.io → zagros-*-rpc.pezkuwi.network

**Rococo testnet:**
- Decide on Rococo replacement name
- Update all rococo-rpc.polkadot.io references

**Files to update:**
- Runtime test files (substrate/frame/staking-async/*)
- Utility test files (polkadot/utils/remote-ext-tests/*)
- GitHub workflow configs (.github/workflows/runtimes-matrix.json)

#### Test Node Names (Preserve)

Generic test endpoints and localhost references need no changes:
- ws://localhost:9944
- http://127.0.0.1:*
- ws://polkadot-alice:9944 (test node name, can preserve or rebrand)
- ws://something/path (generic placeholder)

**Total endpoints to analyze:** ~176 (2.7% of all URLs)

---

## Implementation Priority Matrix

### Phase 1: Critical Infrastructure (Week 1)

**Impact:** High | **Effort:** Low | **Count:** ~30 refs

1. Telemetry endpoint constants (15 locations)
2. Chain spec JSON files (6 files)
3. Chain spec Rust constants (5 files)

### Phase 2: Testnet Rebranding (Week 2)

**Impact:** High | **Effort:** Medium | **Count:** ~40 refs

1. Westend → Zagros RPC endpoints
2. Runtime test configurations
3. GitHub workflow matrices
4. Utility test default endpoints

### Phase 3: Documentation URLs (Week 3-4)

**Impact:** Medium | **Effort:** Medium | **Count:** ~50 refs

1. docs.polkadot.com references
2. wiki.polkadot.network educational links
3. Template documentation
4. README files

### Phase 4: Branding Polish (Week 5)

**Impact:** Low | **Effort:** Low | **Count:** ~150 refs

1. Corporate domain references (polkadot.com)
2. Forum links
3. Spec site references
4. Email addresses (context dependent)

---

## Risk Assessment

### High Risk Changes

**Telemetry Endpoints:**
- Risk: Breaking telemetry submission if infrastructure not ready
- Mitigation: Ensure telemetry.pezkuwi.network is operational before updating
- Fallback: Keep dual endpoints during transition

**RPC Endpoints:**
- Risk: Breaking integration tests and developer workflows
- Mitigation: Update infrastructure before code
- Fallback: Environment variable overrides for testing

### Medium Risk Changes

**Documentation URLs:**
- Risk: Broken links if replacement docs don't exist
- Mitigation: Create replacement docs first OR preserve original references
- Fallback: Use redirect infrastructure (docs.pezkuwi.network → wiki.polkadot.network)

### Low Risk Changes

**License Headers:**
- Risk: Minimal - historical references
- Mitigation: Preserve Parity references in inherited code
- Fallback: Not applicable

**Test/Example URLs:**
- Risk: Minimal - local development only
- Mitigation: Update test documentation
- Fallback: Not applicable

---

## Automation Opportunities

### Search & Replace Candidates

**Safe for bulk replacement:**
```bash
# Telemetry endpoints
telemetry.polkadot.io → telemetry.pezkuwi.network

# Documentation (if replacement exists)
docs.polkadot.com → docs.pezkuwi.network

# Branding
polkadot.com → pezkuwi.com
polkadot.network → pezkuwi.network
```

**Requires case-by-case review:**
- wiki.polkadot.network (educational references)
- paritytech.github.io (external docs)
- parity.io (corporate/infrastructure)
- Email addresses in Cargo.toml

### Recommended Tools

1. **ripgrep (rg):** Already used for discovery
2. **sd (find-replace):** Safe, preview-capable replacements
3. **Custom script:** Category-based replacement with validation
4. **Git hooks:** Prevent accidental re-introduction of old URLs

---

## Next Steps

### Phase 0.2: Infrastructure Preparation

1. **Deploy Pezkuwi infrastructure:**
   - telemetry.pezkuwi.network
   - rpc.pezkuwi.network
   - zagros-rpc.pezkuwi.network (testnet)
   - docs.pezkuwi.network

2. **Verify infrastructure:**
   - Test telemetry submission
   - Test RPC endpoints
   - Set up monitoring

### Phase 0.3: Categorization Refinement

1. **Manual review of edge cases:**
   - 100 files with Polkadot branding
   - Context-dependent references
   - Email address strategy

2. **Create replacement mapping:**
   - Document old → new URL pairs
   - Identify docs requiring creation
   - Define redirect strategy

### Phase 0.4: Validation Script

Create automated validator to:
1. Detect Polkadot branding URLs
2. Categorize by replacement strategy
3. Validate replacement URLs are reachable
4. Generate replacement PRs

---

## Appendix: Command Reference

### Discovery Commands Used

```bash
# Total URL count
grep -rn "https\?://" . --include="*.rs" --include="*.toml" --include="*.json" --include="*.md" | wc -l

# Polkadot branding URLs
grep -rn "polkadot\.network\|polkadot\.io\|parity\.io" . --include="*.rs" --include="*.toml" --include="*.json" --include="*.md" | wc -l

# External dependencies
grep -rn "github\.com\|crates\.io\|docs\.substrate\.io" . --include="*.rs" --include="*.toml" --include="*.md" | wc -l

# Endpoints
grep -rn "wss\?://\|127\.0\.0\.1\|localhost" . --include="*.rs" --include="*.toml" --include="*.json" | wc -l

# Domain frequency analysis
grep -roh "https\?://[a-zA-Z0-9.-]*" . --include="*.rs" --include="*.toml" --include="*.json" --include="*.md" | sort | uniq -c | sort -rn

# GitHub repositories
grep -roh "https://github\.com/[a-zA-Z0-9_-]*/[a-zA-Z0-9_-]*" . --include="*.rs" --include="*.toml" --include="*.md" | sort | uniq -c | sort -rn
```

### Recommended Follow-up Commands

```bash
# Find all Cargo.toml with Parity authors
grep -r "admin@parity.io" . --include="Cargo.toml"

# Find chain spec files
find . -name "*.json" -path "*/chain-spec*"

# Find telemetry constants
grep -rn "TELEMETRY_URL" . --include="*.rs"

# Find RPC endpoint environment variables
grep -rn "WS.*polkadot\.io" . --include="*.rs"
```

---

## Contact & Questions

For questions about this inventory or rebranding strategy:
- Review: /home/mamostehp/polkadot-sdk-fresh/REBRANDING_HANDOFF.md
- Git branch: rebranding-phase0-discovery

---

**Document Version:** 1.0
**Last Updated:** 2025-11-26
**Status:** Discovery Complete - Ready for Infrastructure Planning
