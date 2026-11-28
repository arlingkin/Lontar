# ROADMAP
## Status singkat
M0 (fondasi), M1 (lexer), dan M2 (parser lengkap: ekspresi, `let`, blok, `fn`, `if`/`while`/`for`/`return`, penugasan, `struct`/`enum`/`impl`) selesai. M3 (pemeriksa tipe) dimulai: langkah 1 (`src/tipe.rs`, aturan tipe operator) selesai; 86 tes lulus, CI hijau pada f2abf26.
Keputusan posisi pada AST sudah diambil (lihat Keputusan desain). **Tugas sesi berikutnya: M3 langkah 2 (rincian di Berikutnya #1).**
## Keputusan desain
- Implementasi kompiler ditulis dalam Rust; bahasa Lontar mengambil sintaks yang familier dari Rust tanpa borrow checker dan lifetime.
- Pipeline: sumber -> lexer -> parser/AST -> pemeriksa tipe -> IR -> backend.
- Tahap awal berfokus pada fondasi kecil, tes tiap tahap, dan umpan balik bertahap.
- Kesalahan nantinya menyertakan baris dan kolom; pesan berbahasa Indonesia.
- Aturan checklist: setiap perubahan dicatat di sini sebagai `[x]`/`[ ]`; item kode hanya `[x]` jika run CI GitHub Actions untuk commit itu hijau (fmt, build, test; Rust 1.75.0 dan stable). Cargo lokal opsional; CI adalah penentu. Jika CI merah, perbaiki sebelum menambah fitur.
- Tanpa dependensi eksternal sampai ada kebutuhan nyata. Dependensi baru hanya dengan persetujuan eksplisit; catat alasan di sini dan pin versi eksak.
- Versi: MSRV Rust 1.75.0 (CI menguji 1.75.0 dan stable); `Cargo.lock` di-commit dan CI memakai `--locked`. `version` di `Cargo.toml` (SemVer 0.x) hanya dinaikkan saat milestone ditutup, di commit tersendiri, bersama pembaruan README dan ROADMAP.
- Posisi pada AST (diputuskan 2026-11-28): `Posisi { baris, kolom }` = token pertama node. `Ekspr` menjadi struct `{ jenis: JenisEkspr, pos }` dengan `PartialEq` yang mengabaikan `pos` (tes AST lama tetap valid). Deklarasi dan nama tipe berposisi di langkah terpisah. Detail di SPEC.md.
- Pemeriksa tipe melapor kesalahan pertama saja (`Result<_, Kesalahan>`); pelaporan banyak kesalahan menyusul.
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
- [x] 2026-11-26: M2 langkah 6 parser: `struct Nama { a: T, }`, `enum Nama { A, B, }` (varian tanpa data), `impl Nama { fn .. }` tingkat atas; `Program` kini punya `struktur`, `enumerasi`, `impls` (dikelompokkan per jenis). 8 tes baru; total 78 lulus, fmt bersih, 0 warning; CI hijau pada 017a5ef. M2 selesai.
- [x] 2026-11-26: Ikon proyek `icon.png` (512x512) dan ditampilkan rata tengah di bagian atas `README.md`.
- [x] 2026-11-27: M3 langkah 1: `src/tipe.rs` (`Tipe` = `Int`/`Float`/`Bool`/`Unit`; `dari_nama`; `tipe_biner`, `tipe_negasi`, `tipe_tidak`), aturan di SPEC.md. 8 tes baru; 86 tes lulus, fmt bersih, 0 warning; CI hijau pada f2abf26.
- [x] 2026-11-28: Serah-terima sesi: keputusan posisi AST, rincian M3 langkah 2, peta jalan sampai selesai (ROADMAP.md); rancangan posisi AST dan pemeriksa tipe (SPEC.md); RITUAL SESI wajib awal/akhir, aturan dependensi dan versi (CLAUDE.md). Hanya dokumen; kode tidak berubah.
## Berikutnya
1. **M3 langkah 2: posisi pada `Ekspr`** (tugas sesi depan, ambil apa adanya)
   - Tujuan: tiap node ekspresi tahu baris:kolom token pertamanya, agar pemeriksa tipe bisa melapor `kesalahan di baris B, kolom K`.
   - Berkas: `src/parser.rs` (utama); perbarui SPEC.md (bagian Posisi pada AST) dan README.md (daftar kode).
   - Langkah:
     1. Tambah `pub struct Posisi { pub baris: usize, pub kolom: usize }` (derive Debug, Clone, Copy, PartialEq, Eq).
     2. Ubah `pub enum Ekspr` menjadi `pub enum JenisEkspr` (isi tidak berubah); buat `pub struct Ekspr { pub jenis: JenisEkspr, pub pos: Posisi }` dengan `PartialEq` manual yang hanya membandingkan `jenis`. Anak node (`Box<Ekspr>`, `Vec<Ekspr>`, `Blok.nilai`) tetap bertipe `Ekspr`.
     3. Parser mengisi `pos` = posisi token pertama node (`Biner`: token pertama operan kiri). Perilaku parsing tidak boleh berubah.
     4. Sesuaikan pemakaian `Ekspr::X(...)`: sekitar 19 titik di implementasi dan 34 di tes (64 tes parser). Cara hemat: ubah helper tes (`nama`, `int`, `biner`, dst.) agar membungkus `Posisi` kosong; tambah helper `ekspr(jenis)` untuk situs lain.
     5. Tambah minimal 4 tes posisi: `a + b` pada (1,1); argumen `f(x)` pada kolom yang benar; ekspresi di baris ke-2 blok; `if` bersarang.
   - Selesai bila: semua tes lama lulus tanpa mengubah ekspektasi, tes posisi lulus, `cargo fmt --check` bersih, 0 warning, CI hijau; SPEC.md dan README.md cocok dengan kenyataan.
   - Awas: jangan menambah `pos` ke deklarasi atau nama tipe di langkah ini (itu langkah 3); jangan mengubah `src/tipe.rs`.
2. M3 langkah 3: posisi pada deklarasi (`Pernyataan::Let`, `Param`, `Fungsi`, nama/medan/varian `struct`/`enum`/`impl`) dan nama tipe (`tipe: String` menjadi `NamaTipe { nama, pos }`, `PartialEq` mengabaikan `pos`).
3. M3 langkah 4: `src/pemeriksa.rs` dengan `Lingkungan` (lingkup variabel) dan `periksa_ekspr`: literal, nama (`nama 'x' belum didefinisikan`), operator (pakai `tipe.rs`), blok, `let` (anotasi tipe). Kesalahan berposisi.
4. M3 langkah 5: kontrol alur: `if`/`else`, `while`, `for` + rentang, `return`, penugasan (variabel harus `mut`).
5. M3 langkah 6: fungsi dan panggilan (jumlah dan tipe argumen, tipe balik) dan `periksa_program(&Program) -> Result<(), Kesalahan>`.
## Peta jalan sampai selesai
- M3 selesai bila: `periksa_program` menerima program contoh valid dan menolak tiap kategori kesalahan (tipe tak sama, nama tak ada, argumen salah, penugasan ke non-`mut`, dst.) dengan pesan berposisi; ada tes per aturan; SPEC.md bagian Tipe lengkap. Langkah 7: tipe pengguna (`struct`/`enum`/`impl`).
- M2b (susulan, sebelum M5): lexer string/char/escape/`/* */`; parser akses `.`, literal struct, `match`, varian enum berdata; tiap fitur dengan tes dan SPEC.
- M4 IR: `src/ir.rs` dan `turunkan(&Program) -> Ir`; putuskan di awal apakah memakai AST berketik; selesai bila program contoh diturunkan dan format IR terdokumentasi di SPEC.md.
- M5 backend JavaScript: `ke_js(&Ir) -> String`; selesai bila tes snapshot lulus dan satu tes menjalankan hasilnya dengan `node` (tersedia di runner CI).
- M6 `match` dan enum lengkap lintas pemeriksa, IR, dan JS.
- M7 backend Python, lalu C (satu backend per sesi).
- M8 CLI (`lontar berkas.lontar --target js`), stdlib mini (cetak), playground, contoh program di `contoh/` yang diuji CI.
- Penutupan proyek (rapi): semua milestone `[x]`, README dan SPEC lengkap dan akurat, tidak ada catatan usang di ROADMAP, versi dinaikkan sesuai aturan, CI hijau.
## Ide / Utang teknis
- Tipe awal menyatukan semua ukuran bilangan (`Int`, `Float`); pisahkan (`i32` vs `i64`) bila backend membutuhkannya.
- Parser belum mengenal akses `.`, `self`/metode dengan penerima, varian enum berdata, `match`, penugasan ke sasaran selain nama, `let` tanpa nilai awal, dan tipe non-sederhana (`&T`, generik). Operator biner melanjutkan ekspresi lintas baris (`a\n+ b` = `a + b`).
- Lexer belum mengenal string, char, `/* */`, pemisah angka `1_000`, dan escape; tambahkan saat dibutuhkan parser.
- Evaluasi backend pertama setelah IR; JavaScript tetap kandidat awal.
- Tentukan strategi pengelolaan memori sebelum implementasi runtime (cakupan tipe awal sudah diputuskan: Int/Float/Bool/Unit).
- Sandbox (jika perlu cargo lokal): `apt-get update && apt-get install cargo rustfmt` (Rust 1.75, tanpa clippy); kode harus kompatibel Rust 1.75 selama dipakai.
