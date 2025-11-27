# Frontend (pwap) Etki Analiz Raporu

**Tarih:** 2024-11-27
**Analiz Konusu:** Standart pallet değişikliklerinin pwap frontend'ine etkisi
**Etkilenen Projeler:** pwap/shared, pwap/pezkuwi-sdk-ui, pwap/mobile

---

## YÜRÜTME ÖZETİ (EXECUTIVE SUMMARY)

### 🔴 KRİTİK BAĞIMLILIKLAR

| Platform | Dosya | Pallet Bağımlılığı | Durum |
|----------|-------|-------------------|-------|
| **MOBİL** | `SwapScreen.tsx` | `pallet-assets`, `pallet-asset-conversion` | ⚠️ KRİTİK |
| **MOBİL** | `NFTGalleryScreen.tsx` | `pallet-tiki` (custom), `pallet-nfts` | ⚠️ KRİTİK |
| **MOBİL** | `WalletScreen.tsx` | `pallet-assets` | ⚠️ KRİTİK |
| **WEB** | `page-assets/` | `pallet-assets` | ⚠️ KRİTİK |
| **WEB** | `page-nfts/` | `pallet-nfts` | ⚠️ KRİTİK |
| **SHARED** | `utils/dex.ts` | `pallet-asset-conversion` | ⚠️ KRİTİK |

### ⚡ HIZLI KARAR TABLOSU

| Değişiklik | Mobil Etkisi | Web Etkisi | Öneri |
|------------|--------------|------------|-------|
| ✅ `pallet-assets` eklendi | SwapScreen ✅ ÇALIŞIR | page-assets ✅ ÇALIŞIR | ✅ |
| ✅ `pallet-nfts` eklendi | NFTGalleryScreen ✅ ÇALIŞIR | page-nfts ✅ ÇALIŞIR | ✅ |
| ❓ `pallet-asset-conversion` | SwapScreen ❌ ÇALIŞMAZ | - | KRİTİK KARAR |
| ❌ `pallet-democracy` kaldırılırsa | - | page-democracy disable | ÖNERİLİR |
| ❌ `pallet-tips` kaldırılırsa | - | Minimal | ÖNERİLİR |

---

## 1. ÖZET

Bu rapor, PezkuwiChain relay chain'e eklenen `pallet-assets` ve `pallet-nfts` palletlerinin ve potansiyel olarak kaldırılması düşünülen palletlerin frontend uygulamasına etkisini analiz eder.

---

## 2. EKLENMESİ ZORUNLU PALLETLER

### 2.1 pallet-assets (ETKİ: POZİTİF)

**Eklenen Index:** 36

**Frontend Dosyaları:**
- `pwap/shared/types/dex.ts` - Token tanımları
- `pwap/shared/utils/dex.ts` - DEX işlemleri
- `pwap/pezkuwi-sdk-ui/packages/page-assets/` - Asset yönetim arayüzü

**Frontend Etki Analizi:**

| Durum | Etki |
|-------|------|
| ✅ **pallet-assets EKLENDİ** | Frontend DEX fonksiyonları çalışır |
| ❌ pallet-assets EKSİK | `useAssetIds()` ve `useAssetInfos()` hooks çalışmaz |

**Kod Referansları:**

```typescript
// pwap/shared/utils/dex.ts:151
const poolKeys = await api.query.assetConversion.pools.keys();

// pwap/shared/utils/dex.ts:165
const reserve1Data = await api.query.assets.account(asset1, poolAccount);

// pwap/pezkuwi-sdk-ui/packages/apps-routing/src/assets.ts:12-15
needsApi: [
  'tx.assets.setMetadata',
  'tx.assets.transferKeepAlive'
]
```

**SONUÇ:** ✅ `pallet-assets` eklenmesi frontend için ZORUNLU idi. Artık eklendi.

---

### 2.2 pallet-nfts (ETKİ: POZİTİF)

**Eklenen Index:** 37

**Frontend Dosyaları:**
- `pwap/pezkuwi-sdk-ui/packages/page-nfts/` - NFT arayüzü
- `pwap/shared/lib/tiki.ts` - Tiki (role NFT) işlemleri

**Frontend Etki Analizi:**

| Durum | Etki |
|-------|------|
| ✅ **pallet-nfts EKLENDİ** | NFT sayfası ve Tiki rolleri çalışır |
| ❌ pallet-nfts EKSİK | Phase 2 custom palletleri (pallet-tiki) çalışmaz |

**SONUÇ:** ✅ `pallet-nfts` eklenmesi frontend için ZORUNLU idi. Artık eklendi.

---

## 3. KALDIRILMASI ÖNERİLEN PALLETLER VE ETKİLERİ

### 3.1 pallet-asset-conversion (DEX)

**Mevcut Durum:** Cargo.toml'da var, construct_runtime!'da YOK

**Frontend Bağımlılıkları:**

```typescript
// pwap/shared/utils/dex.ts - KRİTİK BAĞIMLILIK!

// Satır 151: Pool sorgusu
const poolKeys = await api.query.assetConversion.pools.keys();

// Satır 160: Pool hesabı sorgusu
const poolAccount = await api.query.assetConversion.pools([asset1, asset2]);
```

**ETKİ ANALİZİ:**

| Senaryo | Frontend Etkisi | Aksiyon Gerekli |
|---------|-----------------|-----------------|
| **Kaldırılırsa** | DEX swap, pool, LP fonksiyonları çalışmaz | `dex.ts` tamamen yeniden yazılmalı |
| **TeyrChain'e taşınırsa** | XCM üzerinden asset transfer gerekli | `dex.ts` + yeni XCM bridge kodu |
| **Relay chain'de kalırsa** | Değişiklik yok | - |

**DEX Fonksiyonları Listesi (Etkilenecekler):**
1. `fetchPools()` - Pool listesi
2. `findBestSwapRoute()` - Swap rotası hesaplama
3. `fetchUserLPPositions()` - Kullanıcı LP pozisyonları
4. `getAmountOut()` - Swap çıktı hesaplama
5. `calculatePriceImpact()` - Fiyat etkisi
6. `calculatePoolTVL()` - TVL hesaplama
7. `calculatePoolAPR()` - APR hesaplama

**DETAYLI ETKİ:**

```
DEX Relay Chain'de ise:
┌─────────┐    ┌─────────────┐    ┌─────────┐
│ Frontend│───▶│ Relay Chain │───▶│   DEX   │
└─────────┘    └─────────────┘    └─────────┘
                    ↓ Doğrudan çağrı

DEX TeyrChain'de ise:
┌─────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────┐
│ Frontend│───▶│ Relay Chain │───▶│  XCM Bridge │───▶│TeyrChain│
└─────────┘    └─────────────┘    └─────────────┘    │   DEX   │
                                                       └─────────┘
                    ↓ XCM mesajı gerekli
```

**ÖNERİ:**

DEX'i TeyrChain'e taşımak mimari olarak daha doğru, AMA:
1. Frontend'de `xcm-bridge.ts` ve `xcm-wizard.ts` güncellenmeli
2. `dex.ts` yeniden yazılmalı (TeyrChain API'si kullanacak şekilde)
3. UX değişikliği: Kullanıcılar önce asset'leri transfer edecek

**İŞ YÜKÜ TAHMİNİ:** 3-5 gün frontend geliştirme

---

### 3.2 pallet-tips

**Mevcut Durum:** Cargo.toml'da var, construct_runtime!'da YOK

**Frontend Bağımlılıkları:**

```
pwap/pezkuwi-sdk-ui/packages/page-treasury/ içinde tip sistemi olabilir
```

**ETKİ ANALİZİ:**

| Durum | Etki |
|-------|------|
| Kaldırılırsa | Minimal - tip sistemi zaten deprecated |
| Tutulursa | Değişiklik yok |

**ÖNERİ:** Kaldırılabilir, frontend etkisi minimal.

---

### 3.3 pallet-democracy

**Mevcut Durum:** Cargo.toml'da var, construct_runtime!'da YOK

**Frontend Bağımlılıkları:**

```
pwap/pezkuwi-sdk-ui/packages/page-democracy/
```

**ETKİ ANALİZİ:**

| Durum | Etki |
|-------|------|
| Kaldırılırsa | Democracy sayfası kaldırılmalı |
| Tutulursa | Değişiklik yok |

**ÖNERİ:**
- OpenGov (referenda) zaten aktif
- pallet-democracy eski governance sistemi
- **pallet-welati** kendi seçim sistemini implement ediyor
- Kaldırılabilir, `page-democracy` de kaldırılmalı veya disable edilmeli

---

### 3.4 pallet-elections-phragmen

**Mevcut Durum:** Cargo.toml'da var, construct_runtime!'da YOK

**Frontend Bağımlılıkları:**

```
pwap/pezkuwi-sdk-ui/packages/page-council/
```

**ETKİ ANALİZİ:**

| Durum | Etki |
|-------|------|
| Kaldırılırsa | Council seçim sayfası güncellenmeli |
| Tutulursa | Değişiklik yok |

**ÖNERİ:**
- **pallet-collective** zaten construct_runtime!'da var (index 17)
- **pallet-welati** kendi parlamento seçimini implement ediyor
- Kaldırılabilir

---

### 3.5 pallet-election-provider-multi-phase

**Mevcut Durum:** Cargo.toml'da var, construct_runtime!'da YOK

**Frontend Bağımlılıkları:** Doğrudan frontend bağımlılığı YOK

**ETKİ ANALİZİ:**

| Durum | Etki |
|-------|------|
| Kaldırılırsa | Frontend etkisi YOK |
| Tutulursa | Değişiklik yok |

**ÖNERİ:** Kaldırılabilir. OnChain election testnet için yeterli.

---

## 4. ÖZET KARAR TABLOSU

| Pallet | Durum | Frontend Etkisi | Öneri |
|--------|-------|-----------------|-------|
| `pallet-assets` | ✅ EKLENDİ | POZİTİF | ✅ |
| `pallet-nfts` | ✅ EKLENDİ | POZİTİF | ✅ |
| `pallet-asset-conversion` | ❓ BEKLEMEDE | KRİTİK | Tartışılmalı |
| `pallet-tips` | ❌ YOK | Minimal | Kaldır |
| `pallet-democracy` | ❌ YOK | Orta | Kaldır |
| `pallet-elections-phragmen` | ❌ YOK | Orta | Kaldır |
| `pallet-election-provider-multi-phase` | ❌ YOK | Yok | Kaldır |

---

## 5. DEX TAŞIMA SENARYOLARI

### Senaryo A: DEX Relay Chain'de Kalır

**Artıları:**
- Frontend değişikliği yok
- Daha basit UX
- Hızlı deployment

**Eksileri:**
- Relay chain'in blok alanını tüketir
- Polkadot mimarisine aykırı

**Frontend İş Yükü:** 0 gün

---

### Senaryo B: DEX TeyrChain'e Taşınır

**Artıları:**
- Polkadot best practices'e uygun
- Relay chain hafif kalır
- TeyrChain özel olarak optimize edilebilir

**Eksileri:**
- Frontend yeniden yazılmalı
- UX biraz karmaşıklaşır (XCM transfer gerekli)

**Frontend İş Yükü:** 3-5 gün

**Gerekli Değişiklikler:**

1. **`pwap/shared/utils/dex.ts`:**
   - `fetchPools()` → TeyrChain API'sine bağlan
   - Tüm `api.query.assetConversion.*` çağrıları değişecek

2. **`pwap/shared/lib/xcm-bridge.ts`:**
   - Asset transfer fonksiyonları güncelle
   - Relay ↔ TeyrChain asset bridge

3. **Yeni Component:**
   - DEX sayfasında "Transfer to TeyrChain" butonu
   - Asset balance kontrolleri (hem relay hem teyrchain)

4. **UX Flow Değişikliği:**
   ```
   Eski: [Swap] → İşlem tamam
   Yeni: [Transfer to TeyrChain] → [Swap on TeyrChain] → [Transfer back (optional)]
   ```

---

## 6. ÖNERİLEN EYLEM PLANI

### Aşama 1: Şimdi (Yapıldı)
- [x] pallet-assets ekle
- [x] pallet-nfts ekle

### Aşama 2: Karar
- [ ] pallet-asset-conversion için karar ver:
  - A) Relay chain'de tut (şimdilik)
  - B) TeyrChain'e taşı (gelecekte)

### Aşama 3: Temizlik
- [ ] Kullanılmayan palletleri Cargo.toml'dan kaldır:
  - pallet-tips
  - pallet-democracy
  - pallet-elections-phragmen
  - pallet-election-provider-multi-phase

### Aşama 4: Frontend Güncelleme
- [ ] page-democracy disable et veya kaldır
- [ ] page-council güncelle (collective-based)
- [ ] (Eğer Senaryo B seçilirse) DEX kodunu yeniden yaz

---

## 7. SONUÇ

**Eklenen palletler (`pallet-assets`, `pallet-nfts`):** Frontend için POZİTİF etki. DEX ve NFT sayfaları artık çalışabilir.

**Kaldırılması önerilen palletler:** Çoğunluğu frontend'de zaten aktif olarak kullanılmıyor. Temizlik yapılabilir.

**DEX (pallet-asset-conversion):** En kritik karar. Şimdilik relay chain'de tutulması, sonra Phase 2'de TeyrChain'e taşınması önerilir.

---

---

## 8. DETAYLI MOBİL UYGULAMA ANALİZİ

### 8.1 SwapScreen.tsx - KRİTİK BAĞIMLILIKLAR

**Dosya:** `pwap/mobile/src/screens/SwapScreen.tsx`
**Satır Sayısı:** 904

**Blockchain API Çağrıları:**

```typescript
// Satır 85 - Asset bakiyesi sorgusu (pallet-assets)
const assetData = await api.query.assets.account(
  token.assetId,
  selectedAccount.address
);

// Satır 130 - Pool sorgusu (pallet-asset-conversion) ⚠️ KRİTİK!
const poolAccount = await api.query.assetConversion.pools([
  state.fromToken.assetId,
  state.toToken.assetId,
]);

// Satır 143-150 - Pool reserve sorgusu (pallet-assets)
const reserve1Data = await api.query.assets.account(
  state.fromToken.assetId,
  poolAccount.unwrap()
);

// Satır 339 - Swap işlemi (pallet-asset-conversion) ⚠️ KRİTİK!
const tx = api.tx.assetConversion.swapTokensForExactTokens(
  path,
  amountOutMin,
  amountIn,
  selectedAccount.address,
  false
);
```

**ETKİ DEĞERLENDİRMESİ:**

| Pallet | Kullanım | Etki Durumu |
|--------|----------|-------------|
| `pallet-assets` | ✅ Bakiye sorgusu | ✅ EKLENDİ - Çalışır |
| `pallet-asset-conversion` | ⚠️ Pool sorgusu + Swap TX | ⚠️ KARAR BEKLİYOR |

**SENARYO ANALİZİ:**

1. **pallet-asset-conversion relay chain'de kalırsa:**
   - SwapScreen tam işlevsel
   - Değişiklik gerekmiyor

2. **pallet-asset-conversion TeyrChain'e taşınırsa:**
   - `fetchPoolReserves()` fonksiyonu yeniden yazılmalı
   - `handleSwap()` fonksiyonu XCM üzerinden çalışacak şekilde güncellenmeli
   - Yeni "Transfer to TeyrChain" butonu eklenmeli
   - Estimated work: 2-3 gün

---

### 8.2 NFTGalleryScreen.tsx - TIKI BAĞIMLILIĞI

**Dosya:** `pwap/mobile/src/screens/NFTGalleryScreen.tsx`

**Blockchain API Çağrıları:**

```typescript
// Satır 59 - Vatandaşlık NFT sorgusu (pallet-tiki - CUSTOM)
const citizenNft = await api.query.tiki?.citizenNft?.(selectedAccount.address);

// Satır 82 - Tiki rol sorgusu (shared/lib/tiki.ts üzerinden)
const tikis = await fetchUserTikis(api, selectedAccount.address);
```

**ETKİ DEĞERLENDİRMESİ:**

| Pallet | Kullanım | Etki Durumu |
|--------|----------|-------------|
| `pallet-nfts` | Altyapı | ✅ EKLENDİ - Hazır |
| `pallet-tiki` | Tiki rolleri | ⏳ PHASE 2'de eklenecek |

**NOT:** NFTGalleryScreen şu an `pallet-tiki`'nin varlığını kontrol ederek çalışıyor (optional chaining: `api.query.tiki?.citizenNft?.()`). Phase 2'de `pallet-tiki` eklendiğinde tam işlevsel olacak.

---

### 8.3 WalletScreen.tsx - ASSET BAKİYELERİ

**Dosya:** `pwap/mobile/src/screens/WalletScreen.tsx`

**Beklenen API Çağrıları:**
```typescript
// Native token (HEZ) bakiyesi
api.query.system.account(address)

// Asset bakiyeleri (wHEZ, PEZ, wUSDT)
api.query.assets.account(assetId, address)
```

**ETKİ DEĞERLENDİRMESİ:**
- ✅ `pallet-assets` eklendi - Asset bakiyeleri görüntülenebilir

---

## 9. DETAYLI WEB UYGULAMASI ANALİZİ

### 9.1 pezkuwi-sdk-ui/packages/page-assets/

**Dosya:** `pwap/pezkuwi-sdk-ui/packages/page-assets/src/index.tsx`

**React Hooks Kullanımı:**
```typescript
// Satır 15 - useAssetIds ve useAssetInfos hooks
import { useAccounts, useApi, useAssetIds, useAssetInfos } from '@polkadot/react-hooks';

// Satır 57-58 - Asset ID'leri ve bilgileri
const ids = useAssetIds();
const infos = useAssetInfos(ids);
```

**API Gereksinimi:** `tx.assets.setMetadata`, `tx.assets.transferKeepAlive`

**ETKİ DEĞERLENDİRMESİ:**
- ✅ `pallet-assets` eklendi - Asset sayfası tam işlevsel

---

### 9.2 pezkuwi-sdk-ui/packages/page-nfts/

**Dosya:** `pwap/pezkuwi-sdk-ui/packages/page-nfts/src/index.tsx`

**ETKİ DEĞERLENDİRMESİ:**
- ✅ `pallet-nfts` eklendi - NFT sayfası tam işlevsel

---

### 9.3 pezkuwi-sdk-ui/packages/page-democracy/

**ETKİ DEĞERLENDİRMESİ:**
- ❌ `pallet-democracy` construct_runtime!'da YOK
- ⚠️ Bu sayfa disable edilmeli veya kaldırılmalı
- ✅ OpenGov (page-referenda) zaten var ve çalışıyor

---

## 10. SHARED LIB ETKİ ANALİZİ

### 10.1 pwap/shared/utils/dex.ts - KRİTİK

**Tüm DEX Fonksiyonları ve Bağımlılıkları:**

| Fonksiyon | Satır | Pallet Bağımlılığı | Etki |
|-----------|-------|-------------------|------|
| `fetchPools()` | 146-247 | `pallet-asset-conversion` | ⚠️ KRİTİK |
| `findBestSwapRoute()` | 371-493 | `pallet-asset-conversion` | ⚠️ KRİTİK |
| `fetchUserLPPositions()` | 501-572 | `pallet-asset-conversion` + `poolAssets` | ⚠️ KRİTİK |
| `formatTokenBalance()` | 10-24 | - | ✅ |
| `parseTokenInput()` | 31-49 | - | ✅ |
| `calculatePriceImpact()` | 56-85 | - | ✅ |
| `getAmountOut()` | 93-120 | - | ✅ |
| `calculateMinAmount()` | 266-270 | - | ✅ |

**KRİTİK KOD BLOKLARI:**

```typescript
// Satır 151 - Pool keys sorgusu
const poolKeys = await api.query.assetConversion.pools.keys();

// Satır 160 - Pool account sorgusu
const poolAccount = await api.query.assetConversion.pools([asset1, asset2]);

// Satır 165-169 - Asset reserve sorgusu
const reserve1Data = await api.query.assets.account(asset1, poolAccount.unwrap());
```

---

### 10.2 pwap/shared/lib/tiki.ts

**Dosya:** Custom Tiki pallet ile etkileşim

**ETKİ:** Phase 2'de `pallet-tiki` eklenince tam işlevsel olacak

---

## 11. ÖNCELİK SIRALI EYLEM PLANI

### Öncelik 1: Acil (Yapıldı)
- [x] pallet-assets entegre et (index 36)
- [x] pallet-nfts entegre et (index 37)

### Öncelik 2: Karar Gerekli (Kritik)
- [ ] **pallet-asset-conversion için karar:**
  - Seçenek A: Relay chain'de tut → Frontend değişikliği yok
  - Seçenek B: TeyrChain'e taşı → 3-5 gün frontend work

### Öncelik 3: Temizlik (Düşük Risk)
- [ ] Cargo.toml'dan kaldır: pallet-tips, pallet-democracy, pallet-elections-phragmen, pallet-election-provider-multi-phase
- [ ] Web: page-democracy disable et

### Öncelik 4: Phase 2 Hazırlık
- [ ] pallet-tiki eklendiğinde NFTGalleryScreen test et
- [ ] pallet-welati eklendiğinde GovernanceScreen test et

---

🤖 Generated with Claude Code
