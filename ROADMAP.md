# ROADMAP
## Status singkat
M0 (fondasi), M1 (lexer), dan M2 (parser ekspresi, `let`, blok, `fn`) selesai dan terverifikasi lokal: `cargo fmt --check`, `cargo build` (0 warning), `cargo test` (38 tes) lulus.
CI GitHub Actions (`.github/workflows/ci.yml`) sudah ditulis; hasil hijau di Actions belum dikonfirmasi.
Berikutnya parser `if`/`while`/`return` dan panggilan fungsi (M2 lanjutan), atau M3 pemeriksa tipe.
## Keputusan desain
- Implementasi kompiler ditulis dalam Rust; bahasa Lontar mengambil sintaks yang familier dari Rust tanpa borrow checker dan lifetime.
- Pipeline: sumber -> lexer -> parser/AST -> pemeriksa tipe -> IR -> backend.
- Tahap awal berfokus pada fondasi kecil, tes tiap tahap, dan umpan balik bertahap.
- Kesalahan nantinya menyertakan baris dan kolom; pesan berbahasa Indonesia.
- Aturan checklist: setiap perubahan dicatat di sini sebagai `[x]`/`[ ]`; item hanya `[x]` jika sudah diverifikasi (tes dijalankan). Gerbang sebelum push: `cargo build` dan `cargo test` lulus, perilaku lama tidak rusak.
- Tanpa dependensi eksternal sampai ada kebutuhan nyata.
## Selesai
- [x] 2026-11-24: Membuat SPEC.md awal dan roadmap proyek.
- [x] 2026-11-25: M0 kode: `Cargo.toml`, `src/lib.rs` (`salam()`), `src/main.rs`, `tests/dasar.rs`, `.gitignore`.
- [x] 2026-11-25: M0 verifikasi: fmt bersih, build sukses, 2 tes lulus, biner mencetak `Lontar v0.1.0`; `Cargo.lock` dicommit.
- [x] 2026-11-25: `CLAUDE.md` diselaraskan dengan keputusan Rust (sebelumnya menyebut Python).
- [x] 2026-11-25: M1 lexer: `src/lexer.rs` (`tokenisasi`, `Token`, `Jenis`) dan `src/kesalahan.rs`. Mendukung kata kunci, identifier, Int/Float, operator, pemisah, komentar `//`, posisi baris:kolom, kesalahan berbahasa Indonesia. 13 tes unit lulus.
- [x] 2026-11-25: M2 langkah 1 parser: `src/parser.rs` (`parse_ekspr`, `Ekspr`, `Op`) mendukung literal, nama, negasi, `+ - * / %`, perbandingan, dan kurung; kesalahan berbahasa Indonesia berposisi. 10 tes unit; total 24 tes lulus, fmt bersih, build 0 warning.
- [x] 2026-11-26: M2 parser `let` (`let [mut] x [: T] = e`), blok (`{ ...; nilai }`), `fn nama(p: T) [-> T] { ... }`, `parse_program`. Titik koma opsional: pernyataan berakhir di `;`, `}`, atau baris baru (dihitung dari `baris` token, `Token` tidak diubah). 14 tes baru; total 38 lulus, fmt bersih, build 0 warning (lokal).
- [ ] 2026-11-26: CI `.github/workflows/ci.yml` ditulis (fmt, build, test; Rust 1.75.0 dan stable); centang setelah hijau di Actions.
## Berikutnya
1. Konfirmasi CI hijau di tab Actions (stable dan 1.75.0); perbaiki jika merah.
2. M2 lanjutan: `if`/`else`, `while`, `return`, panggilan fungsi, `&&`/`||`/`!`.
3. M3: Pemeriksa tipe dasar.
4. M4: IR sederhana.
## Ide / Utang teknis
- Parser belum mengenal panggilan fungsi, `&&`/`||`/`!`, `if`/`while`/`return`, akses `.`, `let` tanpa nilai awal, tipe non-sederhana (`&T`, generik), serta `struct`/`enum`/`impl` tingkat atas. Operator biner melanjutkan ekspresi lintas baris (`a\n+ b` = `a + b`).
- Lexer belum mengenal string, char, `/* */`, pemisah angka `1_000`, dan escape; tambahkan saat dibutuhkan parser.
- Evaluasi backend pertama setelah IR; JavaScript tetap kandidat awal.
- Tentukan cakupan tipe awal dan strategi pengelolaan memori sebelum implementasi runtime.
- Sandbox: `apt-get install cargo rustfmt` (Rust 1.75, tanpa clippy); kode harus kompatibel Rust 1.75 selama dipakai.
