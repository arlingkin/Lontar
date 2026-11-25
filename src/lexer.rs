//! Lexer Lontar: mengubah teks sumber menjadi daftar token berposisi.
//! Baris dan kolom dimulai dari 1; kolom dihitung per karakter.

use crate::kesalahan::Kesalahan;

#[derive(Debug, Clone, PartialEq)]
pub enum Jenis {
    // Literal dan nama
    Int(i64),
    Float(f64),
    Ident(String),
    // Kata kunci
    Fn,
    Let,
    Mut,
    Struct,
    Enum,
    Impl,
    Match,
    If,
    Else,
    For,
    In,
    While,
    Return,
    True,
    False,
    // Operator
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AndAnd,
    OrOr,
    Bang,
    Arrow,
    FatArrow,
    Dot,
    DotDot,
    DotDotEq,
    // Pemisah
    Comma,
    Colon,
    ColonColon,
    Semi,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub jenis: Jenis,
    pub baris: usize,
    pub kolom: usize,
}

/// Memecah `sumber` menjadi token; token terakhir selalu `Jenis::Eof`.
pub fn tokenisasi(sumber: &str) -> Result<Vec<Token>, Kesalahan> {
    let mut lexer = Lexer {
        c: sumber.chars().collect(),
        i: 0,
        baris: 1,
        kolom: 1,
    };
    let mut hasil = Vec::new();
    loop {
        let token = lexer.token_berikut()?;
        let selesai = token.jenis == Jenis::Eof;
        hasil.push(token);
        if selesai {
            return Ok(hasil);
        }
    }
}

fn kata_kunci(teks: &str) -> Option<Jenis> {
    Some(match teks {
        "fn" => Jenis::Fn,
        "let" => Jenis::Let,
        "mut" => Jenis::Mut,
        "struct" => Jenis::Struct,
        "enum" => Jenis::Enum,
        "impl" => Jenis::Impl,
        "match" => Jenis::Match,
        "if" => Jenis::If,
        "else" => Jenis::Else,
        "for" => Jenis::For,
        "in" => Jenis::In,
        "while" => Jenis::While,
        "return" => Jenis::Return,
        "true" => Jenis::True,
        "false" => Jenis::False,
        _ => return None,
    })
}

struct Lexer {
    c: Vec<char>,
    i: usize,
    baris: usize,
    kolom: usize,
}

impl Lexer {
    fn lihat(&self) -> Option<char> {
        self.c.get(self.i).copied()
    }

    fn lihat_ke(&self, n: usize) -> Option<char> {
        self.c.get(self.i + n).copied()
    }

    fn maju(&mut self) -> Option<char> {
        let ch = self.lihat()?;
        self.i += 1;
        if ch == '\n' {
            self.baris += 1;
            self.kolom = 1;
        } else {
            self.kolom += 1;
        }
        Some(ch)
    }

    /// Maju satu karakter jika karakter berikutnya sama dengan `diharapkan`.
    fn cocok(&mut self, diharapkan: char) -> bool {
        if self.lihat() == Some(diharapkan) {
            self.maju();
            true
        } else {
            false
        }
    }

    fn lewati_spasi_dan_komentar(&mut self) {
        loop {
            match self.lihat() {
                Some(c) if c.is_whitespace() => {
                    self.maju();
                }
                Some('/') if self.lihat_ke(1) == Some('/') => {
                    while self.lihat().is_some_and(|c| c != '\n') {
                        self.maju();
                    }
                }
                _ => break,
            }
        }
    }

    fn token_berikut(&mut self) -> Result<Token, Kesalahan> {
        self.lewati_spasi_dan_komentar();
        let (baris, kolom) = (self.baris, self.kolom);
        let buat = |jenis: Jenis| Token {
            jenis,
            baris,
            kolom,
        };
        let c = match self.lihat() {
            None => return Ok(buat(Jenis::Eof)),
            Some(c) => c,
        };
        if c.is_ascii_digit() {
            return Ok(buat(self.angka()?));
        }
        if c.is_alphabetic() || c == '_' {
            return Ok(buat(self.nama()));
        }
        self.maju();
        let jenis = match c {
            '+' => Jenis::Plus,
            '*' => Jenis::Star,
            '/' => Jenis::Slash,
            '%' => Jenis::Percent,
            '-' if self.cocok('>') => Jenis::Arrow,
            '-' => Jenis::Minus,
            '=' if self.cocok('=') => Jenis::EqEq,
            '=' if self.cocok('>') => Jenis::FatArrow,
            '=' => Jenis::Assign,
            '!' if self.cocok('=') => Jenis::NotEq,
            '!' => Jenis::Bang,
            '<' if self.cocok('=') => Jenis::LtEq,
            '<' => Jenis::Lt,
            '>' if self.cocok('=') => Jenis::GtEq,
            '>' => Jenis::Gt,
            '&' if self.cocok('&') => Jenis::AndAnd,
            '|' if self.cocok('|') => Jenis::OrOr,
            '&' | '|' => {
                return Err(Kesalahan::baru(
                    format!("'{}' tunggal belum didukung; maksud Anda '{}{}'?", c, c, c),
                    baris,
                    kolom,
                ))
            }
            '.' if self.cocok('.') => {
                if self.cocok('=') {
                    Jenis::DotDotEq
                } else {
                    Jenis::DotDot
                }
            }
            '.' => Jenis::Dot,
            ',' => Jenis::Comma,
            ':' if self.cocok(':') => Jenis::ColonColon,
            ':' => Jenis::Colon,
            ';' => Jenis::Semi,
            '(' => Jenis::LParen,
            ')' => Jenis::RParen,
            '{' => Jenis::LBrace,
            '}' => Jenis::RBrace,
            '[' => Jenis::LBracket,
            ']' => Jenis::RBracket,
            _ => {
                return Err(Kesalahan::baru(
                    format!("karakter tidak dikenal '{}'", c),
                    baris,
                    kolom,
                ))
            }
        };
        Ok(buat(jenis))
    }

    fn ambil_digit(&mut self, teks: &mut String) {
        while let Some(c) = self.lihat().filter(|c| c.is_ascii_digit()) {
            teks.push(c);
            self.maju();
        }
    }

    fn angka(&mut self) -> Result<Jenis, Kesalahan> {
        let (baris, kolom) = (self.baris, self.kolom);
        let mut teks = String::new();
        self.ambil_digit(&mut teks);
        // `1..5` adalah rentang; titik hanya milik angka jika diikuti digit.
        let pecahan =
            self.lihat() == Some('.') && self.lihat_ke(1).is_some_and(|c| c.is_ascii_digit());
        if pecahan {
            teks.push('.');
            self.maju();
            self.ambil_digit(&mut teks);
        }
        if let Some(c) = self.lihat().filter(|c| c.is_alphabetic() || *c == '_') {
            return Err(Kesalahan::baru(
                format!("angka tidak boleh langsung diikuti '{}'", c),
                self.baris,
                self.kolom,
            ));
        }
        if pecahan {
            // Teks hanya berisi digit dan satu titik, jadi parse selalu berhasil.
            Ok(Jenis::Float(teks.parse().unwrap()))
        } else {
            teks.parse::<i64>().map(Jenis::Int).map_err(|_| {
                Kesalahan::baru(
                    format!("bilangan bulat '{}' terlalu besar", teks),
                    baris,
                    kolom,
                )
            })
        }
    }

    fn nama(&mut self) -> Jenis {
        let mut teks = String::new();
        while let Some(c) = self.lihat().filter(|c| c.is_alphanumeric() || *c == '_') {
            teks.push(c);
            self.maju();
        }
        match kata_kunci(&teks) {
            Some(kunci) => kunci,
            None => Jenis::Ident(teks),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jenis(sumber: &str) -> Vec<Jenis> {
        tokenisasi(sumber)
            .unwrap()
            .into_iter()
            .map(|t| t.jenis)
            .collect()
    }

    fn ident(s: &str) -> Jenis {
        Jenis::Ident(s.to_string())
    }

    #[test]
    fn sumber_kosong_hanya_eof() {
        assert_eq!(jenis(""), vec![Jenis::Eof]);
        assert_eq!(jenis("  \n // komentar saja"), vec![Jenis::Eof]);
    }

    #[test]
    fn kata_kunci_dan_identifier() {
        assert_eq!(
            jenis("fn let mut struct enum impl match if else for in while return true false"),
            vec![
                Jenis::Fn,
                Jenis::Let,
                Jenis::Mut,
                Jenis::Struct,
                Jenis::Enum,
                Jenis::Impl,
                Jenis::Match,
                Jenis::If,
                Jenis::Else,
                Jenis::For,
                Jenis::In,
                Jenis::While,
                Jenis::Return,
                Jenis::True,
                Jenis::False,
                Jenis::Eof
            ]
        );
        // Awalan kata kunci tetap identifier.
        assert_eq!(
            jenis("fnx _a nama_2 lets"),
            vec![
                ident("fnx"),
                ident("_a"),
                ident("nama_2"),
                ident("lets"),
                Jenis::Eof
            ]
        );
    }

    #[test]
    fn angka_bulat_dan_pecahan() {
        assert_eq!(
            jenis("0 42 3.14"),
            vec![
                Jenis::Int(0),
                Jenis::Int(42),
                Jenis::Float(3.14),
                Jenis::Eof
            ]
        );
    }

    #[test]
    fn rentang_tidak_dianggap_pecahan() {
        assert_eq!(
            jenis("1..5 1..=5"),
            vec![
                Jenis::Int(1),
                Jenis::DotDot,
                Jenis::Int(5),
                Jenis::Int(1),
                Jenis::DotDotEq,
                Jenis::Int(5),
                Jenis::Eof
            ]
        );
    }

    #[test]
    fn operator_satu_dan_dua_karakter() {
        assert_eq!(
            jenis("+ - * / % = == != < <= > >= && || ! -> => . .. ..="),
            vec![
                Jenis::Plus,
                Jenis::Minus,
                Jenis::Star,
                Jenis::Slash,
                Jenis::Percent,
                Jenis::Assign,
                Jenis::EqEq,
                Jenis::NotEq,
                Jenis::Lt,
                Jenis::LtEq,
                Jenis::Gt,
                Jenis::GtEq,
                Jenis::AndAnd,
                Jenis::OrOr,
                Jenis::Bang,
                Jenis::Arrow,
                Jenis::FatArrow,
                Jenis::Dot,
                Jenis::DotDot,
                Jenis::DotDotEq,
                Jenis::Eof
            ]
        );
    }

    #[test]
    fn pemisah() {
        assert_eq!(
            jenis(", : :: ; ( ) { } [ ]"),
            vec![
                Jenis::Comma,
                Jenis::Colon,
                Jenis::ColonColon,
                Jenis::Semi,
                Jenis::LParen,
                Jenis::RParen,
                Jenis::LBrace,
                Jenis::RBrace,
                Jenis::LBracket,
                Jenis::RBracket,
                Jenis::Eof
            ]
        );
    }

    #[test]
    fn program_fungsi_kecil() {
        assert_eq!(
            jenis("fn tambah(a: i32, b: i32) -> i32 { a + b }"),
            vec![
                Jenis::Fn,
                ident("tambah"),
                Jenis::LParen,
                ident("a"),
                Jenis::Colon,
                ident("i32"),
                Jenis::Comma,
                ident("b"),
                Jenis::Colon,
                ident("i32"),
                Jenis::RParen,
                Jenis::Arrow,
                ident("i32"),
                Jenis::LBrace,
                ident("a"),
                Jenis::Plus,
                ident("b"),
                Jenis::RBrace,
                Jenis::Eof
            ]
        );
    }

    #[test]
    fn komentar_dilewati_dan_posisi_benar() {
        let token = tokenisasi("let x = 1 // satu\n  let y = 22").unwrap();
        let posisi: Vec<(usize, usize)> = token.iter().map(|t| (t.baris, t.kolom)).collect();
        assert_eq!(
            posisi,
            vec![
                (1, 1),
                (1, 5),
                (1, 7),
                (1, 9),
                (2, 3),
                (2, 7),
                (2, 9),
                (2, 11),
                (2, 13)
            ]
        );
        assert_eq!(token[8].jenis, Jenis::Eof);
    }

    #[test]
    fn karakter_tidak_dikenal() {
        let k = tokenisasi("let a = 1\n  @").unwrap_err();
        assert_eq!(k, Kesalahan::baru("karakter tidak dikenal '@'", 2, 3));
    }

    #[test]
    fn dan_atau_tunggal_ditolak_dengan_saran() {
        let k = tokenisasi("a & b").unwrap_err();
        assert_eq!(
            k,
            Kesalahan::baru("'&' tunggal belum didukung; maksud Anda '&&'?", 1, 3)
        );
        assert!(tokenisasi("a | b").is_err());
    }

    #[test]
    fn bilangan_terlalu_besar_dan_angka_menempel_huruf() {
        let k = tokenisasi("x = 9223372036854775808").unwrap_err();
        assert_eq!(
            k,
            Kesalahan::baru("bilangan bulat '9223372036854775808' terlalu besar", 1, 5)
        );
        assert_eq!(jenis("9223372036854775807")[0], Jenis::Int(i64::MAX));
        let k = tokenisasi("12abc").unwrap_err();
        assert_eq!(
            k,
            Kesalahan::baru("angka tidak boleh langsung diikuti 'a'", 1, 3)
        );
    }
}
