# SPEC

## Tujuan
Lontar adalah bahasa pemrograman bertipe statis dengan sintaks familier bagi pengguna Rust, dirancang agar lebih mudah dipelajari dan dapat ditargetkan ke beberapa platform.

## Prinsip awal
- Implementasi kompiler ditulis dalam Rust.
- Tidak mengadopsi borrow checker atau lifetime Rust.
- Semantik nilai dibuat sederhana; strategi pengelolaan memori ditetapkan sebelum fitur runtime dibangun.
- Fitur bahasa ditambahkan bertahap, dengan tes pada setiap tahap.

## Sintaks sasaran
`fn`, `let`, `let mut`, `struct`, `enum`, `impl`, `match`, `if/else`, `for`, `while`, anotasi tipe seperti `n: i32`, serta tipe balik `-> i32`.

## Semantik awal
- Tipe statis dengan inferensi lokal.
- Ekspresi terakhir dalam blok menjadi nilai blok.
- Titik koma opsional.
- Diagnostik menyertakan baris dan kolom serta menggunakan bahasa Indonesia.

## Tata bahasa yang sudah didukung parser
```
program    := fn*
fn         := 'fn' nama '(' [param (',' param)* [',']] ')' ['->' tipe] blok
param      := nama ':' tipe
blok       := '{' (pernyataan | ';')* [ekspr] '}'     // ekspr terakhir tanpa ';' = nilai blok
pernyataan := 'let' ['mut'] nama [':' tipe] '=' ekspr | ekspr
ekspr      := 'return' [ekspr] | nama '=' ekspr | rentang   // penugasan: asosiatif kanan, terendah
rentang    := atau [('..' | '..=') atau]                      // tidak bisa dirantai
atau       := dan ('||' dan)*;  dan := banding ('&&' banding)*
banding    := tambah (('==' | '!=' | '<' | '<=' | '>' | '>=') tambah)*
tambah     := kali (('+' | '-') kali)*;  kali := unary (('*' | '/' | '%') unary)*
unary      := ('-' | '!') unary | panggilan
panggilan  := primer ('(' [ekspr (',' ekspr)* [',']] ')')*
primer     := Int | Float | 'true' | 'false' | nama | '(' ekspr ')' | blok | jika | selama | untuk
jika       := 'if' ekspr blok ['else' (jika | blok)]          // ekspresi; syarat tanpa kurung
selama     := 'while' ekspr blok
untuk      := 'for' nama 'in' ekspr blok
```
Tipe saat ini hanya nama sederhana (mis. `i32`). Operator biner asosiatif kiri.

## Aturan titik koma opsional
Pernyataan berakhir di `;`, di `}`, atau saat token berikutnya berada di baris lebih baru. Dua pernyataan dalam satu baris wajib dipisah `;`. `(` hanya memulai panggilan jika satu baris dengan token sebelumnya. Operator biner tetap melanjutkan ekspresi lintas baris. `return` tanpa nilai jika diikuti `;`, `}`, `)`, `,`, akhir berkas, atau baris baru; `else` boleh di baris baru setelah `}`.

## Belum didukung
penugasan ke sasaran selain nama (mis. `a.b = e`), `let` tanpa nilai awal, `struct`/`enum`/`impl`/`match`, akses `.`, string dan char, tipe majemuk (`&T`, generik).

## Arsitektur kompiler
Sumber -> lexer -> parser dan AST -> pemeriksa tipe -> IR sederhana -> backend.

## Tahapan backend
Backend awal akan dipilih setelah bentuk IR ditetapkan. JavaScript merupakan kandidat pertama; backend lain ditambahkan berdasarkan kebutuhan dan tes.

## Verifikasi
Tes tiap tahap dijalankan oleh CI GitHub Actions (`.github/workflows/ci.yml`) pada setiap push ke `main`: `cargo fmt --check`, `cargo build`, `cargo test` pada Rust 1.75.0 dan stable. Fitur dianggap selesai setelah CI hijau.

## Di luar cakupan fondasi
Self-hosting, borrow checker, lifetime, dan pustaka standar luas tidak termasuk tahap awal.
