# ROADMAP
## Status singkat
Dokumen fondasi Lontar dibuat berdasarkan `CLAUDE.md` dan arahan Rust sebagai implementasi kompiler.
M0 dimulai; belum ada kode yang ditulis.
## Keputusan desain
- Implementasi kompiler ditulis dalam Rust; bahasa Lontar mengambil sintaks yang familier dari Rust tanpa borrow checker dan lifetime.
- Pipeline: sumber -> lexer -> parser/AST -> pemeriksa tipe -> IR -> backend.
- Tahap awal berfokus pada fondasi kecil, tes tiap tahap, dan umpan balik bertahap.
- Kesalahan nantinya menyertakan baris dan kolom; pesan berbahasa Indonesia.
## Selesai
- 2026-11-24: Membuat SPEC.md awal dan roadmap proyek.
## Berikutnya
1. M0: Buat kerangka crate Rust dan tes dasar.
2. M1: Lexer minimal dan tes token.
3. M2: Parser ekspresi, `let`, dan `fn`.
4. M3: Pemeriksa tipe dasar.
5. M4: IR sederhana.
## Ide / Utang teknis
- Evaluasi backend pertama setelah IR; JavaScript tetap kandidat awal.
- Tentukan cakupan tipe awal dan strategi pengelolaan memori sebelum implementasi runtime.
