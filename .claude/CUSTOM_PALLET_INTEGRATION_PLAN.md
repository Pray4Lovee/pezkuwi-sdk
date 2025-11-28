# Pezkuwi Pallet Integration Plan v3.0

**Date:** 2025-11-28
**Status:** MASTER PLAN - Follow Exactly
**Goal:** Clean start with correct architecture

---

## BÖLÜM 1: MİMARİ PRENSİPLER

### 1.1 Temel Kural

```
RELAY CHAIN = Konsensüs + Güvenlik + Parachain Koordinasyonu
PARACHAIN   = Uygulama Mantığı
```

### 1.2 Polkadot Referans Mimarisi

Polkadot'un kendisi bu yapıyı kullanıyor:

| Chain | Amaç | Örnek Palletler |
|-------|------|-----------------|
| **Relay Chain** | Güvenlik, konsensüs, parachain koordinasyonu | staking, session, babe, grandpa, paras_* |
| **Asset Hub** | Tüm asset/token/NFT işlemleri | assets, nfts, asset_conversion |
| **People** | Kimlik ve kişisel veriler | identity |
| **Collectives** | Teknik governance, fellowship | ranked_collective, fellowship_* |
| **Bridge Hub** | Cross-chain köprüler | bridge_*, ethereum_* |

---

## BÖLÜM 2: PEZKUWI MİMARİSİ

### 2.1 Tam Mimari Diyagram

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          PEZKUWICHAIN (Relay Chain)                              │
│                              HEZ Native Token                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│   KONSENSÜS & GÜVENLİK:                                                         │
│   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│   │    Babe      │ │   Grandpa    │ │   Session    │ │   Staking    │          │
│   └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘          │
│   ┌──────────────┐ ┌──────────────┐                                             │
│   │ NominationPools│ │FastUnstake │  + validator-pool (TNPoS Shadow Mode)       │
│   └──────────────┘ └──────────────┘                                             │
│                                                                                  │
│   TEMEL:                                                                         │
│   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│   │   System     │ │  Balances    │ │  Timestamp   │ │   Indices    │          │
│   │              │ │    (HEZ)     │ │              │ │              │          │
│   └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘          │
│                                                                                  │
│   GOVERNANCE (HEZ için):                                                         │
│   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│   │  Treasury    │ │  Referenda   │ │ConvictionVote│ │  Whitelist   │          │
│   └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘          │
│                                                                                  │
│   PARACHAIN KOORDİNASYONU:                                                       │
│   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│   │    Paras     │ │ ParaInclusion│ │     Hrmp     │ │   XcmPallet  │          │
│   └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘          │
│                                                                                  │
│   UTILITY:                                                                       │
│   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│   │   Utility    │ │    Proxy     │ │   Multisig   │ │  Scheduler   │          │
│   └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘          │
│                                                                                  │
│   ❌ ÇIKARILACAKLAR (Asset Hub'a taşınacak):                                    │
│   Assets, Nfts, PoolAssets, AssetRate, AssetConversion, Nis                     │
│                                                                                  │
│   ❌ ÇIKARILACAKLAR (People'a taşınacak):                                       │
│   Identity, Society, Recovery                                                    │
│                                                                                  │
│   ❌ ÇIKARILACAKLAR (Collectives'e taşınacak):                                  │
│   Bounties, ChildBounties, FellowshipCollective, FellowshipReferenda            │
│                                                                                  │
└──────────────────────────────────────┬──────────────────────────────────────────┘
                                       │
                                       │ XCM
                                       │
        ┌──────────────────────────────┼──────────────────────────────┐
        │                              │                              │
        ▼                              ▼                              ▼
┌───────────────────────┐  ┌───────────────────────┐  ┌───────────────────────┐
│   PEOPLE-PEZKUWICHAIN │  │ ASSET-HUB-PEZKUWICHAIN│  │ COLLECTIVES-PEZKUWICHAIN│
│      (Parachain)      │  │      (Parachain)      │  │      (Parachain)      │
├───────────────────────┤  ├───────────────────────┤  ├───────────────────────┤
│                       │  │                       │  │                       │
│ STANDART:             │  │ STANDART:             │  │ STANDART:             │
│ ├─ Identity           │  │ ├─ Assets             │  │ ├─ FellowshipCollective│
│ ├─ (migrated from     │  │ ├─ Nfts               │  │ ├─ FellowshipReferenda │
│ │   relay)            │  │ ├─ PoolAssets         │  │ ├─ FellowshipTreasury │
│ │                     │  │ ├─ AssetConversion    │  │ ├─ Alliance           │
│ │                     │  │ ├─ Nis                │  │ │                     │
│ │                     │  │ │                     │  │ │                     │
│ CUSTOM PEZKUWI:       │  │ CUSTOM PEZKUWI:       │  │ CUSTOM PEZKUWI:       │
│ ├─ identity-kyc       │  │ ├─ pez-treasury       │  │ ├─ welati (parliament)│
│ ├─ tiki (NFT badges)  │  │ ├─ pez-rewards        │  │ │                     │
│ ├─ trust              │  │ ├─ presale            │  │ │                     │
│ ├─ referral           │  │ ├─ staking-score      │  │ │                     │
│ ├─ perwerde           │  │ ├─ token-wrapper      │  │ │                     │
│ │                     │  │ │                     │  │ │                     │
└───────────────────────┘  └───────────────────────┘  └───────────────────────┘
```

### 2.2 Token Dağılımı

| Token | Chain | Pallet | Açıklama |
|-------|-------|--------|----------|
| **HEZ** | Relay Chain | pallet-balances | Native gas token, DOT eşdeğeri |
| **PEZ** | Asset Hub | pallet-assets veya native | Governance token, fixed supply |
| **Tiki NFTs** | People | pallet-nfts + pallet-tiki | Vatandaşlık rozetleri |

---

## BÖLÜM 3: RELAY CHAIN TEMİZLİĞİ

### 3.1 Çıkarılacak Palletler

**Asset Hub'a Taşınacak:**
```rust
// KALDIRILACAK - pezkuwichain/src/lib.rs'den
Assets: pallet_assets::<Instance1> = 36,        // ❌ KALDIR
Nfts: pallet_nfts = 37,                         // ❌ KALDIR
PoolAssets: pallet_assets::<Instance2> = 16,    // ❌ KALDIR
AssetRate: pallet_asset_rate = 39,              // ❌ KALDIR
AssetConversion: pallet_asset_conversion = 11,  // ❌ KALDIR
Nis: pallet_nis = 38,                           // ❌ KALDIR
NisCounterpartBalances: pallet_balances::<Instance2> = 45, // ❌ KALDIR
```

**People Parachain'e Taşınacak:**
```rust
// KALDIRILACAK - zaten IdentityMigrator ile taşınacak
Identity: pallet_identity = 25,                 // ❌ KALDIR (migrator var)
Society: pallet_society = 26,                   // ❌ KALDIR
Recovery: pallet_recovery = 27,                 // ❌ KALDIR
```

**Collectives Parachain'e Taşınacak:**
```rust
// KALDIRILACAK
Bounties: pallet_bounties = 35,                 // ❌ KALDIR
ChildBounties: pallet_child_bounties = 40,      // ❌ KALDIR
```

### 3.2 Kalacak Palletler (Relay Chain)

```rust
construct_runtime! {
    pub enum Runtime
    {
        // === TEMEL SİSTEM ===
        System: frame_system = 0,
        Timestamp: pallet_timestamp = 2,
        Indices: pallet_indices = 3,
        Balances: pallet_balances = 4,              // HEZ native token
        Parameters: pallet_parameters = 6,
        TransactionPayment: pallet_transaction_payment = 33,

        // === KONSENSÜS ===
        Babe: pallet_babe = 1,
        Authorship: pallet_authorship = 5,
        Offences: pallet_offences = 7,
        Historical: session_historical = 34,
        Session: pallet_session = 8,
        Staking: pallet_staking = 9,                // NPoS
        Grandpa: pallet_grandpa = 10,
        AuthorityDiscovery: pallet_authority_discovery = 12,
        NominationPools: pallet_nomination_pools = 14,
        FastUnstake: pallet_fast_unstake = 15,
        VoterBagsList: pallet_bags_list::<Instance1> = 100,

        // === GOVERNANCE (HEZ için) ===
        Council: pallet_collective::<Instance1> = 17,
        Treasury: pallet_treasury = 18,             // HEZ treasury
        ConvictionVoting: pallet_conviction_voting = 20,
        Referenda: pallet_referenda = 21,
        Origins: pallet_custom_origins = 43,
        Whitelist: pallet_whitelist = 44,
        Claims: claims = 19,

        // === UTILITY ===
        Utility: pallet_utility = 24,
        Vesting: pallet_vesting = 28,
        Scheduler: pallet_scheduler = 29,
        Proxy: pallet_proxy = 30,
        Multisig: pallet_multisig = 31,
        Preimage: pallet_preimage = 32,

        // === TEYRCHAIN (PARACHAIN) KOORDİNASYONU ===
        TeyrchainsOrigin: teyrchains_origin = 50,
        Configuration: teyrchains_configuration = 51,
        ParasShared: teyrchains_shared = 52,
        ParaInclusion: teyrchains_inclusion = 53,
        ParaInherent: teyrchains_paras_inherent = 54,
        ParaScheduler: teyrchains_scheduler = 55,
        Paras: teyrchains_paras = 56,
        Initializer: teyrchains_initializer = 57,
        Dmp: teyrchains_dmp = 58,
        Hrmp: teyrchains_hrmp = 60,
        ParaSessionInfo: teyrchains_session_info = 61,
        ParasDisputes: teyrchains_disputes = 62,
        ParasSlashing: teyrchains_slashing = 63,
        MessageQueue: pallet_message_queue = 64,
        OnDemandAssignmentProvider: teyrchains_on_demand = 66,
        CoretimeAssignmentProvider: teyrchains_assigner_coretime = 68,

        // === TEYRCHAIN ONBOARDING ===
        Registrar: paras_registrar = 70,
        Slots: slots = 71,
        Auctions: auctions = 72,
        Crowdloan: crowdloan = 73,
        Coretime: coretime = 74,

        // === XCM ===
        XcmPallet: pallet_xcm = 99,

        // === BRIDGE (BEEFY) ===
        Beefy: pallet_beefy = 240,
        Mmr: pallet_mmr = 241,
        MmrLeaf: pallet_beefy_mmr = 242,

        // === MİGRASYON (geçici) ===
        IdentityMigrator: identity_migrator = 248,
        MultiBlockMigrations: pallet_migrations = 98,
        StateTrieMigration: pallet_state_trie_migration = 254,

        // === DEV/TEST ===
        ParasSudoWrapper: paras_sudo_wrapper = 250,
        AssignedSlots: assigned_slots = 251,
        ValidatorManager: validator_manager = 252,
        RootTesting: pallet_root_testing = 249,
        Sudo: pallet_sudo = 255,

        // === CUSTOM PEZKUWI (SADECE BU) ===
        ValidatorPool: pallet_validator_pool = 91,  // TNPoS Shadow Mode
    }
}
```

---

## BÖLÜM 4: PARACHAIN ENTEGRASYONU

### 4.1 People-Pezkuwichain

**Konum:** `cumulus/teyrchains/runtimes/people/people-pezkuwichain/`

**Mevcut Palletler (kalacak):**
- System, TeyrchainSystem, Timestamp, TeyrchainInfo
- Balances, TransactionPayment
- Collator pallets (Authorship, CollatorSelection, Session, Aura, AuraExt)
- XCM pallets (XcmpQueue, PezkuwiXcm, CumulusXcm, MessageQueue)
- Utility, Multisig, Proxy
- Identity (migrated from relay)
- IdentityMigrator

**Eklenecek Custom Palletler:**
```rust
// people-pezkuwichain/src/lib.rs - construct_runtime! içine

// === CUSTOM PEZKUWI PALLETS ===
IdentityKyc: pallet_identity_kyc = 51,
Tiki: pallet_tiki = 52,
Trust: pallet_trust = 53,
Referral: pallet_referral = 54,
Perwerde: pallet_perwerde = 55,
```

**Bağımlılık Sırası:**
```
1. identity-kyc (foundation - no deps)
2. perwerde (standalone)
3. referral (depends on identity-kyc)
4. tiki (depends on identity-kyc + pallet-nfts)
5. trust (aggregates all above)
```

### 4.2 Asset-Hub-Pezkuwichain

**Konum:** `cumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain/`

**Mevcut Palletler (kalacak):**
- System, TeyrchainSystem, Timestamp, TeyrchainInfo
- Balances, TransactionPayment, AssetTxPayment
- Collator pallets
- XCM pallets
- Utility, Multisig, Proxy
- Assets, Nfts, PoolAssets, ForeignAssets
- AssetConversion, AssetsFreezer, etc.

**Eklenecek Custom Palletler:**
```rust
// asset-hub-pezkuwichain/src/lib.rs - construct_runtime! içine

// === CUSTOM PEZKUWI PALLETS ===
StakingScore: pallet_staking_score = 60,
TokenWrapper: pallet_token_wrapper = 61,
PezTreasury: pallet_pez_treasury = 62,
PezRewards: pallet_pez_rewards = 63,
Presale: pallet_presale = 64,
```

**Bağımlılık Sırası:**
```
1. staking-score (reads from relay via XCM)
2. token-wrapper (standalone)
3. pez-treasury (manages PEZ)
4. presale (uses treasury)
5. pez-rewards (uses treasury + needs trust from People via XCM)
```

### 4.3 Collectives-Pezkuwichain

**Konum:** `cumulus/teyrchains/runtimes/collectives/`

**Not:** Şu an `collectives-zagros` var, `collectives-pezkuwichain` oluşturulmalı.

**Eklenecek Custom Palletler:**
```rust
// === CUSTOM PEZKUWI PALLETS ===
Welati: pallet_welati = 70,  // Parliament + Elections
```

---

## BÖLÜM 5: DOSYA KONUMLARI

### 5.1 Pallet Dizin Yapısı

```
Pezkuwi-SDK/
├── pezkuwi/
│   └── pallets/
│       └── validator-pool/              # ✅ Relay chain ONLY
│
└── cumulus/
    └── teyrchains/
        └── pallets/                     # YENİ - tüm custom palletler buraya
            ├── identity-kyc/            # People parachain
            ├── tiki/                    # People parachain
            ├── trust/                   # People parachain
            ├── referral/                # People parachain
            ├── perwerde/                # People parachain
            ├── pez-treasury/            # Asset Hub parachain
            ├── pez-rewards/             # Asset Hub parachain
            ├── presale/                 # Asset Hub parachain
            ├── staking-score/           # Asset Hub parachain
            ├── token-wrapper/           # Asset Hub parachain
            └── welati/                  # Collectives parachain
```

### 5.2 Cargo.toml Değişiklikleri

**Workspace Root (Cargo.toml):**
```toml
[workspace]
members = [
    # Existing...

    # Relay Chain Custom Pallet
    "pezkuwi/pallets/validator-pool",

    # Parachain Custom Pallets
    "cumulus/teyrchains/pallets/identity-kyc",
    "cumulus/teyrchains/pallets/tiki",
    "cumulus/teyrchains/pallets/trust",
    "cumulus/teyrchains/pallets/referral",
    "cumulus/teyrchains/pallets/perwerde",
    "cumulus/teyrchains/pallets/pez-treasury",
    "cumulus/teyrchains/pallets/pez-rewards",
    "cumulus/teyrchains/pallets/presale",
    "cumulus/teyrchains/pallets/staking-score",
    "cumulus/teyrchains/pallets/token-wrapper",
    "cumulus/teyrchains/pallets/welati",
]

[workspace.dependencies]
# Relay Chain
pallet-validator-pool = { path = "pezkuwi/pallets/validator-pool", default-features = false }

# Parachain Pallets
pallet-identity-kyc = { path = "cumulus/teyrchains/pallets/identity-kyc", default-features = false }
pallet-tiki = { path = "cumulus/teyrchains/pallets/tiki", default-features = false }
pallet-trust = { path = "cumulus/teyrchains/pallets/trust", default-features = false }
pallet-referral = { path = "cumulus/teyrchains/pallets/referral", default-features = false }
pallet-perwerde = { path = "cumulus/teyrchains/pallets/perwerde", default-features = false }
pallet-pez-treasury = { path = "cumulus/teyrchains/pallets/pez-treasury", default-features = false }
pallet-pez-rewards = { path = "cumulus/teyrchains/pallets/pez-rewards", default-features = false }
pallet-presale = { path = "cumulus/teyrchains/pallets/presale", default-features = false }
pallet-staking-score = { path = "cumulus/teyrchains/pallets/staking-score", default-features = false }
pallet-token-wrapper = { path = "cumulus/teyrchains/pallets/token-wrapper", default-features = false }
pallet-welati = { path = "cumulus/teyrchains/pallets/welati", default-features = false }
```

---

## BÖLÜM 6: XCM KOMÜNİKASYONU

### 6.1 Relay ↔ People Parachain

**validator-pool trust/tiki verisi gerektirir:**

```rust
// Shadow Mode için stub implementation (başlangıç):
pub struct StubTrustProvider;
impl TrustScoreProvider<AccountId> for StubTrustProvider {
    fn trust_score_of(_who: &AccountId) -> u128 {
        1000 // Default trust for shadow mode
    }
}

// Phase 2: XCM cache
pub struct XcmTrustCache;
impl TrustScoreProvider<AccountId> for XcmTrustCache {
    fn trust_score_of(who: &AccountId) -> u128 {
        // People parachain'den periyodik push ile güncellenen cache
        TrustScoreCache::<Runtime>::get(who).unwrap_or(0)
    }
}
```

### 6.2 Asset Hub ↔ People Parachain

**pez-rewards trust score gerektirir:**

```rust
// pez-rewards dağıtımı için citizenship check
impl pallet_pez_rewards::Config for Runtime {
    type CitizenshipChecker = XcmCitizenshipBridge;
}

pub struct XcmCitizenshipBridge;
impl CitizenshipChecker<AccountId> for XcmCitizenshipBridge {
    fn is_citizen(who: &AccountId) -> bool {
        // XCM query to People parachain
    }
}
```

---

## BÖLÜM 7: UYGULAMA PLANI

### Phase 0: Relay Chain Temizliği

**Adım 0.1:** Yanlış yerleştirilmiş palletleri kaldır
```bash
# pezkuwichain/src/lib.rs'den kaldır:
- Assets, Nfts, PoolAssets, AssetRate, AssetConversion
- Nis, NisCounterpartBalances
- Identity, Society, Recovery
- Bounties, ChildBounties
```

**Adım 0.2:** Yanlış yorum satırlarını sil
```bash
# Kaldır (satır 2038-2053):
// PHASE 2 - Custom Pezkuwi Pallets yorumları
```

**Adım 0.3:** Build ve test
```bash
cargo check -p pezkuwichain-runtime
```

### Phase 1: Relay Chain - validator-pool

**Adım 1.1:** validator-pool'u runtime'a ekle
```rust
// pezkuwichain/src/lib.rs
ValidatorPool: pallet_validator_pool = 91,
```

**Adım 1.2:** Config trait implement et (stub providers ile)
```rust
impl pallet_validator_pool::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_validator_pool::WeightInfo<Runtime>;
    type Randomness = Babe;
    type TrustSource = StubTrustProvider;      // Stub
    type TikiSource = StubTikiProvider;        // Stub
    type ReferralSource = StubReferralProvider; // Stub
    type PerwerdeSource = StubPerwerdeProvider; // Stub
    type PoolManagerOrigin = EnsureRoot<AccountId>;
    type MaxValidators = ConstU32<1000>;
    type MaxPoolSize = ConstU32<5000>;
    type MinStakeAmount = ConstU128<{ 100 * UNITS }>;
}
```

**Adım 1.3:** Build ve test
```bash
cargo build -p pezkuwichain-runtime --release
cargo test -p pezkuwichain-runtime
```

### Phase 2: People Parachain

**Adım 2.1:** Pallet dizini oluştur
```bash
mkdir -p cumulus/teyrchains/pallets
```

**Adım 2.2:** Palletleri kopyala (SDK-2'den)
```bash
cp -r SDK-2/pezkuwi/pallets/identity-kyc cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/perwerde cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/referral cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/tiki cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/trust cumulus/teyrchains/pallets/
```

**Adım 2.3:** sp_std → alloc düzelt (her pallet için)
```bash
# Her pallet'in lib.rs ve types.rs dosyasında:
# sp_std::vec::Vec → alloc::vec::Vec
# extern crate alloc; ekle
```

**Adım 2.4:** Cargo.toml path'leri düzelt

**Adım 2.5:** people-pezkuwichain runtime'a entegre et

**Adım 2.6:** Build ve test
```bash
cargo build -p people-pezkuwichain-runtime --release
cargo test -p people-pezkuwichain-runtime
```

### Phase 3: Asset Hub Parachain

**Adım 3.1:** Palletleri kopyala
```bash
cp -r SDK-2/pezkuwi/pallets/staking-score cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/token-wrapper cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/pez-treasury cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/pez-rewards cumulus/teyrchains/pallets/
cp -r SDK-2/pezkuwi/pallets/presale cumulus/teyrchains/pallets/
```

**Adım 3.2:** sp_std → alloc düzelt

**Adım 3.3:** asset-hub-pezkuwichain runtime'a entegre et

**Adım 3.4:** Build ve test
```bash
cargo build -p asset-hub-pezkuwichain-runtime --release
cargo test -p asset-hub-pezkuwichain-runtime
```

### Phase 4: Collectives Parachain

**Adım 4.1:** collectives-pezkuwichain oluştur (zagros'tan kopyala)

**Adım 4.2:** welati pallet'i kopyala ve entegre et

**Adım 4.3:** Build ve test

### Phase 5: XCM Entegrasyonu

**Adım 5.1:** Trust score cache mekanizması tasarla

**Adım 5.2:** People → Relay XCM push implement et

**Adım 5.3:** Asset Hub → People XCM query implement et

**Adım 5.4:** End-to-end test

### Phase 6: Final Test

**Adım 6.1:** Local relay chain + parachains başlat

**Adım 6.2:** XCM mesajları test et

**Adım 6.3:** validator-pool shadow mode test et

**Adım 6.4:** Full integration test

---

## BÖLÜM 8: CHECKLIST

### Phase 0: Relay Temizliği
- [ ] Assets, Nfts, PoolAssets kaldır
- [ ] AssetRate, AssetConversion kaldır
- [ ] Nis, NisCounterpartBalances kaldır
- [ ] Identity, Society, Recovery kaldır
- [ ] Bounties, ChildBounties kaldır
- [ ] Yorum satırlarını (2038-2053) sil
- [ ] Cargo.toml dependencies güncelle
- [ ] Build geçiyor
- [ ] Test geçiyor

### Phase 1: Relay - validator-pool
- [ ] validator-pool runtime'a ekle
- [ ] Stub providers implement et
- [ ] Genesis config ekle
- [ ] Weights dosyası ekle
- [ ] Build geçiyor
- [ ] Test geçiyor

### Phase 2: People Parachain
- [ ] cumulus/teyrchains/pallets/ dizini oluştur
- [ ] identity-kyc kopyala ve düzelt
- [ ] perwerde kopyala ve düzelt
- [ ] referral kopyala ve düzelt
- [ ] tiki kopyala ve düzelt
- [ ] trust kopyala ve düzelt
- [ ] Workspace Cargo.toml güncelle
- [ ] people-pezkuwichain runtime güncelle
- [ ] Build geçiyor
- [ ] Test geçiyor

### Phase 3: Asset Hub Parachain
- [ ] staking-score kopyala ve düzelt
- [ ] token-wrapper kopyala ve düzelt
- [ ] pez-treasury kopyala ve düzelt
- [ ] pez-rewards kopyala ve düzelt
- [ ] presale kopyala ve düzelt
- [ ] Workspace Cargo.toml güncelle
- [ ] asset-hub-pezkuwichain runtime güncelle
- [ ] Build geçiyor
- [ ] Test geçiyor

### Phase 4: Collectives Parachain
- [ ] collectives-pezkuwichain oluştur
- [ ] welati kopyala ve düzelt
- [ ] Runtime güncelle
- [ ] Build geçiyor
- [ ] Test geçiyor

### Phase 5: XCM
- [ ] Trust cache mekanizması
- [ ] People → Relay push
- [ ] Asset Hub → People query
- [ ] Test geçiyor

### Phase 6: Final
- [ ] Local network test
- [ ] XCM test
- [ ] Shadow mode test
- [ ] Full integration test

---

## BÖLÜM 9: NOTLAR

### Önemli Kurallar

1. **Her adımda build ve test yap** - Kırık kod commit etme
2. **Bağımlılık sırasına uy** - Layer 0 → 1 → 2 → 3
3. **sp_std kullanma** - Modern Polkadot SDK'da `alloc` kullan
4. **Stub ile başla** - XCM karmaşıklığını sonraya bırak

### Risk Azaltma

1. **Shadow Mode:** validator-pool aktif modda değil, güvenli test
2. **Stub Providers:** XCM olmadan çalışabilir başlangıç
3. **Incremental Build:** Her phase sonunda working state

---

**Document Version:** 3.0
**Architecture:** Clean Parachain-based
**Last Updated:** 2025-11-28
