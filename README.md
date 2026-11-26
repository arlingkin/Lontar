# Lontar

Lontar adalah bahasa pemrograman buatan sendiri: sintaks mirip Rust, tetapi lebih ringan (tanpa borrow checker dan lifetime), dengan pesan kesalahan berbahasa Indonesia lengkap dengan nomor baris dan kolom. Kompilernya ditulis dalam Rust tanpa dependensi eksternal.

## Status

Lontar masih tahap awal. Yang sudah jalan: **lexer** dan **parser** (sampai ekspresi, `let`, blok, `fn`, panggilan fungsi). Belum ada pemeriksa tipe, IR, backend, maupun cara menjalankan program Lontar. `cargo run` baru mencetak versi.

Detail kemajuan ada di [ROADMAP.md](ROADMAP.md), rancangan bahasa di [SPEC.md](SPEC.md).

## Contoh sintaks yang sudah bisa diurai

```rust
fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x: i32 = tambah(2, 3) * 4
    let ok = x > 10 && !false
    x
}
```

Titik koma opsional; ekspresi terakhir blok (tanpa `;`) menjadi nilai blok.

## Cara menggunakan

Prasyarat: Rust (diuji di 1.75.0 dan stable).

| Perintah | Fungsi |
|---|---|
| `cargo build` | Membangun kompiler |
| `cargo run` | Mencetak `Lontar v0.1.0` |
| `cargo test` | Menjalankan seluruh tes |
| `cargo fmt --check` | Memeriksa format (juga diperiksa CI) |

Saat ini Lontar dipakai sebagai pustaka Rust. Tambahkan di `Cargo.toml` proyekmu:

```toml
[dependencies]
lontar = { git = "https://github.com/arlingkin/Lontar" }
```

Lalu:

```rust
use lontar::{lexer::tokenisasi, parser::parse_program};

fn main() {
    let sumber = "fn f(a: i32) -> i32 { a + 1 }";
    let token = tokenisasi(sumber).expect("lexer gagal");
    match parse_program(&token) {
        Ok(program) => println!("{} fungsi", program.fungsi.len()),
        Err(e) => eprintln!("{}", e),
    }
}
```

Contoh pesan kesalahan untuk `fn f( {`:

```
kesalahan di baris 1, kolom 7: diharapkan nama parameter, ditemukan '{'
```

## Daftar kode

| Berkas | Isi utama | Penjelasan |
|---|---|---|
| `src/lib.rs` | `salam()`, `NAMA`, `VERSI` | Pintu masuk pustaka; mendaftarkan modul dan menyediakan teks identitas kompiler. |
| `src/main.rs` | `main` | Biner `lontar`; sementara hanya mencetak `salam()`. |
| `src/kesalahan.rs` | `Kesalahan` | Kesalahan kompilasi (`pesan`, `baris`, `kolom`), tampil sebagai `kesalahan di baris B, kolom K: pesan`. |
| `src/lexer.rs` | `tokenisasi`, `Token`, `Jenis` | Mengubah teks sumber menjadi token berposisi: angka (Int/Float), nama, kata kunci, operator, pemisah, komentar `//` dilewati. Token terakhir selalu `Eof`. |
| `src/parser.rs` | `parse_ekspr`, `parse_program`, `Ekspr`, `Pernyataan`, `Blok`, `Fungsi`, `Program`, `Op` | Mengubah token menjadi AST. `parse_ekspr` mengurai satu ekspresi; `parse_program` mengurai deretan `fn` tingkat atas. |
| `tests/dasar.rs` | tes integrasi | Memastikan biner mencetak nama dan versi. |
| `Cargo.toml`, `Cargo.lock` | | Konfigurasi proyek (edition 2021, tanpa dependensi). |
| `.github/workflows/ci.yml` | | CI GitHub Actions: `cargo fmt --check`, `cargo build`, `cargo test` pada Rust 1.75.0 dan stable, dijalankan setiap push ke `main` dan pada PR. |

### Presedensi operator (rendah ke tinggi)

`||` → `&&` → perbandingan (`== != < <= > >=`) → `+ -` → `* / %` → unary (`-`, `!`) → panggilan `f(...)` → primer (literal, nama, `( )`, `{ }`).

## Dokumen proyek

| Berkas | Fungsi |
|---|---|
| `SPEC.md` | Rancangan dan tata bahasa bahasa Lontar |
| `ROADMAP.md` | Memori antar sesi: status, keputusan, checklist, tugas berikutnya |
| `CLAUDE.md` | Instruksi kerja untuk asisten AI yang membantu membangun proyek ini |

## Lisensi

Apache-2.0, lihat [LICENSE](LICENSE).
