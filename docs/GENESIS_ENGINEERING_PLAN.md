# Pezkuwi Network Genesis & Launch Engineering Plan

Bu doküman, Pezkuwi Ağı'nın (Relay Chain, Asset Hub, People Chain) geliştirme aşamasından Mainnet'e kadar olan başlatma süreçlerini, konfigürasyon detaylarını ve test stratejilerini içerir.

## 1. Genel Bakış ve Hedefler

Projenin başarısı, ağın farklı aşamalarda (Dev -> Mainnet) doğru başlangıç durumları (Genesis State) ile ayağa kalkmasına bağlıdır. Bu plan aşağıdaki hedefleri kapsar:

1.  **Asset Tutarlılığı:** Asset Hub üzerinde PEZ (ID 1) ve wHEZ (ID 2) tokenlarının her aşamada var olmasını garanti altına almak.
2.  **Kimlik Önyüklemesi:** People Chain üzerinde "İlk Vatandaş" (Founder) yapısının kurulması.
3.  **Validator Seti Yönetimi:** Ağ büyüklüğüne göre (1 -> 2 -> 4 -> 8 -> 21 -> 100) validator setlerinin doğru yapılandırılması.

---

## 2. Aşamalı Uygulama Planı

### Phase 1: Dev Network (Geliştirme Ortamı)
*Tek node, hızlı blok üretimi, anlık test.*

*   **Hedef:** Tek bir makinede tüm runtime mantığını doğrulamak.
*   **Validator:** 1 (Alice).
*   **Preset Adı:** `DEV_RUNTIME_PRESET` (Mevcut).
*   **Yapılacak Değişiklikler:**
    *   **Asset Hub:** `cumulus/.../asset-hub-pezkuwichain/src/genesis_config_presets.rs` dosyasına `AssetsConfig` (TrustBacked) eklenmeli. PEZ (ID 1) ve wHEZ (ID 2) Alice hesabına tanımlanmalı.
    *   **People Chain:** `cumulus/.../people-pezkuwichain/src/genesis_config_presets.rs` dosyasına `IdentityKyc` bootstrap mantığı eklenmeli (Alice = Vatandaş).
*   **Test Komutu:** `./pezkuwi-omni-node --dev --chain=asset-hub-pezkuwichain-dev`
*   **Başarı Kriteri:** Zincir başlar, Asset Hub'da `Assets` pallet'inde 1 ve 2 ID'li varlıklar sorgulanabilir.

### Phase 2: Local Testnet (Lokal Ağ)
*İki node, konsensüs ve finalization testi.*

*   **Hedef:** İki node (Alice & Bob) arasındaki blok üretimini ve senkronizasyonu test etmek.
*   **Validator:** 2 (Alice, Bob).
*   **Preset Adı:** `LOCAL_TESTNET_RUNTIME_PRESET` (Mevcut).
*   **Yapılacak Değişiklikler:**
    *   **Asset Hub:** Phase 1'deki `AssetsConfig` buraya da uygulanmalı. Varlıklar Alice ve Bob'a dağıtılabilir.
*   **Test Komutu:**
    *   Node A: `./pezkuwi-omni-node --chain=local --alice ...`
    *   Node B: `./pezkuwi-omni-node --chain=local --bob ...`
*   **Başarı Kriteri:** Node'lar birbirini görür, blok üretir ve bloklar "Finalized" durumuna geçer.

### Phase 3: Alpha Testnet (Çekirdek Ekip Testi)
*Dört node, küçük ölçekli ağ simülasyonu.*

*   **Hedef:** 4 validator ile ağ kararlılığını test etmek.
*   **Validator:** 4 (Alice, Bob, Charlie, Dave).
*   **Preset Adı:** `alpha_testnet` (Yeni).
*   **Yapılacak Değişiklikler:**
    *   **Relay Chain:** `pezkuwi/.../genesis_config_presets.rs` dosyasındaki mevcut `versi_local_testnet` preset'i `alpha_testnet` olarak yeniden adlandırılacak.
    *   **Asset Hub & People:** Bu zincirler için de `alpha_testnet` preset'i oluşturulacak (4 collator ile).
*   **Başarı Kriteri:** 4 node sorunsuz çalışır, Asset Hub ve People chain paranchain olarak bağlanır.

### Phase 4: Beta Testnet (Dış Katılımcı Hazırlığı)
*Sekiz node, gerçek key kullanımı.*

*   **Hedef:** Üretilen gerçek güvenli keyler ile ağı başlatmak.
*   **Validator:** 8 (Hardcoded Hex Keys).
*   **Preset Adı:** `beta_testnet` (Yeni).
*   **Yapılacak Değişiklikler:**
    *   **Relay Chain:** `pezkuwi/.../genesis_config_presets.rs` dosyasındaki mevcut `staging_testnet` (içinde 8 key olan) `beta_testnet` olarak yeniden adlandırılacak. Keylerin doğruluğu teyit edilecek.
    *   **Asset Hub & People:** `beta_testnet` preset'i eklenecek.
*   **Başarı Kriteri:** Ağa sadece doğru keylere sahip node'lar validator olarak katılabilir.

### Phase 5: Staging Testnet (Halka Açık Test Öncesi)
*Yirmi bir node, performans ve yük testi.*

*   **Hedef:** Mainnet öncesi son prova.
*   **Validator:** 21 (Yeni Key Seti).
*   **Preset Adı:** `staging_testnet` (Yeni/Güncel).
*   **Yapılacak Değişiklikler:**
    *   **Relay Chain:** `staging_testnet` preset'i 21 validator alacak şekilde yeniden yapılandırılacak. Validator keyleri ya script ile üretilecek ya da manuel eklenecek.
*   **Başarı Kriteri:** 21 validator ile ağ performansı stabil kalmalı.

### Phase 6: Mainnet (Canlı Ağ)
*Yüz node, merkeziyetsiz yapı.*

*   **Hedef:** Canlı ağın başlatılması.
*   **Validator:** 100 (Başlangıçta Sudo/Governance ile seçilenler, sonra NPoS).
*   **Preset Adı:** `mainnet` (Yeni).
*   **Yapılacak Değişiklikler:**
    *   Tüm `genesis_config_presets.rs` dosyalarına `mainnet` preset'i eklenecek.
    *   `chainType` = `Live` olacak.
    *   Bootnode listeleri eklenecek.
*   **Başarı Kriteri:** Genesis block hash'i sabitlenir ve ağ başlar.

---

## 3. Teknik Uygulama Kontrol Listesi (Checklist)

### A. Asset Hub Genesis (`cumulus/.../asset-hub-pezkuwichain/src/genesis_config_presets.rs`)
- [ ] `RuntimeGenesisConfig` içine `AssetsConfig` (Instance1) eklemesi yapıldı mı?
- [ ] Asset ID 1 (PEZ) tanımlandı mı? (Owner: Root/Alice, Sufficient: True)
- [ ] Asset ID 2 (wHEZ) tanımlandı mı? (Owner: Root/Alice, Sufficient: True)
- [ ] `DEV`, `LOCAL`, `ALPHA`, `BETA`, `STAGING`, `MAINNET` presetleri bu assetleri içeriyor mu?

### B. People Chain Genesis (`cumulus/.../people-pezkuwichain/src/genesis_config_presets.rs`)
- [ ] `IdentityKyc` başlangıç durumu (Founder Vatandaş) eklendi mi?
- [ ] `Identity` pallet config'i doğrulandı mı?

### C. Relay Chain Genesis (`pezkuwi/.../genesis_config_presets.rs`)
- [ ] `versi_local_testnet` -> `alpha_testnet` dönüşümü yapıldı mı?
- [ ] `staging_testnet` -> `beta_testnet` dönüşümü yapıldı mı? (8 key korunarak)
- [ ] Yeni `staging_testnet` (21 validator) oluşturuldu mu?
- [ ] Yeni `mainnet` (100 validator) oluşturuldu mu?

### D. Chain Spec Oluşturma
- [ ] `pezkuwi-omni-node build-spec --chain <chain-id>` komutu her preset için JSON çıktısı üretiyor mu?
- [ ] Üretilen JSON içinde `assets` (Asset Hub) ve `identity` (People) bölümleri dolu mu?

---

## 4. Eylem Planı

1.  **Onay:** Bu planın Claude ve Sizin tarafınızdan onaylanması.
2.  **Uygulama (Sıralı):**
    *   Dev & Local düzeltmeleri (Asset Hub & People).
    *   Alpha & Beta refactoring (Relay Chain).
    *   Staging & Mainnet eklemeleri.
3.  **Doğrulama:** Her aşama için `build-spec` çıktısının kontrol edilmesi.
