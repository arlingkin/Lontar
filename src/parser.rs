//! Parser Lontar (M2): ekspresi (literal, nama, operator, kurung, blok,
//! panggilan fungsi, `if`/`else`, `while`, `for x in a..b`, `return`,
//! penugasan `x = e`), pernyataan `let`, dan `fn`. Penugasan berasosiasi kanan
//! dan terendah; rentang `a..b`/`a..=b` (tanpa rantai) tepat di atasnya.
//! Presedensi dari terendah: `||`, `&&`, perbandingan, `+ -`, `* / %`,
//! unary (`-`, `!`), panggilan, primer. `(` hanya memulai panggilan jika satu
//! baris dengan token sebelumnya.
//! Titik koma opsional: pernyataan berakhir di `;`, di `}`, atau saat token
//! berikutnya ada di baris lebih baru. Ekspresi terakhir blok tanpa `;` = nilai.

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
    Dan,
    Atau,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ekspr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Nama(String),
    Negasi(Box<Ekspr>),
    Tidak(Box<Ekspr>),
    Panggil {
        fungsi: Box<Ekspr>,
        arg: Vec<Ekspr>,
    },
    Biner {
        op: Op,
        kiri: Box<Ekspr>,
        kanan: Box<Ekspr>,
    },
    Blok(Blok),
    /// `if syarat { .. } [else { .. } | else if ..]`; `lainnya` = `Blok` atau `Jika`.
    Jika {
        syarat: Box<Ekspr>,
        maka: Blok,
        lainnya: Option<Box<Ekspr>>,
    },
    Selama {
        syarat: Box<Ekspr>,
        badan: Blok,
    },
    /// `for nama in iter { .. }`; `iter` biasanya `Rentang`.
    Untuk {
        nama: String,
        iter: Box<Ekspr>,
        badan: Blok,
    },
    /// `awal..akhir` (`termasuk` = false) atau `awal..=akhir` (true); kedua ujung wajib.
    Rentang {
        awal: Box<Ekspr>,
        akhir: Box<Ekspr>,
        termasuk: bool,
    },
    Balik(Option<Box<Ekspr>>),
    /// `sasaran = nilai`; saat ini sasaran selalu `Ekspr::Nama`.
    Tugas {
        sasaran: Box<Ekspr>,
        nilai: Box<Ekspr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Blok {
    pub pernyataan: Vec<Pernyataan>,
    /// Ekspresi terakhir tanpa `;` (nilai balik blok), jika ada.
    pub nilai: Option<Box<Ekspr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pernyataan {
    Let {
        nama: String,
        ubah: bool,
        tipe: Option<String>,
        nilai: Ekspr,
    },
    Ekspresi(Ekspr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub nama: String,
    pub tipe: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fungsi {
    pub nama: String,
    pub param: Vec<Param>,
    pub balik: Option<String>,
    pub badan: Blok,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub fungsi: Vec<Fungsi>,
}

/// Mengurai seluruh berkas: saat ini hanya deretan `fn` tingkat atas.
pub fn parse_program(token: &[Token]) -> Result<Program, Kesalahan> {
    if token.is_empty() {
        return Err(Kesalahan::baru(
            "diharapkan 'fn', ditemukan akhir berkas",
            1,
            1,
        ));
    }
    let mut p = Parser { token, pos: 0 };
    let mut fungsi = Vec::new();
    while p.lihat().jenis != Jenis::Eof {
        fungsi.push(p.fungsi()?);
    }
    Ok(Program { fungsi })
}

enum Akhir {
    TitikKoma,
    Tutup,
    BarisBaru,
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
    let e = p.ekspr()?;
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

    fn sebelumnya(&self) -> &'a Token {
        let toks: &'a [Token] = self.token;
        &toks[self.pos.saturating_sub(1)]
    }

    fn harap(&mut self, j: Jenis, apa: &str) -> Result<(), Kesalahan> {
        let t = self.lihat();
        if t.jenis != j {
            return Err(harapan(apa, t));
        }
        self.maju();
        Ok(())
    }

    fn ident(&mut self, apa: &str) -> Result<String, Kesalahan> {
        let t = self.lihat();
        match &t.jenis {
            Jenis::Ident(s) => {
                let s = s.clone();
                self.maju();
                Ok(s)
            }
            _ => Err(harapan(apa, t)),
        }
    }

    /// Penutup pernyataan: `;` (dikonsumsi), `}` (tidak dikonsumsi), atau baris baru.
    fn akhir(&mut self) -> Result<Akhir, Kesalahan> {
        let t = self.lihat();
        match t.jenis {
            Jenis::Semi => {
                self.maju();
                Ok(Akhir::TitikKoma)
            }
            Jenis::RBrace => Ok(Akhir::Tutup),
            Jenis::Eof => Ok(Akhir::BarisBaru),
            _ if t.baris > self.sebelumnya().baris => Ok(Akhir::BarisBaru),
            _ => Err(harapan("';' atau baris baru", t)),
        }
    }

    fn fungsi(&mut self) -> Result<Fungsi, Kesalahan> {
        self.harap(Jenis::Fn, "'fn'")?;
        let nama = self.ident("nama fungsi")?;
        self.harap(Jenis::LParen, "'('")?;
        let mut param = Vec::new();
        while self.lihat().jenis != Jenis::RParen {
            let n = self.ident("nama parameter")?;
            self.harap(Jenis::Colon, "':'")?;
            let tipe = self.ident("nama tipe")?;
            param.push(Param { nama: n, tipe });
            if self.lihat().jenis == Jenis::Comma {
                self.maju();
            } else {
                break;
            }
        }
        self.harap(Jenis::RParen, "')'")?;
        let balik = if self.lihat().jenis == Jenis::Arrow {
            self.maju();
            Some(self.ident("nama tipe")?)
        } else {
            None
        };
        let badan = self.blok()?;
        Ok(Fungsi {
            nama,
            param,
            balik,
            badan,
        })
    }

    fn pernyataan_let(&mut self) -> Result<Pernyataan, Kesalahan> {
        self.maju(); // `let`
        let ubah = self.lihat().jenis == Jenis::Mut;
        if ubah {
            self.maju();
        }
        let nama = self.ident("nama variabel")?;
        let tipe = if self.lihat().jenis == Jenis::Colon {
            self.maju();
            Some(self.ident("nama tipe")?)
        } else {
            None
        };
        self.harap(Jenis::Assign, "'='")?;
        let nilai = self.ekspr()?;
        Ok(Pernyataan::Let {
            nama,
            ubah,
            tipe,
            nilai,
        })
    }

    fn blok(&mut self) -> Result<Blok, Kesalahan> {
        self.harap(Jenis::LBrace, "'{'")?;
        let mut pernyataan = Vec::new();
        let mut nilai = None;
        loop {
            let t = self.lihat();
            match t.jenis {
                Jenis::RBrace => {
                    self.maju();
                    break;
                }
                Jenis::Eof => {
                    return Err(Kesalahan::baru(
                        "blok tidak ditutup: diharapkan '}', ditemukan akhir berkas",
                        t.baris,
                        t.kolom,
                    ))
                }
                Jenis::Semi => self.maju(),
                Jenis::Let => {
                    let p = self.pernyataan_let()?;
                    self.akhir()?;
                    pernyataan.push(p);
                }
                _ => {
                    let e = self.ekspr()?;
                    match self.akhir()? {
                        Akhir::Tutup => nilai = Some(Box::new(e)),
                        _ => pernyataan.push(Pernyataan::Ekspresi(e)),
                    }
                }
            }
        }
        Ok(Blok { pernyataan, nilai })
    }

    fn ekspr(&mut self) -> Result<Ekspr, Kesalahan> {
        if self.lihat().jenis == Jenis::Return {
            return self.balik();
        }
        let awal = self.lihat();
        let kiri = self.rentang()?;
        if self.lihat().jenis != Jenis::Assign {
            return Ok(kiri);
        }
        if !matches!(kiri, Ekspr::Nama(_)) {
            return Err(Kesalahan::baru(
                "sasaran penugasan harus berupa nama",
                awal.baris,
                awal.kolom,
            ));
        }
        self.maju();
        let nilai = self.ekspr()?;
        Ok(Ekspr::Tugas {
            sasaran: Box::new(kiri),
            nilai: Box::new(nilai),
        })
    }

    /// `a..b` atau `a..=b`; tidak bisa dirantai (`a..b..c` ditolak oleh pemanggil).
    fn rentang(&mut self) -> Result<Ekspr, Kesalahan> {
        let awal = self.atau()?;
        let termasuk = match self.lihat().jenis {
            Jenis::DotDot => false,
            Jenis::DotDotEq => true,
            _ => return Ok(awal),
        };
        self.maju();
        let akhir = self.atau()?;
        Ok(Ekspr::Rentang {
            awal: Box::new(awal),
            akhir: Box::new(akhir),
            termasuk,
        })
    }

    fn untuk(&mut self) -> Result<Ekspr, Kesalahan> {
        self.maju(); // `for`
        let nama = self.ident("nama variabel")?;
        self.harap(Jenis::In, "'in'")?;
        let iter = Box::new(self.ekspr()?);
        let badan = self.blok()?;
        Ok(Ekspr::Untuk { nama, iter, badan })
    }

    /// `return [e]`: nilai dilewati jika token berikutnya `;`, `}`, `)`, `,`,
    /// akhir berkas, atau ada di baris lain.
    fn balik(&mut self) -> Result<Ekspr, Kesalahan> {
        let kata = self.lihat();
        self.maju();
        let t = self.lihat();
        let ada = t.baris == kata.baris
            && !matches!(
                t.jenis,
                Jenis::Semi | Jenis::RBrace | Jenis::RParen | Jenis::Comma | Jenis::Eof
            );
        let nilai = if ada {
            Some(Box::new(self.ekspr()?))
        } else {
            None
        };
        Ok(Ekspr::Balik(nilai))
    }

    fn jika(&mut self) -> Result<Ekspr, Kesalahan> {
        self.maju(); // `if`
        let syarat = Box::new(self.ekspr()?);
        let maka = self.blok()?;
        let lainnya = if self.lihat().jenis == Jenis::Else {
            self.maju();
            if self.lihat().jenis == Jenis::If {
                Some(Box::new(self.jika()?))
            } else {
                Some(Box::new(Ekspr::Blok(self.blok()?)))
            }
        } else {
            None
        };
        Ok(Ekspr::Jika {
            syarat,
            maka,
            lainnya,
        })
    }

    fn selama(&mut self) -> Result<Ekspr, Kesalahan> {
        self.maju(); // `while`
        let syarat = Box::new(self.ekspr()?);
        let badan = self.blok()?;
        Ok(Ekspr::Selama { syarat, badan })
    }

    fn atau(&mut self) -> Result<Ekspr, Kesalahan> {
        let mut kiri = self.dan()?;
        while self.lihat().jenis == Jenis::OrOr {
            self.maju();
            let kanan = self.dan()?;
            kiri = biner(Op::Atau, kiri, kanan);
        }
        Ok(kiri)
    }

    fn dan(&mut self) -> Result<Ekspr, Kesalahan> {
        let mut kiri = self.banding()?;
        while self.lihat().jenis == Jenis::AndAnd {
            self.maju();
            let kanan = self.banding()?;
            kiri = biner(Op::Dan, kiri, kanan);
        }
        Ok(kiri)
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
        if self.lihat().jenis == Jenis::Bang {
            self.maju();
            let e = self.unary()?;
            return Ok(Ekspr::Tidak(Box::new(e)));
        }
        self.panggilan()
    }

    fn panggilan(&mut self) -> Result<Ekspr, Kesalahan> {
        let mut e = self.primer()?;
        while self.lihat().jenis == Jenis::LParen && self.lihat().baris == self.sebelumnya().baris {
            self.maju();
            let mut arg = Vec::new();
            while self.lihat().jenis != Jenis::RParen {
                arg.push(self.ekspr()?);
                if self.lihat().jenis == Jenis::Comma {
                    self.maju();
                } else {
                    break;
                }
            }
            self.harap(Jenis::RParen, "')'")?;
            e = Ekspr::Panggil {
                fungsi: Box::new(e),
                arg,
            };
        }
        Ok(e)
    }

    fn primer(&mut self) -> Result<Ekspr, Kesalahan> {
        let t = self.lihat();
        let e = match &t.jenis {
            Jenis::Int(n) => Ekspr::Int(*n),
            Jenis::Float(f) => Ekspr::Float(*f),
            Jenis::True => Ekspr::Bool(true),
            Jenis::False => Ekspr::Bool(false),
            Jenis::Ident(s) => Ekspr::Nama(s.clone()),
            Jenis::LBrace => {
                let b = self.blok()?;
                return Ok(Ekspr::Blok(b));
            }
            Jenis::If => return self.jika(),
            Jenis::While => return self.selama(),
            Jenis::For => return self.untuk(),
            Jenis::LParen => {
                self.maju();
                let e = self.ekspr()?;
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

fn harapan(apa: &str, t: &Token) -> Kesalahan {
    Kesalahan::baru(
        format!("diharapkan {}, ditemukan {}", apa, deskripsi(&t.jenis)),
        t.baris,
        t.kolom,
    )
}

fn deskripsi(j: &Jenis) -> String {
    match j {
        Jenis::AndAnd => "'&&'".to_string(),
        Jenis::OrOr => "'||'".to_string(),
        Jenis::Bang => "'!'".to_string(),
        Jenis::Semi => "';'".to_string(),
        Jenis::LBrace => "'{'".to_string(),
        Jenis::RBrace => "'}'".to_string(),
        Jenis::LParen => "'('".to_string(),
        Jenis::RParen => "')'".to_string(),
        Jenis::Colon => "':'".to_string(),
        Jenis::Comma => "','".to_string(),
        Jenis::Assign => "'='".to_string(),
        Jenis::Arrow => "'->'".to_string(),
        Jenis::Fn => "'fn'".to_string(),
        Jenis::Let => "'let'".to_string(),
        Jenis::If => "'if'".to_string(),
        Jenis::Else => "'else'".to_string(),
        Jenis::While => "'while'".to_string(),
        Jenis::Return => "'return'".to_string(),
        Jenis::For => "'for'".to_string(),
        Jenis::In => "'in'".to_string(),
        Jenis::DotDot => "'..'".to_string(),
        Jenis::DotDotEq => "'..='".to_string(),
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

    fn prog(s: &str) -> Result<Program, Kesalahan> {
        let t = tokenisasi(s)?;
        parse_program(&t)
    }

    /// Mengurai `isi` sebagai badan `fn f() { isi }` lalu mengembalikan bloknya.
    fn badan(isi: &str) -> Result<Blok, Kesalahan> {
        let mut p = prog(&format!("fn f() {{ {} }}", isi))?;
        Ok(p.fungsi.remove(0).badan)
    }

    fn s(x: &str) -> String {
        x.to_string()
    }

    #[test]
    fn let_sederhana() {
        let b = badan("let x = 1 + 2").unwrap();
        assert_eq!(
            b.pernyataan,
            vec![Pernyataan::Let {
                nama: s("x"),
                ubah: false,
                tipe: None,
                nilai: biner(Op::Tambah, int(1), int(2)),
            }]
        );
        assert_eq!(b.nilai, None);
    }

    #[test]
    fn let_mut_dengan_tipe() {
        let b = badan("let mut n: i32 = 5;").unwrap();
        assert_eq!(
            b.pernyataan,
            vec![Pernyataan::Let {
                nama: s("n"),
                ubah: true,
                tipe: Some(s("i32")),
                nilai: int(5),
            }]
        );
    }

    #[test]
    fn titik_koma_opsional_antar_baris() {
        let b = badan("let a = 1\nlet b = a\nb").unwrap();
        assert_eq!(b.pernyataan.len(), 2);
        assert_eq!(b.nilai, Some(Box::new(nama("b"))));
    }

    #[test]
    fn ekspresi_terakhir_jadi_nilai_blok() {
        let b = badan("1; 2").unwrap();
        assert_eq!(b.pernyataan, vec![Pernyataan::Ekspresi(int(1))]);
        assert_eq!(b.nilai, Some(Box::new(int(2))));
    }

    #[test]
    fn titik_koma_di_akhir_membuang_nilai() {
        let b = badan("a;").unwrap();
        assert_eq!(b.pernyataan, vec![Pernyataan::Ekspresi(nama("a"))]);
        assert_eq!(b.nilai, None);
    }

    #[test]
    fn dua_pernyataan_satu_baris_tanpa_pemisah_ditolak() {
        let e = badan("let a = 1 let b = 2").unwrap_err();
        assert!(e.pesan.contains("';' atau baris baru"), "{}", e);
        assert_eq!((e.baris, e.kolom), (1, 20));
    }

    #[test]
    fn blok_sebagai_ekspresi() {
        assert_eq!(
            parse("{ 1 }").unwrap(),
            Ekspr::Blok(Blok {
                pernyataan: vec![],
                nilai: Some(Box::new(int(1))),
            })
        );
    }

    #[test]
    fn fn_lengkap() {
        let p = prog("fn tambah(a: i32, b: i32) -> i32 { a + b }").unwrap();
        assert_eq!(
            p.fungsi,
            vec![Fungsi {
                nama: s("tambah"),
                param: vec![
                    Param {
                        nama: s("a"),
                        tipe: s("i32")
                    },
                    Param {
                        nama: s("b"),
                        tipe: s("i32")
                    },
                ],
                balik: Some(s("i32")),
                badan: Blok {
                    pernyataan: vec![],
                    nilai: Some(Box::new(biner(Op::Tambah, nama("a"), nama("b")))),
                },
            }]
        );
    }

    #[test]
    fn fn_tanpa_tipe_balik_dan_koma_akhir() {
        let p = prog("fn f(a: i32,) {}").unwrap();
        assert_eq!(p.fungsi[0].param.len(), 1);
        assert_eq!(p.fungsi[0].balik, None);
    }

    #[test]
    fn beberapa_fungsi_berurutan() {
        let p = prog("fn a() {}\nfn b() {}").unwrap();
        assert_eq!(p.fungsi.len(), 2);
        assert_eq!(p.fungsi[1].nama, "b");
    }

    #[test]
    fn blok_tidak_ditutup_ditolak() {
        let e = prog("fn f() { 1").unwrap_err();
        assert!(e.pesan.contains("blok tidak ditutup"), "{}", e);
    }

    #[test]
    fn tingkat_atas_selain_fn_ditolak() {
        let e = prog("let x = 1").unwrap_err();
        assert!(e.pesan.contains("diharapkan 'fn'"), "{}", e);
        assert_eq!((e.baris, e.kolom), (1, 1));
    }

    #[test]
    fn let_tanpa_nilai_ditolak() {
        let e = badan("let x").unwrap_err();
        assert!(e.pesan.contains("diharapkan '='"), "{}", e);
    }

    #[test]
    fn param_tanpa_tipe_ditolak() {
        let e = prog("fn f(a) {}").unwrap_err();
        assert!(e.pesan.contains("diharapkan ':'"), "{}", e);
    }

    fn tidak(e: Ekspr) -> Ekspr {
        Ekspr::Tidak(Box::new(e))
    }

    fn panggil(f: Ekspr, arg: Vec<Ekspr>) -> Ekspr {
        Ekspr::Panggil {
            fungsi: Box::new(f),
            arg,
        }
    }

    #[test]
    fn dan_lebih_kuat_dari_atau() {
        assert_eq!(
            parse("a || b && c").unwrap(),
            biner(Op::Atau, nama("a"), biner(Op::Dan, nama("b"), nama("c")))
        );
    }

    #[test]
    fn perbandingan_lebih_kuat_dari_dan() {
        assert_eq!(
            parse("a < 1 && b").unwrap(),
            biner(Op::Dan, biner(Op::KurangDari, nama("a"), int(1)), nama("b"))
        );
    }

    #[test]
    fn tidak_unary_dan_bersarang() {
        assert_eq!(
            parse("!a && b").unwrap(),
            biner(Op::Dan, tidak(nama("a")), nama("b"))
        );
        assert_eq!(parse("!!x").unwrap(), tidak(tidak(nama("x"))));
    }

    #[test]
    fn panggilan_tanpa_dan_dengan_argumen() {
        assert_eq!(parse("f()").unwrap(), panggil(nama("f"), vec![]));
        assert_eq!(
            parse("f(1, a + 2,)").unwrap(),
            panggil(
                nama("f"),
                vec![int(1), biner(Op::Tambah, nama("a"), int(2))]
            )
        );
    }

    #[test]
    fn panggilan_bersarang_dan_berantai() {
        assert_eq!(
            parse("f(g(1))").unwrap(),
            panggil(nama("f"), vec![panggil(nama("g"), vec![int(1)])])
        );
        assert_eq!(
            parse("f(1)(2)").unwrap(),
            panggil(panggil(nama("f"), vec![int(1)]), vec![int(2)])
        );
    }

    #[test]
    fn panggilan_lebih_kuat_dari_negasi() {
        assert_eq!(
            parse("-f(1)").unwrap(),
            Ekspr::Negasi(Box::new(panggil(nama("f"), vec![int(1)])))
        );
    }

    #[test]
    fn kurung_di_baris_baru_bukan_panggilan() {
        let b = badan("a\n(b)").unwrap();
        assert_eq!(b.pernyataan, vec![Pernyataan::Ekspresi(nama("a"))]);
        assert_eq!(b.nilai, Some(Box::new(nama("b"))));
    }

    #[test]
    fn argumen_tidak_tertutup_ditolak() {
        let e = parse("f(1").unwrap_err();
        assert!(e.pesan.contains("diharapkan ')'"), "{}", e);
        assert_eq!((e.baris, e.kolom), (1, 4));
    }

    fn blok_nilai(e: Ekspr) -> Blok {
        Blok {
            pernyataan: vec![],
            nilai: Some(Box::new(e)),
        }
    }

    fn kosong() -> Blok {
        Blok {
            pernyataan: vec![],
            nilai: None,
        }
    }

    #[test]
    fn jika_tanpa_else() {
        assert_eq!(
            parse("if a { 1 }").unwrap(),
            Ekspr::Jika {
                syarat: Box::new(nama("a")),
                maka: blok_nilai(int(1)),
                lainnya: None,
            }
        );
    }

    #[test]
    fn jika_dengan_else_dan_syarat_majemuk() {
        assert_eq!(
            parse("if a < 1 && b { 1 } else { 2 }").unwrap(),
            Ekspr::Jika {
                syarat: Box::new(biner(
                    Op::Dan,
                    biner(Op::KurangDari, nama("a"), int(1)),
                    nama("b")
                )),
                maka: blok_nilai(int(1)),
                lainnya: Some(Box::new(Ekspr::Blok(blok_nilai(int(2))))),
            }
        );
    }

    #[test]
    fn else_if_berantai() {
        let e = parse("if a { 1 } else if b { 2 } else { 3 }").unwrap();
        let Ekspr::Jika { lainnya, .. } = e else {
            panic!("diharapkan Jika");
        };
        assert_eq!(
            *lainnya.unwrap(),
            Ekspr::Jika {
                syarat: Box::new(nama("b")),
                maka: blok_nilai(int(2)),
                lainnya: Some(Box::new(Ekspr::Blok(blok_nilai(int(3))))),
            }
        );
    }

    #[test]
    fn else_di_baris_baru_tetap_menyambung() {
        let e = parse("if a { 1 }\nelse { 2 }").unwrap();
        assert!(matches!(
            e,
            Ekspr::Jika {
                lainnya: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn jika_sebagai_nilai_let_dan_blok() {
        let b = badan("let x = if a { 1 } else { 2 }\nx").unwrap();
        assert!(matches!(
            &b.pernyataan[0],
            Pernyataan::Let {
                nilai: Ekspr::Jika { .. },
                ..
            }
        ));
        let b = badan("if a { 1 } else { 2 }").unwrap();
        assert!(matches!(b.nilai.as_deref(), Some(Ekspr::Jika { .. })));
    }

    #[test]
    fn jika_pernyataan_tanpa_titik_koma_lalu_baris_baru() {
        let b = badan("if a { f() }\nlet y = 1\ny").unwrap();
        assert_eq!(b.pernyataan.len(), 2);
        assert_eq!(b.nilai.as_deref(), Some(&nama("y")));
    }

    #[test]
    fn selama_dengan_badan() {
        assert_eq!(
            parse("while i < 3 { i = i + 1 }").unwrap(),
            Ekspr::Selama {
                syarat: Box::new(biner(Op::KurangDari, nama("i"), int(3))),
                badan: blok_nilai(Ekspr::Tugas {
                    sasaran: Box::new(nama("i")),
                    nilai: Box::new(biner(Op::Tambah, nama("i"), int(1))),
                }),
            }
        );
    }

    #[test]
    fn balik_dengan_dan_tanpa_nilai() {
        assert_eq!(
            parse("return a + 1").unwrap(),
            Ekspr::Balik(Some(Box::new(biner(Op::Tambah, nama("a"), int(1)))))
        );
        assert_eq!(parse("return").unwrap(), Ekspr::Balik(None));
        let b = badan("return;\nreturn\n1").unwrap();
        assert_eq!(b.pernyataan.len(), 2);
        assert_eq!(b.pernyataan[0], Pernyataan::Ekspresi(Ekspr::Balik(None)));
        assert_eq!(b.pernyataan[1], Pernyataan::Ekspresi(Ekspr::Balik(None)));
        assert_eq!(b.nilai.as_deref(), Some(&int(1)));
    }

    #[test]
    fn balik_di_dalam_jika_menjadi_nilai_blok() {
        let b = badan("if a { return 1 }").unwrap();
        let Some(Ekspr::Jika { maka, .. }) = b.nilai.as_deref() else {
            panic!("diharapkan Jika");
        };
        assert_eq!(
            maka.nilai.as_deref(),
            Some(&Ekspr::Balik(Some(Box::new(int(1)))))
        );
    }

    #[test]
    fn penugasan_asosiatif_kanan_dan_terendah() {
        assert_eq!(
            parse("a = b = 1 || c").unwrap(),
            Ekspr::Tugas {
                sasaran: Box::new(nama("a")),
                nilai: Box::new(Ekspr::Tugas {
                    sasaran: Box::new(nama("b")),
                    nilai: Box::new(biner(Op::Atau, int(1), nama("c"))),
                }),
            }
        );
    }

    #[test]
    fn penugasan_sebagai_pernyataan() {
        let b = badan("let mut x = 0\nx = x + 1\nx").unwrap();
        assert!(matches!(
            &b.pernyataan[1],
            Pernyataan::Ekspresi(Ekspr::Tugas { .. })
        ));
    }

    #[test]
    fn sasaran_penugasan_bukan_nama_ditolak() {
        let e = parse("1 + 2 = 3").unwrap_err();
        assert_eq!(e.pesan, "sasaran penugasan harus berupa nama");
        assert_eq!((e.baris, e.kolom), (1, 1));
    }

    #[test]
    fn jika_tanpa_kurung_kurawal_ditolak() {
        let e = parse("if a 1").unwrap_err();
        assert_eq!(e.pesan, "diharapkan '{', ditemukan angka '1'");
    }

    #[test]
    fn else_tanpa_if_ditolak() {
        let e = parse("else { 1 }").unwrap_err();
        assert_eq!(e.pesan, "diharapkan ekspresi, ditemukan 'else'");
    }

    #[test]
    fn selama_tanpa_badan_ditolak() {
        let e = parse("while a").unwrap_err();
        assert_eq!(e.pesan, "diharapkan '{', ditemukan akhir berkas");
    }

    #[test]
    fn blok_kosong_di_jika() {
        assert!(matches!(
            parse("if a {}").unwrap(),
            Ekspr::Jika { maka, .. } if maka == kosong()
        ));
    }

    fn rentang(a: Ekspr, b: Ekspr, termasuk: bool) -> Ekspr {
        Ekspr::Rentang {
            awal: Box::new(a),
            akhir: Box::new(b),
            termasuk,
        }
    }

    #[test]
    fn untuk_rentang_eksklusif_dan_inklusif() {
        assert_eq!(
            parse("for i in 0..n { f(i) }").unwrap(),
            Ekspr::Untuk {
                nama: "i".to_string(),
                iter: Box::new(rentang(int(0), nama("n"), false)),
                badan: blok_nilai(panggil(nama("f"), vec![nama("i")])),
            }
        );
        let Ekspr::Untuk { iter, .. } = parse("for i in 1..=5 {}").unwrap() else {
            panic!("diharapkan Untuk");
        };
        assert_eq!(*iter, rentang(int(1), int(5), true));
    }

    #[test]
    fn rentang_lebih_rendah_dari_aritmetika_dan_logika() {
        assert_eq!(
            parse("a + 1..b * 2").unwrap(),
            rentang(
                biner(Op::Tambah, nama("a"), int(1)),
                biner(Op::Kali, nama("b"), int(2)),
                false
            )
        );
        assert_eq!(
            parse("a || b..c").unwrap(),
            rentang(biner(Op::Atau, nama("a"), nama("b")), nama("c"), false)
        );
    }

    #[test]
    fn untuk_bersarang_dan_pernyataan_tanpa_titik_koma() {
        let b = badan("for i in 0..3 { if i == 1 { return } }\nlet y = 1\ny").unwrap();
        assert_eq!(b.pernyataan.len(), 2);
        assert!(matches!(
            &b.pernyataan[0],
            Pernyataan::Ekspresi(Ekspr::Untuk { .. })
        ));
        assert_eq!(b.nilai.as_deref(), Some(&nama("y")));
    }

    #[test]
    fn rentang_tanpa_ujung_ditolak() {
        let e = parse("1..").unwrap_err();
        assert_eq!(e.pesan, "diharapkan ekspresi, ditemukan akhir berkas");
        let e = parse("..3").unwrap_err();
        assert_eq!(e.pesan, "diharapkan ekspresi, ditemukan '..'");
    }

    #[test]
    fn rentang_tidak_bisa_dirantai() {
        let e = parse("1..2..3").unwrap_err();
        assert_eq!(e.pesan, "token tidak terduga setelah ekspresi: '..'");
    }

    #[test]
    fn untuk_tanpa_in_atau_tanpa_nama_ditolak() {
        let e = parse("for x 0..3 {}").unwrap_err();
        assert_eq!(e.pesan, "diharapkan 'in', ditemukan angka '0'");
        let e = parse("for 1 in a {}").unwrap_err();
        assert_eq!(e.pesan, "diharapkan nama variabel, ditemukan angka '1'");
    }

    #[test]
    fn untuk_tanpa_badan_ditolak() {
        let e = parse("for i in 0..3").unwrap_err();
        assert_eq!(e.pesan, "diharapkan '{', ditemukan akhir berkas");
    }

    #[test]
    fn rentang_sebagai_sasaran_penugasan_ditolak() {
        let e = parse("a..b = 1").unwrap_err();
        assert_eq!(e.pesan, "sasaran penugasan harus berupa nama");
    }
}
