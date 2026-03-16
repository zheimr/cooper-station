# Cooper Station — Claude Code Devam Planı

Bu dosyayı Claude Code'a ver, kaldığı yerden devam etsin.

## Proje Bağlamı
- **Repo:** https://github.com/zheimr/cooper-station
- **Ne:** Açık kaynak O'Neill Cylinder uzay habitatı mühendislik projesi
- **Stack:** Rust engine + Three.js landing page + Markdown specs
- **Durum:** İlk commit yapıldı, 33 dosya, 12 modül spec, Rust fizik motoru, 3D landing page mevcut

---

## Görev 1: GitHub Pages Kurulumu
`web/index.html` dosyasını GitHub Pages ile yayınla.

1. `.github/workflows/pages.yml` oluştur:
   - `web/` klasörünü GitHub Pages'e deploy et
   - `main` branch'e push'ta otomatik deploy
   - `actions/upload-pages-artifact` ve `actions/deploy-pages` kullan
2. GitHub CLI ile Pages'i aktif et:
   ```bash
   gh api repos/zheimr/cooper-station/pages -X POST -f source.branch=main -f source.path=/web
   ```
   Ya da workflow-based deployment kullan (GitHub Actions artifact).
3. Doğrula: `https://zheimr.github.io/cooper-station/` açılmalı

---

## Görev 2: GitHub Repo Ayarları
```bash
# Repo açıklaması
gh repo edit zheimr/cooper-station \
  --description "Open-source O'Neill Cylinder space habitat — Rust physics engine, 12 engineering modules, 3D visualization" \
  --homepage "https://zheimr.github.io/cooper-station/"

# Topics
gh api repos/zheimr/cooper-station/topics -X PUT \
  -f "names[]=space" \
  -f "names[]=oneill-cylinder" \
  -f "names[]=rust" \
  -f "names[]=physics-simulation" \
  -f "names[]=space-habitat" \
  -f "names[]=aerospace" \
  -f "names[]=wasm" \
  -f "names[]=open-source" \
  -f "names[]=threejs" \
  -f "names[]=interstellar"
```

---

## Görev 3: Rust Engine — cargo test
Rust engine'deki testleri çalıştır ve CI'ın yeşil olmasını sağla.

```bash
cd engine
cargo test --verbose
cargo clippy -- -D warnings
```

Eğer test failure varsa düzelt. 30+ unit test mevcut:
- `station.rs` — omega, rpm, period, rim velocity hesapları
- `gravity.rs` — gravity gradient, zone mapping, building gradient
- `coriolis.rs` — walking/running/elevator Coriolis etkileri, drop trajectory
- `atmosphere.rs` — pressure gradient, density, ECLSS budget
- `structure.rs` — hoop stress, material analysis, mass breakdown

---

## Görev 4: WASM Build Desteği
Engine'i WebAssembly'ye compile edip landing page'den çağırılabilir yap.

1. `engine/src/wasm.rs` oluştur — wasm-bindgen ile JS-callable API:
   ```rust
   use wasm_bindgen::prelude::*;
   use crate::station::StationConfig;

   #[wasm_bindgen]
   pub fn gravity_at(radius: f64) -> f64 {
       StationConfig::new().gravity_at_radius(radius)
   }

   #[wasm_bindgen]
   pub fn coriolis_at(speed: f64) -> f64 {
       StationConfig::new().coriolis_acceleration(speed)
   }
   // ... daha fazla endpoint
   ```
2. `lib.rs`'e `#[cfg(feature = "wasm")] pub mod wasm;` ekle
3. Build ve test:
   ```bash
   wasm-pack build --target web --features wasm
   ```
4. `web/index.html`'e WASM import ekle — gravity calculator'ı gerçek engine ile çalıştır

---

## Görev 5: Landing Page İyileştirmeleri
`web/index.html` güncellemeleri:

1. **WASM entegrasyonu:** Gravity calculator, Coriolis calculator gerçek Rust engine'den gelsin
2. **Responsive:** Mobil uyumluluğu kontrol et
3. **SEO:** Meta tags, Open Graph tags ekle
4. **Favicon:** Basit bir space/station ikonu
5. **Module tıklanabilirliği:** Her modül kartı ilgili SPEC.md'ye link versin

---

## Görev 6: CI/CD İyileştirmeleri
`.github/workflows/ci.yml` güncelle:

1. Rust test + clippy ✓ (mevcut)
2. WASM build ✓ (mevcut)
3. **Ekle:** GitHub Pages deploy step
4. **Ekle:** Markdown lint (deadlink check)
5. **Ekle:** Release workflow — tag push'ta crates.io publish + WASM artifact

---

## Görev 7 (Opsiyonel): README Görselleri
README.md'ye görsel ekle:

1. `assets/` klasörüne station diagram SVG/PNG
2. Badges: CI status, license, language
   ```markdown
   ![CI](https://github.com/zheimr/cooper-station/actions/workflows/ci.yml/badge.svg)
   ![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
   ```

---

## Öncelik Sırası
1. GitHub Pages (en hızlı impact)
2. Repo ayarları (description, topics)
3. cargo test (CI yeşil olsun)
4. WASM build
5. Landing page iyileştirmeleri
6. CI/CD
7. README görselleri
