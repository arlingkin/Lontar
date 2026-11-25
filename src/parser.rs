//! Parser Lontar (M2, langkah 1): literal, nama, negasi, operator aritmetika
//! dan perbandingan, serta kurung. Presedensi dari terendah: perbandingan,
//! `+ -`, `* / %`, negasi unary, lalu primer. Asosiatif kiri.

use crate::kesalahan::Kesalahan;
use crate::lexer::{Jenis, Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Tambah,
    Kurang,
    Kali,
    Bagi,
    Sisa,
    Sama,
    BedaDari,
    KurangDari,
    KurangSama,
    LebihDari,
    LebihSama,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ekspr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Nama(String),
    Negasi(Box<Ekspr>),
    Biner {
        op: Op,
        kiri: Box<Ekspr>,
        kanan: Box<Ekspr>,
    },
}

/// Mengurai satu ekspresi dari token hasil `tokenisasi`; seluruh token harus habis.
pub fn parse_ekspr(token: &[Token]) -> Result<Ekspr, Kesalahan> {
    if token.is_empty() {
        return Err(Kesalahan::baru(
            "diharapkan ekspresi, ditemukan akhir berkas",
            1,
            1,
        ));
    }
    let mut p = Parser { token, pos: 0 };
    let e = p.banding()?;
    let sisa = p.lihat();
    if sisa.jenis != Jenis::Eof {
        return Err(Kesalahan::baru(
            format!(
                "token tidak terduga setelah ekspresi: {}",
                deskripsi(&sisa.jenis)
            ),
            sisa.baris,
            sisa.kolom,
        ));
    }
    Ok(e)
}

struct Parser<'a> {
    token: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn lihat(&self) -> &'a Token {
        let toks: &'a [Token] = self.token;
        &toks[self.pos]
    }

    /// Maju satu token; tidak melewati `Eof` terakhir.
    fn maju(&mut self) {
        if self.pos + 1 < self.token.len() {
            self.pos += 1;
        }
    }

    fn banding(&mut self) -> Result<Ekspr, Kesalahan> {
        let mut kiri = self.tambah()?;
        while let Some(op) = op_banding(&self.lihat().jenis) {
            self.maju();
            let kanan = self.tambah()?;
            kiri = biner(op, kiri, kanan);
        }
        Ok(kiri)
    }

    fn tambah(&mut self) -> Result<Ekspr, Kesalahan> {
        let mut kiri = self.kali()?;
        while let Some(op) = op_tambah(&self.lihat().jenis) {
            self.maju();
            let kanan = self.kali()?;
            kiri = biner(op, kiri, kanan);
        }
        Ok(kiri)
    }

    fn kali(&mut self) -> Result<Ekspr, Kesalahan> {
        let mut kiri = self.unary()?;
        while let Some(op) = op_kali(&self.lihat().jenis) {
            self.maju();
            let kanan = self.unary()?;
            kiri = biner(op, kiri, kanan);
        }
        Ok(kiri)
    }

    fn unary(&mut self) -> Result<Ekspr, Kesalahan> {
        if self.lihat().jenis == Jenis::Minus {
            self.maju();
            let e = self.unary()?;
            return Ok(Ekspr::Negasi(Box::new(e)));
        }
        self.primer()
    }

    fn primer(&mut self) -> Result<Ekspr, Kesalahan> {
        let t = self.lihat();
        let e = match &t.jenis {
            Jenis::Int(n) => Ekspr::Int(*n),
            Jenis::Float(f) => Ekspr::Float(*f),
            Jenis::True => Ekspr::Bool(true),
            Jenis::False => Ekspr::Bool(false),
            Jenis::Ident(s) => Ekspr::Nama(s.clone()),
            Jenis::LParen => {
                self.maju();
                let e = self.banding()?;
                let tutup = self.lihat();
                if tutup.jenis != Jenis::RParen {
                    return Err(Kesalahan::baru(
                        format!("diharapkan ')', ditemukan {}", deskripsi(&tutup.jenis)),
                        tutup.baris,
                        tutup.kolom,
                    ));
                }
                self.maju();
                return Ok(e);
            }
            _ => {
                return Err(Kesalahan::baru(
                    format!("diharapkan ekspresi, ditemukan {}", deskripsi(&t.jenis)),
                    t.baris,
                    t.kolom,
                ))
            }
        };
        self.maju();
        Ok(e)
    }
}

fn biner(op: Op, kiri: Ekspr, kanan: Ekspr) -> Ekspr {
    Ekspr::Biner {
        op,
        kiri: Box::new(kiri),
        kanan: Box::new(kanan),
    }
}

fn op_banding(j: &Jenis) -> Option<Op> {
    match j {
        Jenis::EqEq => Some(Op::Sama),
        Jenis::NotEq => Some(Op::BedaDari),
        Jenis::Lt => Some(Op::KurangDari),
        Jenis::LtEq => Some(Op::KurangSama),
        Jenis::Gt => Some(Op::LebihDari),
        Jenis::GtEq => Some(Op::LebihSama),
        _ => None,
    }
}

fn op_tambah(j: &Jenis) -> Option<Op> {
    match j {
        Jenis::Plus => Some(Op::Tambah),
        Jenis::Minus => Some(Op::Kurang),
        _ => None,
    }
}

fn op_kali(j: &Jenis) -> Option<Op> {
    match j {
        Jenis::Star => Some(Op::Kali),
        Jenis::Slash => Some(Op::Bagi),
        Jenis::Percent => Some(Op::Sisa),
        _ => None,
    }
}

fn deskripsi(j: &Jenis) -> String {
    match j {
        Jenis::Int(n) => format!("angka '{}'", n),
        Jenis::Float(f) => format!("angka '{}'", f),
        Jenis::Ident(s) => format!("nama '{}'", s),
        Jenis::Eof => "akhir berkas".to_string(),
        _ => "token tidak terduga".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenisasi;

    fn parse(s: &str) -> Result<Ekspr, Kesalahan> {
        let t = tokenisasi(s)?;
        parse_ekspr(&t)
    }

    fn nama(s: &str) -> Ekspr {
        Ekspr::Nama(s.to_string())
    }

    fn int(n: i64) -> Ekspr {
        Ekspr::Int(n)
    }

    #[test]
    fn angka_tunggal() {
        assert_eq!(parse("42").unwrap(), int(42));
    }

    #[test]
    fn kali_lebih_kuat_dari_tambah() {
        assert_eq!(
            parse("1 + 2 * 3").unwrap(),
            biner(Op::Tambah, int(1), biner(Op::Kali, int(2), int(3)))
        );
    }

    #[test]
    fn kurang_asosiatif_kiri() {
        assert_eq!(
            parse("10 - 4 - 3").unwrap(),
            biner(Op::Kurang, biner(Op::Kurang, int(10), int(4)), int(3))
        );
    }

    #[test]
    fn kurung_mengubah_urutan() {
        assert_eq!(
            parse("(1 + 2) * 3").unwrap(),
            biner(Op::Kali, biner(Op::Tambah, int(1), int(2)), int(3))
        );
    }

    #[test]
    fn negasi_unary_lebih_kuat_dari_kali() {
        assert_eq!(
            parse("-x * 2").unwrap(),
            biner(Op::Kali, Ekspr::Negasi(Box::new(nama("x"))), int(2))
        );
    }

    #[test]
    fn perbandingan_paling_rendah() {
        assert_eq!(
            parse("a + 1 == b * 2").unwrap(),
            biner(
                Op::Sama,
                biner(Op::Tambah, nama("a"), int(1)),
                biner(Op::Kali, nama("b"), int(2))
            )
        );
    }

    #[test]
    fn literal_float_dan_bool() {
        assert_eq!(parse("2.5").unwrap(), Ekspr::Float(2.5));
        assert_eq!(parse("true").unwrap(), Ekspr::Bool(true));
    }

    #[test]
    fn kurung_tidak_tertutup_ditolak() {
        let e = parse("(1 + 2").unwrap_err();
        assert!(e.pesan.contains("diharapkan ')'"), "{}", e);
        assert_eq!((e.baris, e.kolom), (1, 7));
    }

    #[test]
    fn token_sisa_ditolak() {
        let e = parse("1 2").unwrap_err();
        assert!(e.pesan.contains("setelah ekspresi"), "{}", e);
        assert_eq!(e.kolom, 3);
    }

    #[test]
    fn ekspresi_kosong_ditolak() {
        let e = parse("").unwrap_err();
        assert!(e.pesan.contains("diharapkan ekspresi"), "{}", e);
    }
}
