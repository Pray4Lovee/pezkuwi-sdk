# Code Audit Report: People Parachain Pallets

**Date:** 2025-11-28
**Auditor:** Gemini
**Status:** ❌ NOT PRODUCTION READY

Runtime entegrasyonu devam ederken yapılan kod incelemesinde, custom pallet'lerde sistem güvenliğini ve istikrarını tehdit eden kritik mantıksal hatalar tespit edilmiştir.

## 1. `pallet-trust` (Kritik)
- **Issue:** `update_all_trust_scores` fonksiyonu `KycStatuses::iter()` ile tüm kullanıcıları tek bir blokta (veya kontrolsüz batch'lerde) gezmeye çalışıyor.
- **Impact:** Kullanıcı sayısı arttığında bu işlem block limitini aşacak ve zinciri durduracaktır (DoS).
- **Fix:** `iter()` yerine cursor tabanlı pagination kullanılmalı veya bu işlem off-chain worker / oracle ile yapılmalı.

## 2. `pallet-perwerde` (Kritik)
- **Issue:** `complete_course` fonksiyonu `ensure_signed(origin)` ile çağrılıyor ve `student` değişkenini `origin`'den alıyor.
- **Impact:** Herhangi bir öğrenci, `complete_course` çağırarak kendi kendine ders tamamlayabilir ve istediği puanı (`points` parametresi) kendine verebilir.
- **Fix:** Bu fonksiyon `ensure_root` veya `ensure_signed` ile sadece ders sahibinin (öğretmenin) çağırabileceği şekilde değiştirilmeli. `student` parametre olarak alınmalı.

## 3. `pallet-identity-kyc` (Yüksek Risk)
- **Issue 1:** `reject_kyc` fonksiyonunda depozito `unreserve` ediliyor (iade ediliyor). Oysa dökümantasyonda ve spam koruması için `slash` edilmesi gerektiği yazıyor.
- **Issue 2:** `confirm_citizenship` fonksiyonu kullanıcının kendini onaylamasına izin veriyor (`ensure_signed(origin)`). Bu, admin onaylı KYC sürecini bypass edebilir.

## 4. `pallet-tiki` (Orta Risk)
- **Issue:** NFT'lerin "Soulbound" (devredilemez) olması `lock_nft_transfer` ile yapılıyor ancak bu kilidin `pallet-nfts` seviyesinde kontrol edilmesi için Runtime tarafında `TransferValidation` hook'unun doğru bağlanması şart. Kodun kendi içinde bu garanti yok.
- **Issue:** Puanlar (`get_bonus_for_tiki`) kod içine gömülü (hardcoded). Değişiklik için runtime upgrade gerekiyor.

## 5. `pallet-referral` (Mantıksal Risk)
- **Issue:** Sistem tamamen KYC'nin "pahalı ve zor" olmasına güveniyor. Eğer KYC aşılabilirse, Sybil saldırısı ile referral puanları manipüle edilebilir.
- **Issue:** `DefaultReferrer` (Founder) tüm sahipsiz referansları topluyor, bu da governance gücünün merkezileşmesine yol açabilir.

## Öneri
Runtime entegrasyonu tamamlandıktan hemen sonra, bu pallet'lerin mantıksal revizyonu yapılmalıdır. Mevcut haliyle canlı ağa alınması önerilmez.
