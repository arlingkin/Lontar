# ROADMAP
## Status singkat
M0 (fondasi), M1 (lexer), dan M2 sampai langkah 3 (ekspresi lengkap, `let`, blok, `fn`) selesai dan M2 langkah 4 (`if`/`else`, `while`, `return`, penugasan) selesai dan CI hijau pada e278bf1. M2 langkah 5 (`for x in a..b`/`..=`) selesai: fmt bersih, 0 warning, 70 tes lulus; CI Actions hijau pada a6891a0.
Berikutnya parser `struct`/`enum`/`impl`.
## Keputusan desain
- Implementasi kompiler ditulis dalam Rust; bahasa Lontar mengambil sintaks yang familier dari Rust tanpa borrow checker dan lifetime.
- Pipeline: sumber -> lexer -> parser/AST -> pemeriksa tipe -> IR -> backend.
- Tahap awal berfokus pada fondasi kecil, tes tiap tahap, dan umpan balik bertahap.
- Kesalahan nantinya menyertakan baris dan kolom; pesan berbahasa Indonesia.
- Aturan checklist: setiap perubahan dicatat di sini sebagai `[x]`/`[ ]`; item kode hanya `[x]` jika run CI GitHub Actions untuk commit itu hijau (fmt, build, test; Rust 1.75.0 dan stable). Cargo lokal opsional; CI adalah penentu. Jika CI merah, perbaiki sebelum menambah fitur.
- Tanpa dependensi eksternal sampai ada kebutuhan nyata.
## Selesai
- [x] 2026-11-24: Membuat SPEC.md awal dan roadmap proyek.
- [x] 2026-11-25: M0 kode: `Cargo.toml`, `src/lib.rs` (`salam()`), `src/main.rs`, `tests/dasar.rs`, `.gitignore`.
- [x] 2026-11-25: M0 verifikasi: fmt bersih, build sukses, 2 tes lulus, biner mencetak `Lontar v0.1.0`; `Cargo.lock` dicommit.
- [x] 2026-11-25: `CLAUDE.md` diselaraskan dengan keputusan Rust (sebelumnya menyebut Python).
- [x] 2026-11-25: M1 lexer: `src/lexer.rs` (`tokenisasi`, `Token`, `Jenis`) dan `src/kesalahan.rs`. Mendukung kata kunci, identifier, Int/Float, operator, pemisah, komentar `//`, posisi baris:kolom, kesalahan berbahasa Indonesia. 13 tes unit lulus.
- [x] 2026-11-25: M2 langkah 1 parser: `src/parser.rs` (`parse_ekspr`, `Ekspr`, `Op`) mendukung literal, nama, negasi, `+ - * / %`, perbandingan, dan kurung; kesalahan berbahasa Indonesia berposisi. 10 tes unit; total 24 tes lulus, fmt bersih, build 0 warning.
- [x] 2026-11-26: M2 parser `let` (`let [mut] x [: T] = e`), blok (`{ ...; nilai }`), `fn nama(p: T) [-> T] { ... }`, `parse_program`. Titik koma opsional: pernyataan berakhir di `;`, `}`, atau baris baru (dihitung dari `baris` token, `Token` tidak diubah). 14 tes baru; total 38 lulus, fmt bersih, build 0 warning (lokal).
- [x] 2026-11-26: CI `.github/workflows/ci.yml` (fmt, build, test; Rust 1.75.0 dan stable) hijau di Actions pada commit 7f0f554.
- [x] 2026-11-26: Dokumen diselaraskan dengan alur CI: `README.md` (status, contoh, cara pakai, daftar kode), `SPEC.md` (tata bahasa dan aturan titik koma), `CLAUDE.md` (verifikasi lewat CI, format penutup). Contoh README dijalankan lokal; CI commit 92eb3d7 hijau.
- [x] 2026-11-26: M2 langkah 3 parser: `&&`, `||`, `!`, dan panggilan fungsi `f(a, b,)` (juga berantai/bersarang; `(` di baris baru bukan panggilan). 8 tes baru; total 46 lulus, fmt bersih, build 0 warning; CI hijau pada 92eb3d7.
- [x] 2026-11-26: M2 langkah 4 parser: `if`/`else`/`else if` (ekspresi, syarat tanpa kurung), `while`, `return [e]`, penugasan `x = e` (asosiatif kanan, terendah; sasaran harus nama). 16 tes baru; total 62 lulus, fmt bersih, 0 warning; CI hijau pada e278bf1.
- [x] 2026-11-26: M2 langkah 5 parser: `for nama in ekspr { .. }` dan rentang `a..b`/`a..=b` (di atas penugasan, di bawah `||`; tanpa rantai; kedua ujung wajib). 8 tes baru; total 70 lulus, fmt bersih, 0 warning; CI hijau pada a6891a0.
## Berikutnya
1. M2 penutup: `struct`/`enum`/`impl` tingkat atas (boleh dipecah).
2. M3: Pemeriksa tipe dasar.
3. M4: IR sederhana.
## Ide / Utang teknis
- Parser belum mengenal akses `.`, penugasan ke sasaran selain nama, `let` tanpa nilai awal, tipe non-sederhana (`&T`, generik), serta `struct`/`enum`/`impl` tingkat atas. Operator biner melanjutkan ekspresi lintas baris (`a\n+ b` = `a + b`).
- Lexer belum mengenal string, char, `/* */`, pemisah angka `1_000`, dan escape; tambahkan saat dibutuhkan parser.
- Evaluasi backend pertama setelah IR; JavaScript tetap kandidat awal.
- Tentukan cakupan tipe awal dan strategi pengelolaan memori sebelum implementasi runtime.
- Sandbox (jika perlu cargo lokal): `apt-get update && apt-get install cargo rustfmt` (Rust 1.75, tanpa clippy); kode harus kompatibel Rust 1.75 selama dipakai.
