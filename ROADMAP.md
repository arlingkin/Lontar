# ROADMAP
## Status singkat
M0 (fondasi), M1 (lexer), dan M2 langkah 1 (parser ekspresi) selesai dan terverifikasi: `cargo fmt --check`, `cargo build` (0 warning), `cargo test` (24 tes) lulus.
Berikutnya parser `let`, blok, dan `fn` (M2 lanjutan).
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
## Berikutnya
1. M2 lanjutan: parser `let`, blok, dan `fn` (AST pernyataan).
2. M3: Pemeriksa tipe dasar.
3. M4: IR sederhana.
## Ide / Utang teknis
- Parser ekspresi belum mengenal panggilan fungsi, `&&`/`||`/`!`, `if`/blok, dan akses `.`; titik koma opsional belum ditangani.
- Lexer belum mengenal string, char, `/* */`, pemisah angka `1_000`, dan escape; tambahkan saat dibutuhkan parser.
- Titik koma opsional: token belum membawa info "ada baris baru sebelum token ini"; putuskan di M2 (tambah flag di `Token` atau hitung dari `baris`).
- Evaluasi backend pertama setelah IR; JavaScript tetap kandidat awal.
- Tentukan cakupan tipe awal dan strategi pengelolaan memori sebelum implementasi runtime.
- Tambahkan CI GitHub Actions (`cargo test`) agar verifikasi tidak bergantung pada sandbox.
- Sandbox: `apt-get install cargo rustfmt` (Rust 1.75, tanpa clippy); kode harus kompatibel Rust 1.75 selama dipakai.
