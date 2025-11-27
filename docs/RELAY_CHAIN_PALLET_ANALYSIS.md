# PezkuwiChain Relay Chain - Kapsamlı Pallet Analizi

**Tarih:** 2024-11-27
**Runtime:** pezkuwichain-runtime v7.0.0
**Analiz Türü:** Custom Pallet Bağımlılıkları + Standart Pallet Gereklilikleri

---

## 1. Custom Pezkuwi Palletleri ve Bağımlılıkları

### 1.1 Pallet Bağımlılık Grafiği

```
                    ┌──────────────────┐
                    │  pallet-identity │ (standard - MEVCUT)
                    │      -kyc        │
                    └────────┬─────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
            ▼                ▼                ▼
    ┌───────────────┐ ┌───────────┐ ┌───────────────┐
    │ pallet-tiki   │ │ pallet-   │ │   pallet-     │
    │ (Role NFTs)   │ │ referral  │ │   welati      │
    └───────┬───────┘ └─────┬─────┘ │ (Governance)  │
            │               │       └───────┬───────┘
            │               │               │
            ▼               ▼               │
    ┌───────────────────────────────┐       │
    │        pallet-trust           │◄──────┘
    │   (Composite Trust Score)     │
    └───────────────┬───────────────┘
                    │
    ┌───────────────┼───────────────┐
    │               │               │
    ▼               ▼               ▼
┌──────────┐ ┌──────────────┐ ┌──────────────┐
│ pallet-  │ │   pallet-    │ │   pallet-    │
│ staking- │ │   perwerde   │ │   validator  │
│ score    │ │ (Education)  │ │    -pool     │
└──────────┘ └──────────────┘ └──────────────┘
     │                               │
     ▼                               ▼
┌──────────────┐             ┌──────────────┐
│ pallet-      │             │   pallet-    │
│ staking      │             │ pez-rewards  │
│ (STANDARD)   │             └──────┬───────┘
└──────────────┘                    │
                                    ▼
                            ┌──────────────┐
                            │   pallet-    │
                            │ pez-treasury │
                            └──────────────┘
```

### 1.2 Custom Palletlerin Standart Pallet Bağımlılıkları

| Custom Pallet | Gerekli Standart Palletler | Kritiklik |
|--------------|---------------------------|-----------|
| **pallet-tiki** | `pallet-nfts` (NFT altyapısı) | **KRİTİK** |
| **pallet-identity-kyc** | `pallet-balances` (deposit) | **KRİTİK** |
| **pallet-token-wrapper** | `pallet-assets` (wHEZ tokenı) | **KRİTİK** |
| **pallet-pez-treasury** | `pallet-assets` (PEZ tokenı) | **KRİTİK** |
| **pallet-pez-rewards** | `pallet-assets` + `pallet-nfts` | **KRİTİK** |
| **pallet-presale** | `pallet-assets` (presale tokenları) | **KRİTİK** |
| **pallet-staking-score** | `pallet-staking` (staking bilgisi) | MEVCUT |
| **pallet-referral** | - (bağımsız) | - |
| **pallet-perwerde** | - (bağımsız) | - |
| **pallet-trust** | - (diğer custom palletlere bağlı) | - |
| **pallet-welati** | `pallet-tiki` (rol ataması) | - |
| **pallet-validator-pool** | `pallet-staking` | MEVCUT |

---

## 2. Standart Palletlerin Relay Chain için Zorunluluk Analizi

### 2.1 ZORUNLU - Relay Chain Temel İşlevleri

Bu palletler olmadan relay chain çalışamaz:

| Pallet | Durum | Açıklama |
|--------|-------|----------|
| `frame-system` | ✅ MEVCUT | Temel sistem |
| `pallet-timestamp` | ✅ MEVCUT | Zaman damgası |
| `pallet-balances` | ✅ MEVCUT | Native token (HEZ) |
| `pallet-transaction-payment` | ✅ MEVCUT | İşlem ücretleri |
| `pallet-authorship` | ✅ MEVCUT | Blok yazarlığı |
| `pallet-session` | ✅ MEVCUT | Oturum yönetimi |
| `pallet-babe` | ✅ MEVCUT | Blok üretim konsensüsü |
| `pallet-grandpa` | ✅ MEVCUT | Finalite gadgeti |
| `pallet-staking` | ✅ MEVCUT | NPoS staking |
| `pallet-offences` | ✅ MEVCUT | Suç kaydı |
| `pallet-bags-list` | ✅ MEVCUT | Voter listesi |

### 2.2 ZORUNLU - Parachain (TeyrChain) Yönetimi

Bu palletler parachain'leri yönetmek için zorunludur:

| Pallet | Durum | Açıklama |
|--------|-------|----------|
| `teyrchains_origin` | ✅ MEVCUT | Parachain origin |
| `teyrchains_configuration` | ✅ MEVCUT | Parachain config |
| `teyrchains_shared` | ✅ MEVCUT | Paylaşılan state |
| `teyrchains_inclusion` | ✅ MEVCUT | Blok dahil etme |
| `teyrchains_paras` | ✅ MEVCUT | Parachain kayıt |
| `teyrchains_scheduler` | ✅ MEVCUT | Core zamanlama |
| `teyrchains_disputes` | ✅ MEVCUT | Anlaşmazlık çözümü |
| `teyrchains_dmp` | ✅ MEVCUT | Downward messaging |
| `teyrchains_hrmp` | ✅ MEVCUT | Horizontal messaging |
| `paras_registrar` | ✅ MEVCUT | Parachain kaydı |
| `slots` | ✅ MEVCUT | Slot yönetimi |
| `auctions` | ✅ MEVCUT | Slot açık artırması |
| `crowdloan` | ✅ MEVCUT | Crowdloan |
| `coretime` | ✅ MEVCUT | Coretime yönetimi |

### 2.3 ZORUNLU - Governance (OpenGov)

Modern OpenGov sistemi için gerekli:

| Pallet | Durum | Açıklama |
|--------|-------|----------|
| `pallet-conviction-voting` | ✅ MEVCUT | Conviction voting |
| `pallet-referenda` | ✅ MEVCUT | Referandum sistemi |
| `pallet-ranked-collective` | ✅ MEVCUT | Fellowship collective |
| `pallet-whitelist` | ✅ MEVCUT | Whitelist çağrıları |
| `pallet-treasury` | ✅ MEVCUT | Hazine yönetimi |
| `pallet-bounties` | ✅ MEVCUT | Bounty sistemi |
| `pallet-child-bounties` | ✅ MEVCUT | Alt bounty'ler |
| `pallet-collective` | ✅ MEVCUT | Council collective |

### 2.4 GEREKLİ - Utility ve Güvenlik

| Pallet | Durum | Açıklama |
|--------|-------|----------|
| `pallet-utility` | ✅ MEVCUT | Batch işlemler |
| `pallet-multisig` | ✅ MEVCUT | Multi-sig cüzdanlar |
| `pallet-proxy` | ✅ MEVCUT | Proxy hesaplar |
| `pallet-scheduler` | ✅ MEVCUT | Zamanlanmış görevler |
| `pallet-preimage` | ✅ MEVCUT | Preimage depolama |
| `pallet-identity` | ✅ MEVCUT | Kimlik yönetimi |
| `pallet-vesting` | ✅ MEVCUT | Token vesting |

### 2.5 GEREKLİ - Bridges ve Interoperability

| Pallet | Durum | Açıklama |
|--------|-------|----------|
| `pallet-beefy` | ✅ MEVCUT | BEEFY konsensüs |
| `pallet-beefy-mmr` | ✅ MEVCUT | BEEFY MMR |
| `pallet-mmr` | ✅ MEVCUT | Merkle Mountain Range |
| `pallet-xcm` | ✅ MEVCUT | Cross-chain messaging |

---

## 3. CARGO.TOML'DA OLUP RUNTIME'DA OLMAYAN PALLETLER

### 3.1 KRİTİK - Custom Palletler için GEREKLİ

| Pallet | Neden Gerekli | Custom Pallet Bağımlılığı |
|--------|---------------|---------------------------|
| **`pallet-assets`** | Fungible asset yönetimi | `pallet-token-wrapper`, `pallet-pez-treasury`, `pallet-pez-rewards`, `pallet-presale` |
| **`pallet-nfts`** | NFT altyapısı | `pallet-tiki` (Soulbound role NFTs), `pallet-pez-rewards` (Parliamentary NFTs) |

**KARAR: Bu iki pallet ZORUNLU olarak entegre edilmelidir.**

### 3.2 OPSİYONEL - Relay Chain için Gerekli DEĞİL

| Pallet | Analiz | Öneri |
|--------|--------|-------|
| **`pallet-asset-conversion`** | DEX fonksiyonelliği - genellikle parachain'de | ⚠️ TeyrChain'e taşı veya kaldır |
| **`pallet-tips`** | Eski tip sistemi, treasury yeterli | ❌ Kaldır |
| **`pallet-democracy`** | Eski governance, OpenGov var | ❌ Kaldır |
| **`pallet-elections-phragmen`** | Eski council seçimleri | ❌ Kaldır |
| **`pallet-election-provider-multi-phase`** | Büyük ağlar için, OnChain yeterli | ❌ Kaldır (testnet için) |

---

## 4. SONUÇ VE ÖNERİLER

### 4.1 Entegre Edilmesi ZORUNLU Palletler

```rust
// construct_runtime!'a eklenmesi gerekenler:

// Asset Management (Custom palletler için ZORUNLU)
Assets: pallet_assets = 36,                    // Fungible assets (wHEZ, PEZ)
Nfts: pallet_nfts = 37,                        // NFT altyapısı (Tiki roles)
```

### 4.2 Kaldırılması Önerilen Bağımlılıklar

Aşağıdaki palletler Cargo.toml'dan kaldırılmalı (kullanılmıyorlar ve binary boyutunu artırıyorlar):

```toml
# KALDIRILACAK (Cargo.toml'dan)
# pallet-tips - OpenGov ile değiştirildi
# pallet-democracy - OpenGov ile değiştirildi
# pallet-elections-phragmen - Council için yeni collective kullanılıyor
# pallet-election-provider-multi-phase - OnChain yeterli (testnet)
# pallet-asset-conversion - Parachain'e taşınabilir
```

### 4.3 Faz 2 için Custom Pallet Entegrasyon Sırası

Custom palletler şu sırayla entegre edilmeli (bağımlılık sırasına göre):

1. **İlk Dalga (Bağımlılık yok):**
   - `pallet-referral`
   - `pallet-perwerde`
   - `pallet-staking-score`

2. **İkinci Dalga (pallet-assets gerekli):**
   - `pallet-token-wrapper`
   - `pallet-pez-treasury`
   - `pallet-presale`

3. **Üçüncü Dalga (pallet-nfts gerekli):**
   - `pallet-tiki`
   - `pallet-identity-kyc`

4. **Dördüncü Dalga (Önceki palletlere bağlı):**
   - `pallet-trust`
   - `pallet-pez-rewards`
   - `pallet-validator-pool`
   - `pallet-welati`

---

## 5. FİNAL PALLET LİSTESİ

### 5.1 Relay Chain için Minimum Zorunlu Standard Palletler

| Kategori | Palletler | Sayı |
|----------|-----------|------|
| **Core System** | frame-system, timestamp, balances, transaction-payment, indices | 5 |
| **Consensus** | babe, grandpa, authorship, session, offences | 5 |
| **Staking** | staking, bags-list, nomination-pools, fast-unstake | 4 |
| **Governance** | conviction-voting, referenda, ranked-collective, whitelist, collective, treasury, bounties, child-bounties | 8 |
| **Parachain** | 14 teyrchains palletleri + registrar, slots, auctions, crowdloan, coretime | 19 |
| **Utility** | utility, multisig, proxy, scheduler, preimage, identity, vesting, recovery, society | 9 |
| **Bridges** | beefy, beefy-mmr, mmr, xcm | 4 |
| **Assets** | assets, nfts, asset-rate | 3 |
| **System** | migrations, parameters, message-queue, state-trie-migration, nis | 5 |
| **Dev** | sudo, root-testing | 2 |
| **TOPLAM** | | **64** |

### 5.2 Kaldırılacak Palletler (5 adet)

1. `pallet-tips`
2. `pallet-democracy`
3. `pallet-elections-phragmen`
4. `pallet-election-provider-multi-phase`
5. `pallet-asset-conversion` (opsiyonel - parachain'e taşınabilir)

---

## 6. EYLEM PLANI

### Adım 1: pallet-assets Entegrasyonu
- Config implementation ekle
- construct_runtime!'a ekle (index 36)
- Benchmark ekle

### Adım 2: pallet-nfts Entegrasyonu
- Config implementation ekle
- construct_runtime!'a ekle (index 37)
- Benchmark ekle

### Adım 3: Gereksiz Bağımlılıkları Temizle
- Cargo.toml'dan kaldır
- std, runtime-benchmarks, try-runtime feature'larından kaldır

### Adım 4: Build ve Test
- `cargo build --release -p pezkuwichain-runtime`
- `cargo build --release -p pezkuwichain-runtime --features runtime-benchmarks`

---

**Rapor Sonu**

🤖 Generated with Claude Code
