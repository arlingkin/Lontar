# ROADMAP
## Status singkat
M0 (fondasi): kerangka crate Rust + tes dasar sudah ditulis, tetapi `cargo test` BELUM dijalankan (sandbox tanpa toolchain Rust).
Berikutnya memverifikasi M0, lalu lexer (M1).
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
- [ ] 2026-11-25: M0 verifikasi: `cargo test` lulus (belum dijalankan).
## Berikutnya
1. M0 verifikasi: jalankan `cargo build && cargo test`, commit `Cargo.lock`, centang item di atas.
2. M1: Lexer minimal (angka, identifier, kata kunci, operator, posisi baris:kolom) dan tes token.
3. M2: Parser ekspresi, `let`, dan `fn`.
4. M3: Pemeriksa tipe dasar.
5. M4: IR sederhana.
## Ide / Utang teknis
- Evaluasi backend pertama setelah IR; JavaScript tetap kandidat awal.
- Tentukan cakupan tipe awal dan strategi pengelolaan memori sebelum implementasi runtime.
- `CLAUDE.md` masih menyebut kompiler ditulis dalam Python; selaraskan dengan keputusan Rust di atas.
- Tambahkan CI GitHub Actions (`cargo test`) agar verifikasi tidak bergantung pada sandbox.
