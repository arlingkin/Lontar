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

## Arsitektur kompiler
Sumber -> lexer -> parser dan AST -> pemeriksa tipe -> IR sederhana -> backend.

## Tahapan backend
Backend awal akan dipilih setelah bentuk IR ditetapkan. JavaScript merupakan kandidat pertama; backend lain ditambahkan berdasarkan kebutuhan dan tes.

## Di luar cakupan fondasi
Self-hosting, borrow checker, lifetime, dan pustaka standar luas tidak termasuk tahap awal.
