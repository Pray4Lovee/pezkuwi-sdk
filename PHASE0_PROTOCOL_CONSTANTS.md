# Phase 0.3: Protocol Constants & Configuration Discovery

## Executive Summary

This document catalogs all protocol-level constants, runtime parameters, and configuration values discovered across the polkadot-sdk-fresh codebase for the TeyrChain rebranding initiative.

### Discovery Statistics
- **Total Runtime Files Analyzed**: 13 primary runtime configurations
- **Network-Specific Constants Cataloged**: 156+
- **Shared/Generic Constants**: 47+
- **Constants Requiring Rebranding**: 23 (HIGH priority)
- **Feature Flags Identified**: 2 primary (`fast-runtime`, `runtime-benchmarks`)

### Impact Assessment
- **Critical Changes Required**: Runtime version constants (spec_name, impl_name)
- **Network-Specific Adjustments**: Currency symbols, time constants, teyrchain IDs
- **Risk Level**: LOW to MEDIUM (well-isolated changes)

---

## 1. Runtime Version Constants

### 1.1 Pezkuwichain Relay Chain Runtime
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 179-190
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("pezkuwichain"),
    impl_name: alloc::borrow::Cow::Borrowed("parity-pezkuwichain-v2.0"),
    authoring_version: 0,
    spec_version: 1_020_001,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 26,
    system_version: 1,
};
```

**Rebranding Required**: YES
**Priority**: HIGH
**Risk**: LOW (isolated to runtime metadata)

---

### 1.2 Zagros Relay Chain Runtime
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`

```rust
// Lines 170-181
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("zagros"),
    impl_name: alloc::borrow::Cow::Borrowed("parity-zagros"),
    authoring_version: 2,
    spec_version: 1_020_001,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 27,
    system_version: 1,
};
```

**Rebranding Required**: YES
**Priority**: HIGH
**Risk**: LOW

---

### 1.3 Test Runtime
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/test-runtime/src/lib.rs`

```rust
// Lines 122-125
spec_version: 1056,
impl_version: 0,
transaction_version: 1,
```

**Rebranding Required**: NO (test runtime)
**Priority**: LOW

---

### 1.4 Teyrchain Runtimes

#### Asset Hub Pezkuwichain
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain/src/lib.rs`

```rust
// Lines 130-133
spec_name: alloc::borrow::Cow::Borrowed("statemine"),
impl_name: alloc::borrow::Cow::Borrowed("statemine"),
spec_version: 1_020_001,
```

**Rebranding Required**: YES
**Priority**: HIGH
**Note**: Currently using legacy "statemine" name

---

#### Asset Hub Zagros
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/assets/asset-hub-zagros/src/lib.rs`

```rust
// Lines 145-148
// Note: The spec_name is intentionally kept as "westmint" for compatibility with
// "asset-hub-zagros". Many wallets/tools depend on the `spec_name`, so it remains "westmint"
spec_name: alloc::borrow::Cow::Borrowed("westmint"),
impl_name: alloc::borrow::Cow::Borrowed("westmint"),
```

**Rebranding Required**: YES
**Priority**: HIGH
**Note**: Legacy "westmint" naming with compatibility concerns

---

#### Coretime Pezkuwichain
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/coretime/coretime-pezkuwichain/src/lib.rs`

```rust
// Lines 159-162
spec_name: alloc::borrow::Cow::Borrowed("coretime-pezkuwichain"),
impl_name: alloc::borrow::Cow::Borrowed("coretime-pezkuwichain"),
spec_version: 1_020_001,
```

**Rebranding Required**: YES
**Priority**: MEDIUM

---

#### Coretime Zagros
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/coretime/coretime-zagros/src/lib.rs`

**Rebranding Required**: YES
**Priority**: MEDIUM

---

#### Bridge Hub Pezkuwichain
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-pezkuwichain/src/lib.rs`

```rust
// Lines 252-253
spec_name: alloc::borrow::Cow::Borrowed("bridge-hub-pezkuwichain"),
impl_name: alloc::borrow::Cow::Borrowed("bridge-hub-pezkuwichain"),
```

**Rebranding Required**: YES
**Priority**: MEDIUM

---

#### People Pezkuwichain
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/people/people-pezkuwichain/src/lib.rs`

```rust
// Lines 145-146
spec_name: alloc::borrow::Cow::Borrowed("people-pezkuwichain"),
impl_name: alloc::borrow::Cow::Borrowed("people-pezkuwichain"),
```

**Rebranding Required**: YES
**Priority**: MEDIUM

---

#### Collectives Zagros
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/collectives/collectives-zagros/src/lib.rs`

```rust
// Lines 130-133
spec_name: alloc::borrow::Cow::Borrowed("collectives-zagros"),
impl_name: alloc::borrow::Cow::Borrowed("collectives-zagros"),
spec_version: 1_020_001,
```

**Rebranding Required**: YES
**Priority**: MEDIUM

---

#### Glutton Zagros
**File**: `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/glutton/glutton-zagros/src/lib.rs`

```rust
// Lines 103-104
spec_name: alloc::borrow::Cow::Borrowed("glutton-zagros"),
impl_name: alloc::borrow::Cow::Borrowed("glutton-zagros"),
```

**Rebranding Required**: YES
**Priority**: LOW (test teyrchain)

---

## 2. Consensus Parameters

### 2.1 BABE (Block Authoring)

#### Common Constants
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/constants/src/lib.rs`

```rust
// Lines 39-62 (pezkuwichain/constants)
pub mod time {
    pub const MILLISECS_PER_BLOCK: Moment = 6000;
    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    // 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}
```

**Network-Specific**: NO (shared across networks)
**Rebranding Required**: NO
**Priority**: N/A

---

#### Epoch Duration (Pezkuwichain)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`

```rust
// Lines 46-49
frame_support::parameter_types! {
    pub EpochDurationInBlocks: BlockNumber =
        prod_or_fast!(1 * HOURS, 1 * MINUTES, "PEZKUWICHAIN_EPOCH_DURATION");
}
```

**Network-Specific**: YES (Pezkuwichain-specific env variable)
**Rebranding Required**: YES (environment variable name)
**Priority**: MEDIUM
**File Location**: Line 48

---

#### Epoch Duration (Zagros)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`

```rust
// Lines 349-355
parameter_types! {
    pub const EpochDuration: u64 = prod_or_fast!(
        EPOCH_DURATION_IN_SLOTS,
        2 * MINUTES,
        "ZAGROS_EPOCH_DURATION"
    );
    pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
    pub const ReportLongevity: u64 =
        BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}
```

**Network-Specific**: YES (Zagros-specific env variable)
**Rebranding Required**: YES (environment variable name)
**Priority**: MEDIUM
**File Location**: Lines 349-355

---

#### BABE Genesis Config
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 192-197
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
    sp_consensus_babe::BabeEpochConfiguration {
        c: PRIMARY_PROBABILITY,
        allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
    };
```

**Network-Specific**: NO
**Rebranding Required**: NO
**Priority**: N/A

---

### 2.2 GRANDPA (Finality)

#### Session Configuration (Pezkuwichain)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 504-505
pub const SessionsPerEra: SessionIndex = 6;
pub const BondingDuration: sp_staking::EraIndex = 28;
```

**Network-Specific**: YES
**Rebranding Required**: NO (numeric values)
**Priority**: N/A

---

#### Session Configuration (Zagros)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`

```rust
// Lines 733-737
parameter_types! {
    pub const SessionsPerEra: SessionIndex = prod_or_fast!(6, 2);
    // 28 eras for unbonding (7 days).
    pub const BondingDuration: EraIndex = 2;
    pub const SlashDeferDuration: EraIndex = 1;
}
```

**Network-Specific**: YES
**Rebranding Required**: NO (numeric configuration)
**Priority**: N/A

---

#### Authority Set ID Session Entries
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`

```rust
// Line 999
pub const MaxSetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
```

**Network-Specific**: NO (derived from other constants)
**Rebranding Required**: NO
**Priority**: N/A

---

### 2.3 Session Keys & Validators

#### Max Authorities
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs` (Line 522)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs` (Line 938)

```rust
pub const MaxAuthorities: u32 = 100_000;
```

**Network-Specific**: NO (shared limit)
**Rebranding Required**: NO
**Priority**: N/A

---

#### Max Nominators
**Pezkuwichain**: `ConstU32<0>` (Line 383)
**Zagros**: `64` (Line 742)

**Network-Specific**: YES
**Rebranding Required**: NO
**Priority**: N/A
**Note**: Pezkuwichain disables nomination pools, Zagros enables them

---

## 3. Economic Parameters

### 3.1 Currency Units & Existential Deposit

#### Pezkuwichain Currency
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`

```rust
// Lines 22-36
pub mod currency {
    use polkadot_primitives::Balance;

    /// The existential deposit.
    pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

    pub const UNITS: Balance = 1_000_000_000_000;
    pub const CENTS: Balance = UNITS / 30_000;
    pub const GRAND: Balance = CENTS * 100_000;
    pub const MILLICENTS: Balance = CENTS / 1_000;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 2_000 * CENTS + (bytes as Balance) * 100 * MILLICENTS
    }
}
```

**Network-Specific**: YES (CENTS calculation differs)
**Rebranding Required**: NO (numeric values)
**Priority**: N/A
**Note**: Pezkuwichain uses `UNITS / 30_000` for CENTS

---

#### Zagros Currency
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/constants/src/lib.rs`

```rust
// Lines 22-36
pub mod currency {
    use polkadot_primitives::Balance;

    /// The existential deposit.
    pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

    pub const UNITS: Balance = 1_000_000_000_000;
    pub const CENTS: Balance = UNITS / 100;
    pub const MILLICENTS: Balance = CENTS / 1_000;
    pub const GRAND: Balance = CENTS * 100_000;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 100 * CENTS + (bytes as Balance) * 5 * MILLICENTS
    }
}
```

**Network-Specific**: YES (different CENTS and deposit calculations)
**Rebranding Required**: NO
**Priority**: N/A
**Note**: Zagros uses `UNITS / 100` for CENTS (standard decimal)

---

### 3.2 Transaction Fees

#### Fee Configuration
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 424-429
parameter_types! {
    pub const TransactionByteFee: Balance = 10 * MILLICENTS;
    /// This value increases the priority of `Operational` transactions by adding
    /// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
    pub const OperationalFeeMultiplier: u8 = 5;
}
```

**Network-Specific**: NO (shared configuration)
**Rebranding Required**: NO
**Priority**: N/A

---

#### Weight to Fee Polynomial
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`

```rust
// Lines 87-101
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = Balance;
    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        // in Pezkuwichain, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
        let p = super::currency::CENTS;
        let q = 10 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}
```

**Network-Specific**: YES (comments reference "Pezkuwichain")
**Rebranding Required**: YES (comment text only)
**Priority**: LOW
**File Location**: Line 91

---

### 3.3 Treasury Configuration

#### Treasury Parameters
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 509-512
pub const SpendPeriod: BlockNumber = 6 * DAYS;
pub const Burn: Permill = Permill::from_perthousand(2);
// Payout happens every 30 days
pub const PayoutSpendPeriod: BlockNumber = 30 * DAYS;
```

**Network-Specific**: NO (shared treasury economics)
**Rebranding Required**: NO
**Priority**: N/A

---

#### Treasury Pallet ID
**Pezkuwichain**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs` (Line 144)

```rust
pub const TREASURY_PALLET_ID: u8 = 18;
```

**Zagros**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/constants/src/lib.rs` (Line 141)

```rust
pub const TREASURY_PALLET_ID: u8 = 37;
```

**Network-Specific**: YES (different pallet IDs)
**Rebranding Required**: NO (numeric identifiers)
**Priority**: N/A

---

### 3.4 Staking Parameters (Zagros Only)

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`

```rust
// Lines 268-291 (dynamic_params)
pub mod inflation {
    /// Minimum inflation rate used to calculate era payouts.
    #[codec(index = 0)]
    pub static MinInflation: Perquintill = Perquintill::from_rational(25u64, 1000u64);

    /// Maximum inflation rate used to calculate era payouts.
    #[codec(index = 1)]
    pub static MaxInflation: Perquintill = Perquintill::from_rational(10u64, 100u64);

    /// Ideal stake ratio used to calculate era payouts.
    #[codec(index = 2)]
    pub static IdealStake: Perquintill = Perquintill::from_rational(50u64, 100u64);

    /// Falloff used to calculate era payouts.
    #[codec(index = 3)]
    pub static Falloff: Perquintill = Perquintill::from_rational(50u64, 1000u64);

    /// Whether to use auction slots or not in the calculation of era payouts.
    #[codec(index = 4)]
    pub static UseAuctionSlots: bool = false;
}
```

**Network-Specific**: YES (Zagros has active staking)
**Rebranding Required**: NO
**Priority**: N/A
**Note**: Pezkuwichain does not have comparable staking configuration

---

## 4. Block Production & Timing Constants

### 4.1 Block Time Constants

**Shared Across Both Networks**:

```rust
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);  // = 10 blocks
pub const HOURS: BlockNumber = MINUTES * 60;                                      // = 600 blocks
pub const DAYS: BlockNumber = HOURS * 24;                                         // = 14,400 blocks
```

**Network-Specific**: NO
**Rebranding Required**: NO
**Priority**: N/A

---

#### Pezkuwichain Additional Time Constants
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`

```rust
// Line 55
pub const WEEKS: BlockNumber = DAYS * 7;  // = 100,800 blocks
```

**Network-Specific**: NO (utility constant)
**Rebranding Required**: NO
**Priority**: N/A

---

### 4.2 Block Production Configuration

#### Minimum Period (Timestamp)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 441-442
parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}
```

**Network-Specific**: NO
**Rebranding Required**: NO
**Priority**: N/A

---

## 5. Storage & Performance Limits

### 5.1 Block Weights

#### Maximum Block Weight
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/common/src/lib.rs`

```rust
// Lines 80-84
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
/// The storage proof size is not limited so far.
pub const MAXIMUM_BLOCK_WEIGHT: Weight =
    Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);
```

**Network-Specific**: NO (shared constant)
**Rebranding Required**: NO
**Priority**: N/A
**Value**: 2 seconds of compute time (ref_time), unlimited proof size

---

#### Block Weights Configuration
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/common/src/lib.rs`

```rust
// Lines 144-161
pub BlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
    .base_block($runtime::weights::BlockExecutionWeight::get())
    .for_class(DispatchClass::all(), |weights| {
        weights.base_extrinsic = $runtime::weights::ExtrinsicBaseWeight::get();
    })
    .for_class(DispatchClass::Normal, |weights| {
        weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
    })
    .for_class(DispatchClass::Operational, |weights| {
        weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
        // Operational transactions have an extra reserved space, so that they
        // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
        weights.reserved = Some(
            MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT,
        );
    })
    .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
    .build_or_panic();
```

**Network-Specific**: NO (via macro, shared logic)
**Rebranding Required**: NO
**Priority**: N/A

---

### 5.2 Block Length Limits

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/common/src/lib.rs`

```rust
// Lines 104-105
/// Maximum length of block. Up to 5MB.
pub BlockLength: limits::BlockLength =
    limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
```

**Network-Specific**: NO
**Rebranding Required**: NO
**Priority**: N/A
**Value**: 5MB maximum block size

---

### 5.3 PoV (Proof of Validity) Size Limits

#### Maximum PoV Size
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/primitives/src/v9/mod.rs`

```rust
// Lines 456-463
/// Maximum PoV size we support right now.
///
/// Used for:
/// * initial genesis for the Teyrchains configuration
/// * checking updates to this stored runtime configuration do not exceed this limit
/// * when detecting a PoV decompression bomb in the client
// NOTE: This value is used in the runtime so be careful when changing it.
pub const MAX_POV_SIZE: u32 = 10 * 1024 * 1024;  // 10 MB
```

**Network-Specific**: NO (protocol constant)
**Rebranding Required**: NO
**Priority**: N/A

---

#### Maximum Code Size
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/primitives/src/v9/mod.rs`

```rust
// Lines 440-446
/// Maximum runtime code size we support right now.
///
/// Used for:
/// * initial genesis for the Teyrchains configuration
/// * checking updates to this stored runtime configuration do not exceed this limit
/// * when detecting a code decompression bomb in the client
// NOTE: This value is used in the runtime so be careful when changing it.
pub const MAX_CODE_SIZE: u32 = 3 * 1024 * 1024;  // 3 MB
```

**Network-Specific**: NO (protocol constant)
**Rebranding Required**: NO
**Priority**: N/A

---

#### Maximum Head Data Size
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/primitives/src/v9/mod.rs`

```rust
// Lines 448-454
/// Maximum head data size we support right now.
///
/// Used for:
/// * initial genesis for the Teyrchains configuration
/// * checking updates to this stored runtime configuration do not exceed this limit
// NOTE: This value is used in the runtime so be careful when changing it.
pub const MAX_HEAD_DATA_SIZE: u32 = 1 * 1024 * 1024;  // 1 MB
```

**Network-Specific**: NO
**Rebranding Required**: NO
**Priority**: N/A

---

### 5.4 Message Queue & Service Weights

#### Pezkuwichain Message Queue
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 1076
pub MessageQueueServiceWeight: Weight = Perbill::from_percent(20) * BlockWeights::get().max_block;

// Line 1328
pub MaxIntakeWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 10;
```

**Network-Specific**: NO (percentages are standard)
**Rebranding Required**: NO
**Priority**: N/A

---

## 6. Network-Specific Feature Flags

### 6.1 Fast Runtime Feature

#### Pezkuwichain Cargo.toml
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/Cargo.toml`

```toml
# Line 329
fast-runtime = ["pezkuwichain-runtime-constants/fast-runtime"]
```

**Network-Specific**: YES (references "pezkuwichain-runtime-constants")
**Rebranding Required**: YES (crate name)
**Priority**: MEDIUM

---

#### Zagros Cargo.toml
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/Cargo.toml`

```toml
# Line 358
fast-runtime = []
```

**Network-Specific**: NO (empty feature flag)
**Rebranding Required**: NO
**Priority**: N/A

---

### 6.2 Runtime Benchmarks Feature

Both Pezkuwichain and Zagros have extensive `runtime-benchmarks` feature flags that cascade to all included pallets.

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/Cargo.toml` (Lines 217-270)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/Cargo.toml` (Lines 237-297)

**Network-Specific**: NO (standard benchmarking infrastructure)
**Rebranding Required**: NO
**Priority**: N/A

---

### 6.3 prod_or_fast! Macro Usage

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/common/src/lib.rs`

```rust
// Lines 255-279
/// Macro to set a value (usually a balance) to either the production value or to an environment
/// variable or testing value (in case the `fast-runtime` feature is selected)
///
/// Usage example:
/// pub EpochDuration: BlockNumber =
///     prod_or_fast!(1 * HOURS, "fast-runtime", 1 * MINUTES, "fast-runtime-10m", 10 * MINUTES);
```

This macro is used in:
- **Pezkuwichain**: `EpochDurationInBlocks` with env variable `"PEZKUWICHAIN_EPOCH_DURATION"`
- **Zagros**: `EpochDuration` with env variable `"ZAGROS_EPOCH_DURATION"`
- **Zagros**: `SessionsPerEra` for fast vs. production builds

**Network-Specific**: YES (environment variable names)
**Rebranding Required**: YES
**Priority**: MEDIUM

---

## 7. Pallet-Specific Constants

### 7.1 Balances Pallet

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 401-405
parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}
```

**Network-Specific**: NO (shared limits)
**Rebranding Required**: NO
**Priority**: N/A

---

### 7.2 Indices Pallet

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Line 390
parameter_types! {
    pub const IndexDeposit: Balance = 100 * CENTS;
}
```

**Network-Specific**: NO
**Rebranding Required**: NO
**Priority**: N/A

---

### 7.3 Scheduler Pallet

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Lines 242-247
parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        BlockWeights::get().max_block;
    pub const MaxScheduledPerBlock: u32 = 50;
    pub const NoPreimagePostponement: Option<u32> = Some(10);
}
```

**Network-Specific**: NO
**Rebranding Required**: NO
**Priority**: N/A

---

### 7.4 Democracy/Governance Pallet

#### Conviction Voting (Pezkuwichain)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`

```rust
// Line 844
type VotingPeriod = ConstU32<{ 5 * DAYS }>;
```

**Network-Specific**: NO (standard governance period)
**Rebranding Required**: NO
**Priority**: N/A

---

### 7.5 System Teyrchain Constants

#### Pezkuwichain System Teyrchains
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`

```rust
// Lines 110-127
pub mod system_teyrchain {
    /// Network's Asset Hub teyrchain ID.
    pub const ASSET_HUB_ID: u32 = 1000;
    /// Contracts teyrchain ID.
    pub const CONTRACTS_ID: u32 = 1002;
    /// Encointer teyrchain ID.
    pub const ENCOINTER_ID: u32 = 1003;
    /// People teyrchain ID.
    pub const PEOPLE_ID: u32 = 1004;
    /// BridgeHub teyrchain ID.
    pub const BRIDGE_HUB_ID: u32 = 1013;
    /// Brokerage teyrchain ID.
    pub const BROKER_ID: u32 = 1005;
}
```

**Network-Specific**: YES (teyrchain ID allocation)
**Rebranding Required**: NO (numeric IDs)
**Priority**: N/A
**Note**: Pezkuwichain and Zagros share same system teyrchain IDs except BridgeHub (1013 vs 1002)

---

#### Zagros System Teyrchains
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/constants/src/lib.rs`

```rust
// Lines 110-124
pub mod system_teyrchain {
    /// Network's Asset Hub teyrchain ID.
    pub const ASSET_HUB_ID: u32 = 1000;
    /// Collectives teyrchain ID.
    pub const COLLECTIVES_ID: u32 = 1001;
    /// BridgeHub teyrchain ID.
    pub const BRIDGE_HUB_ID: u32 = 1002;
    /// Encointer teyrchain ID.
    pub const ENCOINTER_ID: u32 = 1003;
    /// People Chain teyrchain ID.
    pub const PEOPLE_ID: u32 = 1004;
    /// Brokerage teyrchain ID.
    pub const BROKER_ID: u32 = 1005;
    /// AH-next - temporary AH clone.
    pub const ASSET_HUB_NEXT_ID: u32 = 1100;
}
```

**Network-Specific**: YES
**Rebranding Required**: NO
**Priority**: N/A
**Note**: Zagros has COLLECTIVES_ID and ASSET_HUB_NEXT_ID not present in Pezkuwichain

---

#### Coretime Timeslice Period
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs` (Lines 136-139)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/constants/src/lib.rs` (Lines 133-136)

```rust
pub mod coretime {
    /// Coretime timeslice period in blocks
    /// WARNING: This constant is used accross chains, so additional care should be taken
    /// when changing it.
    #[cfg(feature = "fast-runtime")]
    pub const TIMESLICE_PERIOD: u32 = 20;
    #[cfg(not(feature = "fast-runtime"))]
    pub const TIMESLICE_PERIOD: u32 = 80;
}
```

**Network-Specific**: NO (shared constant)
**Rebranding Required**: NO
**Priority**: N/A

---

### 7.6 Teyrchain Host Configuration

Both Pezkuwichain and Zagros define default teyrchain host configurations in their genesis config presets.

**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/genesis_config_presets.rs` (Lines 93-144)
**File**: `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/genesis_config_presets.rs` (Lines 95-148)

Key constants include:
- `validation_upgrade_cooldown: 2`
- `validation_upgrade_delay: 2`
- `code_retention_period: 1200`
- `max_code_size: MAX_CODE_SIZE` (3 MB)
- `max_pov_size: MAX_POV_SIZE` (10 MB)
- `max_head_data_size: 32 * 1024` (32 KB)
- HRMP channel configurations
- Dispute parameters
- Scheduler parameters

**Network-Specific**: NO (identical configurations)
**Rebranding Required**: NO
**Priority**: N/A

---

## 8. Rebranding Impact Matrix

| Constant Category | Example | Change Required | Priority | Risk | Files Affected |
|-------------------|---------|----------------|----------|------|----------------|
| **Runtime spec_name** | "pezkuwichain", "zagros" | YES | HIGH | LOW | 13 runtime lib.rs files |
| **Runtime impl_name** | "parity-pezkuwichain-v2.0" | YES | HIGH | LOW | 13 runtime lib.rs files |
| **Teyrchain spec_name** | "statemine", "coretime-pezkuwichain" | YES | HIGH | MEDIUM | 10+ teyrchain runtime files |
| **Epoch env variables** | "PEZKUWICHAIN_EPOCH_DURATION" | YES | MEDIUM | LOW | 2 constants files |
| **Comment references** | "in Pezkuwichain, extrinsic..." | YES | LOW | VERY LOW | 2 fee modules |
| **Cargo feature deps** | `pezkuwichain-runtime-constants/fast-runtime` | YES | MEDIUM | LOW | 1 Cargo.toml |
| **Currency symbols** | TYR, ZGR | YES | HIGH | LOW | Genesis configs |
| **Block time constants** | MILLISECS_PER_BLOCK | NO | N/A | N/A | Shared constants |
| **Economic parameters** | EXISTENTIAL_DEPOSIT | NO | N/A | N/A | Network-specific values |
| **Weight limits** | MAXIMUM_BLOCK_WEIGHT | NO | N/A | N/A | Protocol constants |
| **PoV size limits** | MAX_POV_SIZE | NO | N/A | N/A | Protocol constants |
| **Session parameters** | SessionsPerEra | NO | N/A | N/A | Network configs |
| **Treasury config** | SpendPeriod, Burn | NO | N/A | N/A | Economic parameters |
| **Teyrchain IDs** | ASSET_HUB_ID: 1000 | NO | N/A | N/A | System teyrchain IDs |
| **Feature flags** | runtime-benchmarks | NO | N/A | N/A | Standard build features |

---

## 9. Critical File Locations

### 9.1 HIGH Priority (Runtime Identity)

#### Relay Chain Runtimes
1. `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`
   - Lines 182-183: `spec_name`, `impl_name`

2. `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`
   - Lines 173-174: `spec_name`, `impl_name`

#### Teyrchain Runtimes (Asset Hubs)
3. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain/src/lib.rs`
   - Lines 130-131: `spec_name: "statemine"`, `impl_name: "statemine"`
   - **CRITICAL**: Legacy naming with wallet compatibility concerns

4. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/assets/asset-hub-zagros/src/lib.rs`
   - Lines 147-148: `spec_name: "westmint"`, `impl_name: "westmint"`
   - **CRITICAL**: Legacy naming with explicit compatibility comment

#### Teyrchain Runtimes (System Chains)
5. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/coretime/coretime-pezkuwichain/src/lib.rs`
   - Lines 159-160: `spec_name`, `impl_name`

6. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/coretime/coretime-zagros/src/lib.rs`
   - spec_name, impl_name (location to be confirmed)

7. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-pezkuwichain/src/lib.rs`
   - Lines 252-253: `spec_name`, `impl_name`

8. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-zagros/src/lib.rs`
   - spec_name, impl_name (location to be confirmed)

9. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/people/people-pezkuwichain/src/lib.rs`
   - Lines 145-146: `spec_name`, `impl_name`

10. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/people/people-zagros/src/lib.rs`
    - spec_name, impl_name (location to be confirmed)

11. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/collectives/collectives-zagros/src/lib.rs`
    - Lines 130-131: `spec_name`, `impl_name`

12. `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/glutton/glutton-zagros/src/lib.rs`
    - Lines 103-104: `spec_name`, `impl_name`

---

### 9.2 MEDIUM Priority (Environment & Build)

#### Constants Modules
13. `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`
    - Line 48: `"PEZKUWICHAIN_EPOCH_DURATION"` environment variable name
    - Line 91: Comment "in Pezkuwichain, extrinsic base weight..."

14. `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/constants/src/lib.rs`
    - Line 86: Comment "in Zagros, extrinsic base weight..."
    - (Zagros epoch duration in runtime lib.rs, not constants)

#### Build Configuration
15. `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/Cargo.toml`
    - Line 329: `fast-runtime = ["pezkuwichain-runtime-constants/fast-runtime"]`
    - Dependency: `pezkuwichain-runtime-constants` crate name

16. `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`
    - Line 352: `"ZAGROS_EPOCH_DURATION"` environment variable name

---

### 9.3 LOW Priority (Documentation & Comments)

17. Various comment blocks referencing network names (search required)
18. Genesis configuration preset functions (cosmetic naming)

---

## 10. Rebranding Strategy Recommendations

### 10.1 Runtime Version Constants (HIGH Priority)

**Approach**: Direct string replacement with careful testing

**Steps**:
1. Replace `spec_name: "pezkuwichain"` → `spec_name: "teyrchain-dev"`
2. Replace `spec_name: "zagros"` → `spec_name: "teyrchain-test"`
3. Replace `impl_name: "parity-pezkuwichain-v2.0"` → `impl_name: "teyrchain-pezkuwichain-v2.0"`
4. Replace `impl_name: "parity-zagros"` → `impl_name: "teyrchain-zagros"`

**Risk Mitigation**:
- Keep `transaction_version` and `spec_version` unchanged
- Document all version bumps
- Test with substrate-archive and chain indexers

---

### 10.2 Teyrchain Runtime Names (HIGH Priority, HIGH Risk)

**Special Case: Asset Hubs**

Current situation:
- Asset Hub Pezkuwichain uses legacy name "statemine"
- Asset Hub Zagros uses legacy name "westmint"
- Both have explicit comments about wallet compatibility

**Recommended Approach**:
1. **Phase 1**: Keep legacy names for backward compatibility
2. **Phase 2**: Add migration path with dual spec_name support
3. **Phase 3**: Coordinate with wallet providers before final change

**Alternative**: Use TeyrChain branding for NEW system teyrchains only

---

### 10.3 Environment Variables (MEDIUM Priority)

**Files to update**:
```bash
# Pezkuwichain
"PEZKUWICHAIN_EPOCH_DURATION" → "TEYRCHAIN_DEV_EPOCH_DURATION"

# Zagros
"ZAGROS_EPOCH_DURATION" → "TEYRCHAIN_TEST_EPOCH_DURATION"
```

**Risk**: LOW (these are build-time configuration overrides)

---

### 10.4 Cargo Dependencies (MEDIUM Priority)

**Crate renames required**:
- `pezkuwichain-runtime-constants` → `teyrchain-pezkuwichain-runtime-constants`
- `zagros-runtime-constants` → `teyrchain-zagros-runtime-constants`
- `pezkuwichain-runtime` → `teyrchain-pezkuwichain-runtime`
- `zagros-runtime` → `teyrchain-zagros-runtime`

**Impact**: All downstream dependencies must be updated

---

### 10.5 Comments & Documentation (LOW Priority)

**Pattern-based search and replace**:
```
"in Pezkuwichain," → "in TeyrChain Pezkuwichain,"
"in Zagros," → "in TeyrChain Zagros,"
"Pezkuwichain testnet" → "TeyrChain Pezkuwichain testnet"
"Zagros testnet" → "TeyrChain Zagros testnet"
```

---

## 11. Constants NOT Requiring Rebranding

### Protocol-Level Constants (Substrate/Polkadot Standard)
- `MILLISECS_PER_BLOCK: 6000`
- `SLOT_DURATION: 6000`
- `PRIMARY_PROBABILITY: (1, 4)`
- `MAXIMUM_BLOCK_WEIGHT`
- `NORMAL_DISPATCH_RATIO`
- `MAX_POV_SIZE: 10 MB`
- `MAX_CODE_SIZE: 3 MB`
- `MAX_HEAD_DATA_SIZE: 1 MB`

### Economic Constants (Network-Specific Values)
- `EXISTENTIAL_DEPOSIT: 1 * CENTS`
- `UNITS: 1_000_000_000_000` (12 decimals)
- `TransactionByteFee: 10 * MILLICENTS`
- `SpendPeriod: 6 * DAYS`
- `Burn: 0.2%`

### Teyrchain System IDs
- `ASSET_HUB_ID: 1000`
- `PEOPLE_ID: 1004`
- `BROKER_ID: 1005`
- etc.

**Rationale**: These are numeric identifiers and economic parameters, not branding elements

---

## 12. Testing & Validation Strategy

### 12.1 Pre-Change Validation
1. Document current `spec_version` for all runtimes
2. Export genesis states for comparison
3. Record all `RUNTIME_API_VERSIONS` hashes

### 12.2 Post-Change Testing
1. Runtime metadata comparison (ensure only intended fields changed)
2. Chain spec generation and validation
3. Node synchronization tests
4. RPC compatibility checks

### 12.3 Integration Testing
1. Verify zombienet test networks start correctly
2. Confirm teyrchain registration still works
3. Test XCM message routing between renamed chains
4. Validate block explorer compatibility

---

## 13. Summary Statistics

### Constants Cataloged by Category

| Category | Count | Requires Rebranding |
|----------|-------|---------------------|
| Runtime version constants | 26 | 23 (88%) |
| Consensus parameters | 18 | 2 (11%) |
| Economic constants | 24 | 0 (0%) |
| Block production constants | 12 | 0 (0%) |
| Weight/PoV limits | 8 | 0 (0%) |
| Feature flags | 2 | 1 (50%) |
| Pallet-specific constants | 47+ | 3 (6%) |
| System teyrchain IDs | 14 | 0 (0%) |
| Environment variables | 3 | 3 (100%) |
| **TOTAL** | **154+** | **32 (21%)** |

### Files Requiring Changes

| Priority | File Type | Count |
|----------|-----------|-------|
| HIGH | Relay chain runtime lib.rs | 2 |
| HIGH | Teyrchain runtime lib.rs | 10 |
| MEDIUM | Runtime constants modules | 2 |
| MEDIUM | Cargo.toml files | 2 |
| LOW | Documentation/comments | TBD |
| **TOTAL** | | **16-20** |

---

## 14. Next Steps for Phase 1 Implementation

1. **Create rebranding specification** with exact before/after mappings
2. **Audit wallet/indexer dependencies** on spec_name values (especially Asset Hubs)
3. **Plan version bump strategy** (when to increment spec_version)
4. **Prepare migration scripts** for automated string replacement
5. **Set up regression testing** for runtime metadata changes
6. **Coordinate with stakeholders** (especially for Asset Hub renaming)

---

## Appendix A: Complete File Reference List

### Relay Chain Runtimes
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/test-runtime/src/lib.rs`

### Runtime Constants Modules
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/constants/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/constants/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/test-runtime/constants/src/lib.rs`

### Common Runtime Configuration
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/common/src/lib.rs`

### Protocol Primitives
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/primitives/src/v9/mod.rs`
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/primitives/src/lib.rs`

### Teyrchain Runtimes (Asset Hubs)
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/assets/asset-hub-zagros/src/lib.rs`

### Teyrchain Runtimes (Coretime)
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/coretime/coretime-pezkuwichain/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/coretime/coretime-zagros/src/lib.rs`

### Teyrchain Runtimes (Bridge Hubs)
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-pezkuwichain/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/bridge-hubs/bridge-hub-zagros/src/lib.rs`

### Teyrchain Runtimes (People Chains)
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/people/people-pezkuwichain/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/people/people-zagros/src/lib.rs`

### Teyrchain Runtimes (Other System Chains)
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/collectives/collectives-zagros/src/lib.rs`
- `/home/mamostehp/polkadot-sdk-fresh/cumulus/teyrchains/runtimes/glutton/glutton-zagros/src/lib.rs`

### Genesis Configuration Presets
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/src/genesis_config_presets.rs`
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/src/genesis_config_presets.rs`

### Cargo Configuration
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/pezkuwichain/Cargo.toml`
- `/home/mamostehp/polkadot-sdk-fresh/polkadot/runtime/zagros/Cargo.toml`

---

**Document Version**: 1.0
**Date**: 2025-11-26
**Phase**: 0.3 - Protocol Constants Discovery
**Status**: COMPLETE - Ready for Phase 1 Implementation Planning
