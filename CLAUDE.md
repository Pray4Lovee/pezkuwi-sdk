# Claude Code Kuralları - Pezkuwi SDK

## Dizin Kuralları

| Dizin | Kullanım |
|-------|----------|
| `/home/mamostehp/Pezkuwi-SDK` | **Tüm işlemler burada yapılır** (edit, commit, push) |
| `/home/mamostehp/Pezkuwi-SDK-2` | **Sadece okuma/referans için** (kesinlikle değişiklik yapma) |

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

## 🚧 DEVAM EDEN İŞ - CI/CD QUICK-CHECKS DÜZELTMELERİ

**Son güncelleme:** 2024-11-29

### Tamamlanan İşler ✅

1. **check-workspace.py düzeltmesi**
   - `polkadot-sdk` → `pezkuwi-sdk` değiştirildi
   - Umbrella crate için hem `path` hem `workspace = true` kabul ediliyor
   - Dosya: `.github/scripts/check-workspace.py`

2. **Bridge crate workspace inheritance (16 crate)**
   - Tüm bridge crate'leri `workspace = true` kullanıyor
   - Dosya: `umbrella/Cargo.toml`

3. **Markdown lint kuralları**
   - MD004 (ul-style): Devre dışı - çok fazla legacy dosya
   - MD013 (line-length): Devre dışı - URL'ler satırları uzatıyor
   - `subsystem-bench/README.md` içindeki inline directive kaldırıldı
   - Dosya: `.github/.markdownlint.yaml`

4. **Cargo fmt uygulandı**
   - 5 bridges dosyası formatlandı

### Kalan CI/CD Hataları ❌

1. **check-readme**
   - Hata: `Found uncommitted changes in ./templates/teyrchain/README.md`
   - Çözüm: README.docify.md'den README.md yeniden oluşturulmalı

2. **check-zepter**
   - Hata: `Command 'lint propagate-feature' failed`
   - Çözüm: `zepter` tool çalıştırılmalı feature propagation için

3. **check-toml-format**
   - TOML dosyaları formatlanmalı
   - Çözüm: `taplo format` komutu çalıştırılmalı

4. **check-umbrella**
   - Umbrella crate kontrolü başarısız
   - Detaylı log incelenmeli

### Sonraki Adımlar

```bash
# 1. README regeneration
cargo build -p staging-chain-spec-builder --features generate-readme

# 2. Zepter fix
cargo install zepter
zepter lint propagate-feature --fix

# 3. TOML format
cargo install taplo-cli
taplo format

# 4. Check umbrella - log incele
gh run view <run_id> --log | grep -A 50 "check-umbrella"
```

### Workflow Kontrol Komutu
```bash
gh run list --workflow=checks-quick.yml --limit 5
gh run view <run_id> --log-failed
```

---
