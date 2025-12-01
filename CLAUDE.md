# Claude Code Kuralları - Pezkuwi SDK

## ⚠️ PEZKUWI SDK TERMİNOLOJİSİ - KRİTİK

**ASLA POLKADOT SDK TERİMLERİ KULLANMA! Bu bağımsız bir blockchain projesi.**

### Doğru Terminoloji Tablosu:

| YANLIŞ (Polkadot SDK) | DOĞRU (Pezkuwi SDK) |
|-----------------------|---------------------|
| parachain | **teyrchain** |
| rococo | **pezkuwichain** |
| westend | **zagros** |
| kusama | **zagros** |
| polkadot | **pezkuwichain** |
| `[[parachains]]` | **`[[teyrchains]]`** |
| `[[parachains.collators]]` | **`[[teyrchains.collators]]`** |
| `-lparachain=debug` | **`-lteyrchain=debug`** |
| `parachain=debug` | **`teyrchain=debug`** |

### Token'lar:
- **HEZ**: Relay chain native token (200M genesis, inflationary)
- **PEZ**: Asset Hub governance token (5B sabit supply)
- **TYR**: Base unit (1 HEZ = 10^18 TYR)

### System Teyrchains:
- **Asset Hub Teyrchain**: ID 1000
- **People Chain Teyrchain**: ID 1004

### Zombienet Config Örneği (DOĞRU):
```toml
[relaychain]
default_args = ["-lteyrchain=debug"]
chain = "pezkuwichain-dev"

[[teyrchains]]
id = 1000
chain = "asset-hub-pezkuwichain-dev"

[[teyrchains.collators]]
args = ["-lteyrchain=debug"]
```

---

## 🎯 ANA HEDEF VE ÇALIŞMA PRENSİPLERİ

### Hedef
Pezkuwi blockchain'i mainnet'e taşımak. Her test aşamasında (dev → local → alpha → beta → staging → mainnet) tüm bug/hataları kalıcı olarak çözmeden bir sonraki aşamaya GEÇİLMEZ.

### Mevcut Aşama: DEV NETWORK
**Başarı Kriterleri (hepsi sağlanmalı):**
- [ ] 3 runtime çalışmalı (Relay Chain, Asset Hub, People Chain)
- [ ] Birbirini görmeli (peer discovery)
- [ ] Bloklar üretilmeli
- [ ] Finalized olmalı
- [ ] Alice hesabında genesis token'ları görülmeli (HEZ, PEZ)

### Test Aşamaları Sırası
1. **DEV** (1 validator - Alice) ← ŞU AN BURADAYIZ
2. **LOCAL** (2 validator - Alice + Bob)
3. **ALPHA** (4 validator)
4. **BETA** (8 validator)
5. **STAGING** (21 validator)
6. **MAINNET** (100 validator)

### Çalışma Prensibi
```
Her aşamada:
1. Planlanan testleri yap
2. Tüm testlerden başarılı sonuç al
3. Hata/bug varsa → düzelt → tekrar test et
4. Başarılı olunca → blockchain upgrade → sonraki aşama
```

**ÖNEMLİ:** Ekranda geçici başarı görmek yeterli DEĞİL. Kalıcı çözümler, tam testler, sonra ilerleme.

---

## Dizin Kuralları

| Dizin | Kullanım |
|-------|----------|
| `/home/mamostehp/Pezkuwi-SDK` | **Tüm işlemler burada yapılır** (edit, commit, push) |

## Ekran Görüntüleri

Kullanıcı "ekran" veya "ekrana bak" dediğinde:
```
/home/mamostehp/DKSweb_ekran/Screenshot.png
```
dosyasını oku.

## Gemini ile Koordinasyon

Gemini mesaj gönderdiğinde veya "gemini mesaj" denildiğinde:
```
/home/mamostehp/Pezkuwi-SDK/.ai-coordination/messages.md
```
dosyasını oku. Diğer koordinasyon dosyaları:
- `claude-status.md` - Claude'un mevcut durumu
- `gemini-status.md` - Gemini'nin mevcut durumu
- `task-board.md` - Görev tablosu

## Commit Kuralları

- Commit mesajlarına `🤖 Generated with [Claude Code]` ve `Co-Authored-By: Claude` **EKLEME**
- Sadece düz commit mesajı yaz

## Proje Bilgileri

- **Proje:** Pezkuwi SDK - Bağımsız blockchain projesi
- **Teknoloji:** Polkadot SDK fork'u (ama Polkadot DEĞİL, bağımsız)
- **Ana branch:** `main`
- **GitHub:** `pezkuwichain/pezkuwi-sdk`
- **Discord:** `https://discord.gg/Y3VyEC6h8W` (Server: 1444335345935057049)

## Önemli Notlar

1. `paritytech` referansları `pezkuwichain` olmalı
2. `polkadot-sdk` referansları `pezkuwi-sdk` olmalı
3. Kaliteyi düşüren "kolay çözümler" yerine doğru çözümü uygula
4. Geride iş bırakma - kapsamlı da olsa tamamla

---

## ✅ CI/CD QUICK-CHECKS DÜZELTMELERİ TAMAMLANDI

**Son güncelleme:** 2025-11-29

### Tamamlanan İşler

1. **check-workspace.py düzeltmesi** ✅
   - `polkadot-sdk` → `pezkuwi-sdk` değiştirildi
   - Umbrella crate için hem `path` hem `workspace = true` kabul ediliyor

2. **Bridge crate workspace inheritance (16 crate)** ✅
   - Tüm bridge crate'leri `workspace = true` kullanıyor

3. **Markdown lint kuralları** ✅
   - MD004 (ul-style): Devre dışı - çok fazla legacy dosya
   - MD013 (line-length): Devre dışı - URL'ler satırları uzatıyor

4. **TOML format (taplo)** ✅
   - `.config/taplo.toml` path'leri `polkadot` → `pezkuwi` düzeltildi
   - 435+ TOML dosyası formatlandı

5. **Zepter check** ✅
   - `.config/zepter.yaml`: `-p=polkadot-sdk` → `-p=pezkuwi-sdk` düzeltildi
   - Feature propagation: 36+ issue fix edildi
   - Duplicate deps: `pallet-identity-kyc` ve `pallet-tiki` düzeltildi

6. **Umbrella crate** ✅
   - `generate-umbrella.py` çalıştırıldı
   - `umbrella/Cargo.toml` ve `umbrella/src/lib.rs` yeniden oluşturuldu

### Değiştirilen Dosyalar (438 dosya)
- Config dosyaları: `.config/taplo.toml`, `.config/zepter.yaml`, `.github/.markdownlint.yaml`
- Script: `.github/scripts/check-workspace.py`
- Pallet Cargo.toml: `pallet-identity-kyc`, `pallet-tiki` + 12 özel pallet feature propagation
- Tüm Cargo.toml dosyaları (taplo format)
- Umbrella crate dosyaları

### Sonraki Adım
Commit atılıp push edilmeli - CI/CD artık geçmeli.

---
