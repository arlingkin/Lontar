# ROADMAP
## Status singkat
M0 (fondasi) selesai dan terverifikasi: `cargo fmt --check`, `cargo build`, `cargo test` lulus (cargo 1.75).
Berikutnya lexer (M1).
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
## Berikutnya
1. M1: Lexer minimal (angka, identifier, kata kunci, operator, posisi baris:kolom) dan tes token.
2. M2: Parser ekspresi, `let`, dan `fn`.
3. M3: Pemeriksa tipe dasar.
4. M4: IR sederhana.
## Ide / Utang teknis
- Evaluasi backend pertama setelah IR; JavaScript tetap kandidat awal.
- Tentukan cakupan tipe awal dan strategi pengelolaan memori sebelum implementasi runtime.
- Tambahkan CI GitHub Actions (`cargo test`) agar verifikasi tidak bergantung pada sandbox.
- Sandbox: `apt-get install cargo rustfmt` (Rust 1.75, tanpa clippy); kode harus kompatibel Rust 1.75 selama dipakai.
