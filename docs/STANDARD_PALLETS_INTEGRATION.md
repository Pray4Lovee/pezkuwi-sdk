# Standard Pallets Integration Plan - Phase 1

**Target Runtime:** PezkuwiChain (Relay Chain)
**Reference Runtime:** Zagros/Westend (existing working implementation)
**Goal:** Add all missing standard Polkadot pallets to make runtime production-ready

---

## Progress Tracker

### Step 1: Cargo.toml Dependencies
- [x] Add `sp-npos-elections`
- [x] Add `frame-election-provider-support`
- [x] Add `pallet-bags-list`
- [x] Add `pallet-election-provider-multi-phase`
- [x] Add `pallet-fast-unstake`
- [x] Add `pallet-collective`
- [x] Add `pallet-asset-conversion`
- [x] Add `pallet-assets`
- [x] Add `pallet-nfts`
- [x] Add `pallet-nomination-pools`
- [x] Add `pallet-nomination-pools-runtime-api`
- [x] Add `pallet-staking-runtime-api`
- [x] Add benchmarking crates (election-provider-support, nomination-pools, offences, session)
- [x] Update `std` feature list
- [x] Update `runtime-benchmarks` feature list
- [x] Update `try-runtime` feature list

### Step 2: lib.rs - Import Statements
- [x] Add `use pallet_staking::UseValidatorsMap` (line 132)
- [x] Add staking-related type aliases
- [ ] Add `VariantCountOf` to frame_support imports (for freeze support)
- [ ] Add `PoolId` from pallet_nomination_pools

### Step 3: Parameter Types
- [x] Add staking parameters (SessionsPerEra, BondingDuration, SlashDeferDuration, etc.)
- [x] Add bags-list parameters (BagThresholds)
- [x] Add nomination pools parameters (PoolsPalletId, MaxPointsToBalance)
- [x] Add fast-unstake parameters
- [ ] Add election parameters (if using ElectionProviderMultiPhase)

### Step 4: Pallet Config Implementations

#### Priority 1 - Core Staking (REQUIRED for PoS)
- [x] `impl pallet_staking::Config for Runtime` (line 707)
- [x] `impl pallet_bags_list::Config<VoterBagsListInstance> for Runtime` (line 655)
- [x] `impl frame_election_provider_support::onchain::Config for OnChainSeqPhragmen`
- [ ] `impl pallet_election_provider_multi_phase::Config for Runtime` (optional - using OnChain)

#### Priority 2 - Staking Extensions
- [x] `impl pallet_nomination_pools::Config for Runtime` (line 753)
- [x] `impl pallet_fast_unstake::Config for Runtime` (line 778)

#### Priority 3 - Governance (check existing)
- [ ] `impl pallet_collective::Config<Instance1> for Runtime` (Council) - VERIFY
- [x] `impl pallet_conviction_voting::Config for Runtime` (in governance/)
- [x] `impl pallet_referenda::Config for Runtime` (in governance/)
- [x] `impl pallet_whitelist::Config for Runtime` (in governance/)

#### Priority 4 - Assets (check existing)
- [ ] `impl pallet_assets::Config for Runtime` - VERIFY
- [ ] `impl pallet_asset_conversion::Config for Runtime` - VERIFY

#### Priority 5 - NFTs
- [ ] `impl pallet_nfts::Config for Runtime` - VERIFY

### Step 5: construct_runtime! Entries

**CRITICAL - THESE ARE MISSING:**

#### Staking Group
- [ ] `Staking: pallet_staking = 9` ❌ NOT IN construct_runtime!
- [ ] `VoterList: pallet_bags_list::<Instance1> = 13` ❌ NOT IN construct_runtime!
- [ ] `NominationPools: pallet_nomination_pools = 14` ❌ NOT IN construct_runtime!
- [ ] `FastUnstake: pallet_fast_unstake = 15` ❌ NOT IN construct_runtime!

#### Currently in construct_runtime (governance):
- [x] `Treasury: pallet_treasury = 18`
- [x] `ConvictionVoting: pallet_conviction_voting = 20`
- [x] `Referenda: pallet_referenda = 21`
- [x] `FellowshipCollective: pallet_ranked_collective::<Instance1> = 22`
- [x] `FellowshipReferenda: pallet_referenda::<Instance2> = 23`
- [x] `Origins: pallet_custom_origins = 43`
- [x] `Whitelist: pallet_whitelist = 44`

### Step 6: Helper Types and Functions
- [x] Add `EraPayout` struct for staking rewards (line 670)
- [x] Add `OnChainSeqPhragmen` type
- [x] Add `VoterBagsListInstance` type alias (line 654)
- [ ] Add `BalanceToU256` and `U256ToBalance` converters
- [ ] Update `pallet_balances::Config` with `FreezeIdentifier = RuntimeFreezeReason`

### Step 7: Runtime API Implementations
- [ ] Add `pallet_nomination_pools_runtime_api::NominationPoolsApi` ❌ NOT IMPLEMENTED

### Step 8: Benchmarks
- [x] Add new pallets to benchmark list in Cargo.toml
- [ ] Generate weights for new pallets (using defaults currently)

### Step 9: Genesis Config
- [ ] Update `genesis_config_presets.rs` with staking genesis
- [ ] Add nomination pools genesis (optional)

### Step 10: Testing
- [x] `cargo check -p pezkuwichain-runtime` - SUCCESS (previous session)
- [ ] `cargo build -p pezkuwichain-runtime --release`
- [ ] `cargo build -p pezkuwichain-runtime --release --features runtime-benchmarks`
- [ ] Run local testnet

---

## IMMEDIATE ACTION REQUIRED

The following must be added to `construct_runtime!` macro:

```rust
// After AuthorityDiscovery: pallet_authority_discovery = 12,
// Add:
Staking: pallet_staking = 9,
VoterList: pallet_bags_list::<Instance1> = 13,
NominationPools: pallet_nomination_pools = 14,
FastUnstake: pallet_fast_unstake = 15,
```

Without these entries, the pallets exist in code but are NOT part of the runtime!

---

## Index Allocation Plan

**Current used indices:** 0-8, 10, 12, 18-35, 38-45, 50-74, 98-99, 240-242, 248-252, 254-255

**New allocations needed:**
| Index | Pallet | Status |
|-------|--------|--------|
| 9 | Staking | Config exists, NOT in construct_runtime |
| 13 | VoterList | Config exists, NOT in construct_runtime |
| 14 | NominationPools | Config exists, NOT in construct_runtime |
| 15 | FastUnstake | Config exists, NOT in construct_runtime |

---

## Reference Files

- **Westend lib.rs:** `/pezkuwi/runtime/westend/src/lib.rs`
- **Target lib.rs:** `/pezkuwi/runtime/pezkuwichain/src/lib.rs`
- **Target Cargo.toml:** `/pezkuwi/runtime/pezkuwichain/Cargo.toml`

---

## Notes

1. **CRITICAL:** Pallet configs exist but pallets are NOT in construct_runtime! - they won't work
2. Staking requires VoterList (bags-list) and ElectionProvider
3. Nomination pools depend on staking
4. Fast unstake depends on staking
5. Using OnChainExecution for election provider (simpler, appropriate for testnet)
6. TransferStake adapter is deprecated - consider DelegateStake for production

---

## Session Log

**2024-11-27 Session 1:**
- [x] Completed Step 1 (Cargo.toml dependencies)
- [x] `cargo check -p pezkuwichain-runtime` - SUCCESS (5m 33s)

**2024-11-27 Session 2:**
- [x] Verified pallet configs exist in lib.rs
- [x] FOUND: construct_runtime! entries are MISSING for Staking, VoterList, NominationPools, FastUnstake
- [x] FOUND: NominationPoolsApi runtime API is NOT implemented
- Next: Add construct_runtime entries + Runtime APIs
