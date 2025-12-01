# Pezkuwi Network Genesis & Launch Engineering Plan

Bu doküman, Pezkuwi Ağı'nın (Relay Chain, Asset Hub, People Chain) geliştirme aşamasından Mainnet'e kadar olan başlatma süreçlerini, konfigürasyon detaylarını ve test stratejilerini içerir.

**Son Güncelleme:** 2025-11-30
**Versiyon:** 2.2.0
**Durum:** FAZ 1 TAMAMLANDI ✅ | BENCHMARK'LAR TAMAMLANDI ✅ | FAZ 2 BEKLEMEDE

---

## İÇİNDEKİLER

1. [Genel Bakış ve Hedefler](#1-genel-bakış-ve-hedefler)
2. [Tokenomik Mimarisi](#2-tokenomik-mimarisi)
3. [Ağ Topolojisi](#3-ağ-topolojisi)
4. [Aşamalı Uygulama Planı](#4-aşamalı-uygulama-planı)
5. [Teknik Uygulama Detayları](#5-teknik-uygulama-detayları)
6. [Validator Key Yönetimi](#6-validator-key-yönetimi)
7. [Genesis Dosya Haritası](#7-genesis-dosya-haritası)
8. [Test ve Doğrulama Stratejisi](#8-test-ve-doğrulama-stratejisi)
9. [Güvenlik Kontrol Listesi](#9-güvenlik-kontrol-listesi)
10. [Uygulama Takvimi](#10-uygulama-takvimi)
11. [Acil Durum Prosedürleri](#11-acil-durum-prosedürleri)

---

## 1. Genel Bakış ve Hedefler

### 1.1. Misyon
Pezkuwi Network'ün güvenli, tutarlı ve ölçeklenebilir şekilde başlatılmasını sağlamak.

### 1.2. Kritik Başarı Faktörleri

| # | Faktör | Açıklama | Öncelik |
|---|--------|----------|---------|
| 1 | **Dual Token Tutarlılığı** | HEZ (native) ve PEZ (asset) tokenlarının tüm aşamalarda doğru genesis durumu | 🔴 KRİTİK |
| 2 | **Asset Varlığı** | Asset Hub'da PEZ (ID:1) ve wHEZ (ID:2) genesis'te mevcut | 🔴 KRİTİK |
| 3 | **Kimlik Bootstrap** | People Chain'de Founder vatandaş ile IdentityKyc başlatma | 🔴 KRİTİK |
| 4 | **Validator Seti** | Her aşama için doğru validator sayısı ve key'ler | 🔴 KRİTİK |
| 5 | **Treasury Başlatma** | PezTreasury pallet'inin 5B PEZ ile başlaması | 🟡 YÜKSEK |
| 6 | **Sentetik Halving** | 48 aylık halving mekanizmasının doğru konfigürasyonu | 🟡 YÜKSEK |

### 1.3. Hedef Ağ Yapısı

```
┌─────────────────────────────────────────────────────────────┐
│                    PEZKUWICHAIN RELAY CHAIN                  │
│                     (100 Validators - Mainnet)               │
│                                                              │
│  Native Token: HEZ (Enflasyonist)                           │
│  Konsensüs: TNPoS (Trust-enhanced NPoS)                     │
│  Blok Süresi: ~6 saniye                                     │
└─────────────────────┬───────────────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│  ASSET HUB    │ │ PEOPLE CHAIN  │ │ BRIDGE HUB    │
│  (ParaId:1000)│ │ (ParaId:1004) │ │ (ParaId:1002) │
├───────────────┤ ├───────────────┤ ├───────────────┤
│ PEZ (ID:1)    │ │ IdentityKyc   │ │ XCM Bridge    │
│ wHEZ (ID:2)   │ │ Welati (Gov)  │ │ Zagros Link   │
│ Presale       │ │ Perwerde      │ │               │
│ TokenWrapper  │ │ Trust         │ │               │
│ PezTreasury   │ │               │ │               │
└───────────────┘ └───────────────┘ └───────────────┘
```

---

## 2. Tokenomik Mimarisi

### 2.1. Dual Token Sistemi

#### HEZ Token (Native - Relay Chain)
| Özellik | Değer |
|---------|-------|
| Tip | Native Balance (Enflasyonist) |
| **Genesis Arzı** | **200,000,000 HEZ** (200M) |
| Enflasyon | Era başına dinamik NPoS (~%10/yıl hedef) |
| Kullanım | Gas, Staking, İşlem Ücretleri |
| Decimal | 12 |
| Birim | 1 HEZ = 10^12 Planck |

### 2.2. HEZ Genesis Dağılımı (Polkadot Modeli)

```
┌────────────────────────────────────────────────────────────────┐
│                    HEZ GENESIS ARZ: 200,000,000                │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ██████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Founder       10%   │
│  ██████████████████████████████████████░░  Presale      50%   │
│  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░  Gov Treasury  20%   │
│  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░  Airdrop       20%   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

| Kategori | Yüzde | Miktar (HEZ) | Hesap | Açıklama |
|----------|-------|--------------|-------|----------|
| Founder | 10% | 20,000,000 | Founder Account | Proje kurucusu |
| Presale | 50% | 100,000,000 | Presale Account | Erken yatırımcılar |
| Kurdistan Hazine Bakanlığı | 20% | 40,000,000 | Gov Treasury | Hükümet fonu |
| Airdrop | 20% | 40,000,000 | Airdrop Account | Topluluk dağıtımı |
| **TOPLAM** | **100%** | **200,000,000** | | |

> **Not:** Genesis sonrası NPoS enflasyonu ile HEZ arzı yıllık ~%10 oranında artacaktır.

#### PEZ Token (Asset - Asset Hub)
| Özellik | Değer |
|---------|-------|
| Asset ID | 1 |
| Tip | pallet_assets (Sabit Arz) |
| Toplam Arz | **5,000,000,000 PEZ** (5 Milyar) |
| Decimal | 12 |
| Birim | 1 PEZ = 10^12 Planck |
| Halving | Sentetik - 48 ayda bir %50 azalan dağıtım |

### 2.3. PEZ Token Dağıtımı (Whitepaper v2.0)

```
┌────────────────────────────────────────────────────────────────┐
│                    PEZ TOPLAM ARZ: 5,000,000,000               │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ████████████████████░░░░░░░░░░░░░░░░░░░░  Treasury    20.25%  │
│  ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Presale      1.875% │
│  ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Founder      1.875% │
│  ████████████████████████████████████████  Rewards     76.00%  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

| Kategori | Yüzde | Miktar (PEZ) | Vesting/Kilit |
|----------|-------|--------------|---------------|
| Treasury (Yönetişim) | 20.25% | 1,012,500,000 | Governance kontrolü |
| Presale | 1.875% | 93,750,000 | Presale pallet yönetimi |
| Founder | 1.875% | 93,750,000 | 4 yıl vesting |
| Validator/Nominator Rewards | 76.00% | 3,800,000,000 | Sentetik halving ile dağıtım |

### 2.4. Sentetik Halving Mekanizması

```rust
// pallet-pez-treasury: lib.rs
const HALVING_PERIOD_MONTHS: u32 = 48;
const INITIAL_EPOCH_REWARD: u128 = 79_166_666 * PEZ; // Ayda ~79M PEZ

fn calculate_epoch_reward(current_month: u32) -> u128 {
    let halving_count = current_month / HALVING_PERIOD_MONTHS;
    INITIAL_EPOCH_REWARD >> halving_count // Her 48 ayda yarıya düşer
}
```

**Dağıtım Takvimi:**
| Yıl | Aylık Dağıtım | Yıllık Toplam | Kümülatif |
|-----|---------------|---------------|-----------|
| 1-4 | ~79.17M PEZ | ~950M PEZ | ~3.8B PEZ |
| 5-8 | ~39.58M PEZ | ~475M PEZ | ~4.75B PEZ |
| 9-12 | ~19.79M PEZ | ~237.5M PEZ | ~4.99B PEZ |
| 13+ | Azalan | ... | ~5B PEZ |

---

## 3. Ağ Topolojisi

### 3.1. Validator Sayısı Yol Haritası

| Aşama | Validator Sayısı | Collator (Parachain) | Açıklama |
|-------|------------------|----------------------|----------|
| Dev | 1 | 1 | Tek node geliştirme |
| Local | 2 | 2 | Alice + Bob |
| Alpha | 4 | 2 | Çekirdek ekip testi |
| Beta | 8 | 4 | Gerçek key'lerle test |
| Staging | 21 | 8 | Performans testi |
| Mainnet | 100 | 16 | Canlı ağ başlangıcı |

### 3.2. Parachain ID'leri

| Parachain | ID | Durum | Güncelleme |
|-----------|----|-------|------------|
| Asset Hub Pezkuwichain | 1000 | ✅ Tanımlı | - |
| Bridge Hub Pezkuwichain | 1002 | ✅ Düzeltildi | **2025-11-30** (1013→1002) |
| People Pezkuwichain | 1004 | ✅ Tanımlı | - |
| Coretime/Broker | 1005 | ✅ Tanımlı | - |

---

## 4. Aşamalı Uygulama Planı

### Phase 1: Dev Network (Geliştirme Ortamı)
**Hedef:** Tek node'da tüm runtime mantığını doğrulamak
**Validator:** 1 (Alice)
**Preset:** `DEV_RUNTIME_PRESET`

#### 4.1.1. Relay Chain Gereksinimleri
- [x] Temel preset mevcut
- [x] HEZ genesis dağılımı (200M) implementasyonu ✅ **2025-11-30**
- [x] Genesis preset eklendi (Founder, Presale, Treasury, Airdrop hesapları)
- [ ] PezTreasury genesis başlatma
- [ ] PezRewards genesis başlatma

#### 4.1.2. Asset Hub Gereksinimleri
- [x] `AssetsConfig` (Instance1) eklendi ✅ **2025-11-30**
- [x] PEZ (ID:1) tanımı - Owner: Treasury, Sufficient: true ✅
- [x] wHEZ (ID:2) tanımı - Owner: Treasury, Sufficient: true ✅
- [x] Genesis preset eklendi (treasury/founder/presale parametreleri) ✅
- [ ] Presale pallet genesis (isteğe bağlı)

#### 4.1.3. People Chain Gereksinimleri
- [x] `IdentityKycConfig` genesis eklendi ✅ **2025-11-30**
- [x] Founding citizen mekanizması: Alice (dev için) ✅
- [x] Genesis preset eklendi ✅
- [ ] Trust pallet başlangıç durumu

#### 4.1.4. Test Komutu
```bash
# Relay Chain
./target/release/pezkuwi --dev

# Asset Hub (ayrı terminal)
./target/release/pezkuwi-omni-node --dev --chain=asset-hub-pezkuwichain-dev

# People Chain (ayrı terminal)
./target/release/pezkuwi-omni-node --dev --chain=people-pezkuwichain-dev
```

#### 4.1.5. Başarı Kriterleri
- [x] Tüm runtime'lar compile oluyor ✅ **2025-11-30**
- [ ] Zincir başlar ve blok üretir
- [ ] Asset Hub'da `state.getStorage(Assets, Asset, 1)` → PEZ metadata döner
- [ ] Asset Hub'da `state.getStorage(Assets, Asset, 2)` → wHEZ metadata döner
- [ ] People Chain'de IdentityKyc sorgulanabilir

---

### Phase 2: Local Testnet
**Hedef:** İki node arası konsensüs ve finalization testi
**Validator:** 2 (Alice, Bob)
**Preset:** `LOCAL_TESTNET_RUNTIME_PRESET`

#### 4.2.1. Ek Gereksinimler
- [ ] Asset Hub'da Alice ve Bob'a PEZ dağıtımı
- [ ] Cross-chain transfer testi hazırlığı

#### 4.2.2. Test Komutu
```bash
# Node A (Alice)
./target/release/pezkuwi --chain=local --alice --port=30333 --rpc-port=9944 \
  --node-key=0x0000000000000000000000000000000000000000000000000000000000000001

# Node B (Bob)
./target/release/pezkuwi --chain=local --bob --port=30334 --rpc-port=9945 \
  --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwM8dC35nPvW66PzC9v6z
```

#### 4.2.3. Başarı Kriterleri
- [ ] Node'lar birbirini keşfeder (peer count >= 1)
- [ ] Bloklar "Finalized" durumuna geçer
- [ ] Her iki node'da aynı blok hash

---

### Phase 3: Alpha Testnet (Çekirdek Ekip)
**Hedef:** 4 validator ile ağ kararlılığı testi
**Validator:** 4 (Alice, Bob, Charlie, Dave)
**Preset:** `alpha_testnet` (YENİ)

#### 4.3.1. Yapılacak Değişiklikler

**Relay Chain:**
```rust
// pezkuwi/runtime/pezkuwichain/src/genesis_config_presets.rs
fn alpha_testnet_genesis() -> serde_json::Value {
    pezkuwichain_testnet_genesis(
        Vec::from([
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
            get_authority_keys_from_seed("Charlie"),
            get_authority_keys_from_seed("Dave"),
        ]),
        Sr25519Keyring::Alice.to_account_id(),
        None,
    )
}
```

**Asset Hub & People Chain:**
- [ ] `alpha_testnet` preset eklenmeli
- [ ] 4 collator tanımı

#### 4.3.2. Başarı Kriterleri
- [ ] 4 validator sorunsuz blok üretir
- [ ] Asset Hub ve People Chain parachain olarak bağlanır
- [ ] XCM mesajları iletilir

---

### Phase 4: Beta Testnet (Gerçek Key'ler)
**Hedef:** Üretim benzeri ortamda güvenli key'lerle test
**Validator:** 8 (Hex Key'ler)
**Preset:** `beta_testnet` (YENİ)

#### 4.4.1. Key Kaynağı
```
D:\PKW\res\beta_testnet_validators_WITH_SEEDS_BACKUP.json
```
**⚠️ UYARI:** Seed phrase'ler ASLA koda yazılmaz, sadece hex public key'ler kullanılır!

#### 4.4.2. Genesis Yapısı
```rust
fn beta_testnet_genesis() -> serde_json::Value {
    use hex_literal::hex;

    let initial_authorities = Vec::from([
        // Validator 1 - sadece PUBLIC KEY'ler
        (
            hex!["...stash..."].into(),
            hex!["...controller..."].into(),
            hex!["...babe..."].unchecked_into(),
            hex!["...grandpa..."].unchecked_into(),
            // ... diğer key'ler
        ),
        // ... 7 validator daha
    ]);

    // ... genesis config
}
```

#### 4.4.3. Founder & Treasury Hesapları
```
Kaynak: D:\PKW\res\Founder_treasury_presale_wallets.json
```

| Hesap | Kullanım | Genesis Bakiye |
|-------|----------|----------------|
| Founder | Sudo, Council, Founder vatandaş | 93,750,000 PEZ + 1M HEZ |
| Treasury | PezTreasury pallet | 1,012,500,000 PEZ |
| Presale | Presale pallet | 93,750,000 PEZ |

#### 4.4.4. Başarı Kriterleri
- [ ] Sadece doğru key'lere sahip node'lar validator olabilir
- [ ] Genesis PEZ dağıtımı doğru
- [ ] PezTreasury aktif ve dağıtıma hazır

---

### Phase 5: Staging Testnet (Pre-Mainnet)
**Hedef:** Mainnet öncesi performans ve yük testi
**Validator:** 21
**Preset:** `staging_testnet` (GÜNCEL)

#### 4.5.1. Key Kaynağı
```
D:\PKW\res\staging_validators.json_WITH_SEEDS_BACKUP.json
```

#### 4.5.2. Performans Testleri
- [ ] 1000 TPS yük testi
- [ ] 7 gün kesintisiz çalışma
- [ ] Parachain senkronizasyonu
- [ ] XCM köprü testleri

#### 4.5.3. Başarı Kriterleri
- [ ] 21 validator stabil çalışır
- [ ] Block time < 6.5 saniye
- [ ] Finality < 30 saniye
- [ ] Memory leak yok

---

### Phase 6: Mainnet
**Hedef:** Canlı ağ başlatma
**Validator:** 100
**Preset:** `mainnet`

#### 4.6.1. Key Kaynağı
```
D:\PKW\res\mainnet_validators_WITH_SEEDS_BACKUP.json
```

#### 4.6.2. Genesis Parametreleri
| Parametre | Değer |
|-----------|-------|
| chainType | Live |
| protocolId | pkw |
| Bootnode sayısı | ≥10 |
| Telemetry | https://telemetry.pezkuwichain.io |

#### 4.6.3. Lansman Kontrol Listesi
- [ ] Tüm 100 validator key'i doğrulandı
- [ ] Genesis block hash sabitlendi
- [ ] Bootnode'lar aktif
- [ ] Explorer entegrasyonu tamamlandı
- [ ] Cüzdan desteği (Polkadot.js, Talisman)
- [ ] Denetim raporu yayınlandı

---

## 5. Teknik Uygulama Detayları

### 5.1. Asset Hub Genesis - AssetsConfig

**Dosya:** `cumulus/teyrchains/runtimes/assets/asset-hub-pezkuwichain/src/genesis_config_presets.rs`

```rust
use crate::{Assets, AssetsConfig, AssetIdForTrustBackedAssets};

// Asset ID sabitleri
pub const PEZ_ASSET_ID: AssetIdForTrustBackedAssets = 1;
pub const WHEZ_ASSET_ID: AssetIdForTrustBackedAssets = 2;

fn asset_hub_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    treasury_account: AccountId,
    founder_account: AccountId,
    presale_account: AccountId,
    id: ParaId,
) -> serde_json::Value {
    const PEZ_TOTAL_SUPPLY: u128 = 5_000_000_000 * UNITS;
    const TREASURY_ALLOCATION: u128 = 1_012_500_000 * UNITS; // 20.25%
    const FOUNDER_ALLOCATION: u128 = 93_750_000 * UNITS;     // 1.875%
    const PRESALE_ALLOCATION: u128 = 93_750_000 * UNITS;     // 1.875%
    const REWARDS_POOL: u128 = 3_800_000_000 * UNITS;        // 76%

    build_struct_json_patch!(RuntimeGenesisConfig {
        // ... diğer config'ler

        // TrustBackedAssets (Instance1) - PEZ ve wHEZ
        assets: AssetsConfig {
            assets: vec![
                // PEZ Token
                (
                    PEZ_ASSET_ID,           // Asset ID
                    treasury_account.clone(), // Admin (owner)
                    true,                    // is_sufficient
                    1,                       // min_balance
                ),
                // Wrapped HEZ (wHEZ)
                (
                    WHEZ_ASSET_ID,
                    treasury_account.clone(),
                    true,
                    1,
                ),
            ],
            metadata: vec![
                (PEZ_ASSET_ID, b"Pez Token".to_vec(), b"PEZ".to_vec(), 12),
                (WHEZ_ASSET_ID, b"Wrapped HEZ".to_vec(), b"wHEZ".to_vec(), 12),
            ],
            accounts: vec![
                // Treasury - Rewards pool dahil
                (PEZ_ASSET_ID, treasury_account.clone(), TREASURY_ALLOCATION + REWARDS_POOL),
                // Founder
                (PEZ_ASSET_ID, founder_account.clone(), FOUNDER_ALLOCATION),
                // Presale
                (PEZ_ASSET_ID, presale_account.clone(), PRESALE_ALLOCATION),
                // wHEZ başlangıçta 0 - sadece wrap edilerek oluşur
            ],
            next_asset_id: Some(3),
        },

        // ... Presale, TokenWrapper config'leri
    })
}
```

### 5.2. People Chain Genesis - IdentityKycConfig

**Dosya:** `cumulus/teyrchains/runtimes/people/people-pezkuwichain/src/genesis_config_presets.rs`

```rust
use crate::{IdentityKycConfig, IdentityKyc};

fn people_chain_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    founder_account: AccountId,
    id: ParaId,
) -> serde_json::Value {
    build_struct_json_patch!(RuntimeGenesisConfig {
        // ... diğer config'ler

        // IdentityKyc - Founder vatandaş
        identity_kyc: IdentityKycConfig {
            // Founder = İlk vatandaş (referrer gerektirmez)
            initial_citizens: vec![
                (
                    founder_account.clone(),
                    pallet_identity_kyc::CitizenInfo {
                        kyc_level: pallet_identity_kyc::KycLevel::Full,
                        registration_block: 0,
                        referrer: None, // Founder'ın referrer'ı yok
                        is_founder: true,
                    },
                ),
            ],
        },
    })
}
```

### 5.3. Relay Chain Genesis - PezTreasury & PezRewards

**Dosya:** `pezkuwi/runtime/pezkuwichain/src/genesis_config_presets.rs`

```rust
use crate::{PezTreasuryConfig, PezRewardsConfig};

fn relay_chain_genesis(/* params */) -> serde_json::Value {
    build_struct_json_patch!(RuntimeGenesisConfig {
        // ... diğer config'ler

        // PEZ Treasury - Rewards dağıtım motoru
        pez_treasury: PezTreasuryConfig {
            // Genesis'te başlat
            initialize_treasury: true,
            // İlk epoch: 0
            current_epoch: 0,
            // Halving sayacı
            halving_count: 0,
            _phantom: Default::default(),
        },

        // PEZ Rewards - Validator/Nominator ödülleri
        pez_rewards: PezRewardsConfig {
            start_rewards_system: true,
            // Parliamentary NFT sahipleri (isteğe bağlı)
            initial_nft_holders: vec![],
            _phantom: Default::default(),
        },
    })
}
```

---

## 6. Validator Key Yönetimi

### 6.1. Key Dosyaları Konumu
```
D:\PKW\res\
├── beta_testnet_validators_WITH_SEEDS_BACKUP.json    (8 validator)
├── staging_validators.json_WITH_SEEDS_BACKUP.json    (21 validator)
├── mainnet_validators_WITH_SEEDS_BACKUP.json         (100 validator)
└── Founder_treasury_presale_wallets.json             (Özel hesaplar)
```

### 6.2. Key Yapısı (Her Validator)
```json
{
  "name": "Validator-01",
  "stash": {
    "address": "5...",
    "public_key_hex": "0x...",
    "seed": "⚠️ ASLA KODA YAZILMAZ"
  },
  "controller": { /* ... */ },
  "session_keys": {
    "babe": "0x...",
    "grandpa": "0x...",
    "para_validator": "0x...",
    "para_assignment": "0x...",
    "authority_discovery": "0x...",
    "beefy": "0x..."
  }
}
```

### 6.3. Güvenlik Kuralları
1. **ASLA** seed phrase'leri kaynak koduna yazma
2. **SADECE** public key hex değerlerini genesis'e ekle
3. Key dosyalarını **GİT'E EKLEME** (.gitignore'da)
4. Seed backup'larını **OFFLINE** ve **ŞİFRELİ** sakla
5. Mainnet key'lerini **HSM** veya **Vault** ile koru

---

## 7. Genesis Dosya Haritası

### 7.1. Değiştirilecek Dosyalar

| Dosya | Değişiklik | Öncelik |
|-------|------------|---------|
| `pezkuwi/runtime/pezkuwichain/src/genesis_config_presets.rs` | alpha, beta, staging, mainnet presetleri | 🔴 |
| `cumulus/.../asset-hub-pezkuwichain/src/genesis_config_presets.rs` | AssetsConfig (PEZ, wHEZ) | 🔴 |
| `cumulus/.../people-pezkuwichain/src/genesis_config_presets.rs` | IdentityKycConfig | 🔴 |
| `cumulus/.../asset-hub-pezkuwichain/src/lib.rs` | Presale, TokenWrapper config | 🟡 |
| `cumulus/.../people-pezkuwichain/src/lib.rs` | Trust, Welati config | 🟡 |

### 7.2. Yeni Oluşturulacak Dosyalar

| Dosya | İçerik |
|-------|--------|
| `pezkuwi/runtime/pezkuwichain/src/validators/beta.rs` | Beta testnet validator hex key'leri |
| `pezkuwi/runtime/pezkuwichain/src/validators/staging.rs` | Staging validator hex key'leri |
| `pezkuwi/runtime/pezkuwichain/src/validators/mainnet.rs` | Mainnet validator hex key'leri |

---

## 8. Test ve Doğrulama Stratejisi

### 8.1. Birim Testleri
```bash
# Her preset için
cargo test -p pezkuwichain-runtime --features runtime-benchmarks
cargo test -p asset-hub-pezkuwichain-runtime --features runtime-benchmarks
cargo test -p people-pezkuwichain-runtime --features runtime-benchmarks
```

### 8.2. Entegrasyon Testleri
```bash
# Chain spec üretimi
./target/release/pezkuwi build-spec --chain=beta_testnet --raw > beta_spec.json

# JSON içerik kontrolü
jq '.genesis.runtimeGenesis.patch.assets' beta_spec.json
jq '.genesis.runtimeGenesis.patch.identityKyc' beta_spec.json
```

### 8.3. Zombienet Testleri
```toml
# zombienet/beta_testnet.toml
[relaychain]
default_command = "./target/release/pezkuwi"
chain = "beta_testnet"

[[relaychain.nodes]]
name = "validator-1"
validator = true

# ... 7 validator daha

[[parachains]]
id = 1000
chain = "asset-hub-pezkuwichain-beta"

[[parachains.collators]]
name = "asset-hub-collator-1"
```

### 8.4. Doğrulama Kontrol Listesi

#### Asset Hub
- [ ] `state_call("AssetsApi_metadata", [1])` → PEZ metadata
- [ ] `state_call("AssetsApi_metadata", [2])` → wHEZ metadata
- [ ] `state_call("AssetsApi_balance", [1, treasury])` → 4,812,500,000 PEZ
- [ ] `state_call("AssetsApi_balance", [1, founder])` → 93,750,000 PEZ
- [ ] `state_call("AssetsApi_balance", [1, presale])` → 93,750,000 PEZ

#### People Chain
- [ ] `IdentityKyc.citizens(founder)` → CitizenInfo döner
- [ ] `IdentityKyc.citizens(founder).is_founder` → true
- [ ] `IdentityKyc.citizenCount()` → 1

#### Relay Chain
- [ ] `Session.validators()` → Beklenen validator sayısı
- [ ] `Staking.activeEra()` → Some(0)
- [ ] `PezTreasury.currentEpoch()` → 0

---

## 9. Güvenlik Kontrol Listesi

### 9.1. Genesis Öncesi
- [ ] Tüm key'ler doğru ve benzersiz
- [ ] Seed phrase'ler kodda YOK
- [ ] `.gitignore`'da key dosyaları var
- [ ] Treasury hesabı multisig veya governance kontrolünde
- [ ] Sudo hesabı güvenli (sonra kaldırılacak)

### 9.2. Genesis Sonrası
- [ ] Genesis block hash kaydedildi
- [ ] İlk bloklar doğru imzalandı
- [ ] Parachain'ler bağlandı
- [ ] Asset'ler sorgulanabilir
- [ ] Telemetry aktif

### 9.3. Mainnet Öncesi
- [ ] Güvenlik denetimi tamamlandı
- [ ] Bug bounty programı aktif
- [ ] Sudo kaldırma planı hazır
- [ ] Governance geçiş planı hazır
- [x] **Telif hakkı güncelleme:** ✅ **Tamamlandı (2025-11-30)**
  - **Dosya:** `/Cargo.toml` (workspace root - satır 2)
  - `authors = ["Parity Technologies <admin@parity.io>", "Kurdistan Tech Institute <info@pezkuwichain.io>"]`

---

## 10. Geliştirme Stratejisi

### 10.0. Temel İlkeler

> **ÖNEMLİ:** Bir kez dev testnet başlatıldıktan sonra mümkün olduğunca **SİFIRLAMAMAK**,
> upgrade ile ilerlemek. Yalnızca büyük bir bug durumunda sıfırlanır.

#### Workflow:
```
1. Benchmark build → Weight generation
2. Dev testnet başlat (Alice validator)
3. OK ise → Runtime upgrade ile Local testnet'e geç
4. OK ise → Runtime upgrade ile Alpha testnet'e geç
5. ... devam
```

#### Sıfırlama Koşulları:
- ❌ Storage migration başarısız olursa
- ❌ Kritik consensus bug'ı varsa
- ❌ Genesis state tutarsızlığı varsa
- ✅ Aksi durumda: **UPGRADE İLE İLERLE**

---

## 11. Uygulama Takvimi

### 10.1. Faz 1: Temel Altyapı (1-2 Gün)
| Görev | Sorumlu | Durum |
|-------|---------|-------|
| Asset Hub AssetsConfig implementasyonu | Claude | ✅ **Tamamlandı** |
| People Chain IdentityKycConfig implementasyonu | Claude | ✅ **Tamamlandı** |
| Relay Chain HEZ genesis dağılımı | Claude | ✅ **Tamamlandı** |
| Bridge Hub Parachain ID düzeltmesi (1013→1002) | Claude | ✅ **Tamamlandı** |
| Dev preset compile testleri | Claude | ✅ **Tamamlandı** |
| **Tüm özel pallet benchmark'ları** | Claude | ✅ **Tamamlandı (2025-11-30)** |
| **Weight generation (gerçek değerler)** | Claude | ✅ **Tamamlandı** |

#### Benchmark Özeti (2025-11-30)
| Pallet | Extrinsic Sayısı | Durum |
|--------|------------------|-------|
| pallet_staking_score | 1 | ✅ Gerçek weight |
| pallet_pez_rewards | 6 | ✅ Gerçek weight |
| pallet_perwerde | 4 | ✅ Gerçek weight |
| pallet_trust | 3 | ✅ Gerçek weight |
| pallet_presale | 6 gerçek + 3 placeholder | ✅ Tamamlandı |
| pallet_identity_kyc | 6 | ✅ Gerçek weight |

**Commit:** `057124ddc7` - Benchmark düzeltmeleri ve weight generation

### 10.2. Faz 2: Testnet Presetleri (2-3 Gün)
| Görev | Sorumlu | Durum |
|-------|---------|-------|
| alpha_testnet preset | Claude | ⬜ |
| beta_testnet preset (8 validator) | Claude | ⬜ |
| Validator key entegrasyonu | Claude | ⬜ |

### 10.3. Faz 3: Staging & Mainnet (3-5 Gün)
| Görev | Sorumlu | Durum |
|-------|---------|-------|
| staging_testnet preset (21 validator) | Claude | ⬜ |
| mainnet preset (100 validator) | Claude | ⬜ |
| Zombienet test senaryoları | Claude | ⬜ |

### 10.4. Faz 4: Doğrulama & Dokümantasyon (2 Gün)
| Görev | Sorumlu | Durum |
|-------|---------|-------|
| Tüm preset'ler için build-spec testi | Claude | ⬜ |
| E2E testleri | Claude | ⬜ |
| Operatör dokümantasyonu | Claude | ⬜ |

---

## 11. Acil Durum Prosedürleri

### 11.1. Genesis Hatası Durumunda
1. **DURDUR:** Tüm node'ları kapat
2. **ANALİZ:** Hatayı tespit et
3. **DÜZELT:** Genesis config'i güncelle
4. **YENİDEN BAŞLAT:** Yeni genesis ile başlat
5. **DOĞRULA:** Tüm kontrolleri tekrarla

### 11.2. Key Sızıntısı Durumunda
1. **İZOLE ET:** Etkilenen validator'ları çıkar
2. **YENİ KEY:** Yeni key seti oluştur
3. **ROTATE:** Session key rotasyonu yap
4. **BİLDİR:** Topluluk bilgilendirmesi

### 11.3. İletişim Kanalları
- **Teknik:** Discord #validators-tech
- **Acil:** Telegram @PezkuwiEmergency
- **Genel:** Discord #announcements

---

## EK A: Komut Referansı

### Genesis Spec Üretimi
```bash
# Dev
./target/release/pezkuwi build-spec --chain=dev --raw > dev_spec.json

# Beta Testnet
./target/release/pezkuwi build-spec --chain=beta_testnet --raw > beta_spec.json

# Mainnet
./target/release/pezkuwi build-spec --chain=mainnet --raw > mainnet_spec.json
```

### Node Başlatma
```bash
# Validator
./target/release/pezkuwi \
  --chain=mainnet_spec.json \
  --validator \
  --name="Validator-01" \
  --base-path=/data/pezkuwi \
  --port=30333 \
  --rpc-port=9944 \
  --prometheus-port=9615 \
  --telemetry-url="wss://telemetry.pezkuwichain.io/submit 0"
```

### Session Key Ekleme
```bash
# Her validator için
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": ["babe", "0x<babe_seed>", "0x<babe_public>"]
}' http://localhost:9944

# Tüm key tipleri için tekrarla: gran, para, asgn, audi, beef
```

---

## EK B: Referans Dosyalar

| Dosya | Açıklama |
|-------|----------|
| `docs/whitepaper.md` | Tokenomik ve mimari detaylar |
| `docs/ornek_genesis-config-preset.rs` | Eski model örnek genesis |
| `CLAUDE.md` | Claude Code kuralları |
| `.ai-coordination/` | AI koordinasyon dosyaları |

---

**Bu plan, Pezkuwi Network'ün güvenli ve başarılı bir şekilde başlatılması için kapsamlı bir yol haritası sağlar. Her aşama titizlikle test edilmeli ve doğrulanmalıdır.**

---
*Son güncelleme: 2025-11-30 | Versiyon: 2.0.0*
