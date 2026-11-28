//! Tipe dasar Lontar (M3, langkah 1): cakupan tipe awal dan aturan operator.
//! Fungsi di sini murni dan tanpa posisi: pesan kesalahan berupa `String`;
//! pemanggil (pemeriksa tipe) menambahkan baris dan kolom.

use crate::parser::Op;
use std::fmt;

/// Tipe awal. Semua tipe bilangan bulat (`i32`, `u8`, ...) dipetakan ke `Int`
/// dan `f32`/`f64` ke `Float`; pembedaan ukuran menyusul.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tipe {
    Int,
    Float,
    Bool,
    Unit,
}

impl Tipe {
    /// Tipe dari nama pada anotasi (`i32`, `f64`, `bool`, ...); `None` jika belum dikenal.
    pub fn dari_nama(nama: &str) -> Option<Tipe> {
        match nama {
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64"
            | "u128" | "usize" => Some(Tipe::Int),
            "f32" | "f64" => Some(Tipe::Float),
            "bool" => Some(Tipe::Bool),
            _ => None,
        }
    }
}

impl fmt::Display for Tipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Tipe::Int => "Int",
            Tipe::Float => "Float",
            Tipe::Bool => "Bool",
            Tipe::Unit => "()",
        })
    }
}

fn simbol(op: Op) -> &'static str {
    match op {
        Op::Tambah => "+",
        Op::Kurang => "-",
        Op::Kali => "*",
        Op::Bagi => "/",
        Op::Sisa => "%",
        Op::Sama => "==",
        Op::BedaDari => "!=",
        Op::KurangDari => "<",
        Op::KurangSama => "<=",
        Op::LebihDari => ">",
        Op::LebihSama => ">=",
        Op::Dan => "&&",
        Op::Atau => "||",
    }
}

fn tak_berlaku(op: Op, kiri: Tipe, kanan: Tipe) -> String {
    format!(
        "operator '{}' tidak dapat dipakai pada {} dan {}",
        simbol(op),
        kiri,
        kanan
    )
}

fn tak_sama(op: Op, kiri: Tipe, kanan: Tipe) -> String {
    format!(
        "tipe kedua sisi '{}' tidak sama: {} dan {}",
        simbol(op),
        kiri,
        kanan
    )
}

/// Tipe hasil `kiri op kanan`.
/// Aritmetika: `Int`/`Float`, kedua sisi sama. Urutan (`< <= > >=`): `Int`/`Float` sama,
/// hasil `Bool`. `==`/`!=`: kedua sisi tipe sama, hasil `Bool`. `&&`/`||`: `Bool`.
pub fn tipe_biner(op: Op, kiri: Tipe, kanan: Tipe) -> Result<Tipe, String> {
    match op {
        Op::Dan | Op::Atau => {
            if kiri == Tipe::Bool && kanan == Tipe::Bool {
                Ok(Tipe::Bool)
            } else {
                Err(tak_berlaku(op, kiri, kanan))
            }
        }
        Op::Sama | Op::BedaDari => {
            if kiri == kanan {
                Ok(Tipe::Bool)
            } else {
                Err(tak_sama(op, kiri, kanan))
            }
        }
        _ => {
            if kiri != kanan {
                return Err(tak_sama(op, kiri, kanan));
            }
            if !matches!(kiri, Tipe::Int | Tipe::Float) {
                return Err(tak_berlaku(op, kiri, kanan));
            }
            let urutan = matches!(
                op,
                Op::KurangDari | Op::KurangSama | Op::LebihDari | Op::LebihSama
            );
            Ok(if urutan { Tipe::Bool } else { kiri })
        }
    }
}

/// Tipe hasil negasi `-e`: hanya `Int` atau `Float`.
pub fn tipe_negasi(t: Tipe) -> Result<Tipe, String> {
    match t {
        Tipe::Int | Tipe::Float => Ok(t),
        _ => Err(format!("negasi '-' tidak dapat dipakai pada {}", t)),
    }
}

/// Tipe hasil `!e`: hanya `Bool`.
pub fn tipe_tidak(t: Tipe) -> Result<Tipe, String> {
    match t {
        Tipe::Bool => Ok(Tipe::Bool),
        _ => Err(format!("'!' tidak dapat dipakai pada {}", t)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nama_tipe_dipetakan() {
        assert_eq!(Tipe::dari_nama("i32"), Some(Tipe::Int));
        assert_eq!(Tipe::dari_nama("u8"), Some(Tipe::Int));
        assert_eq!(Tipe::dari_nama("f64"), Some(Tipe::Float));
        assert_eq!(Tipe::dari_nama("bool"), Some(Tipe::Bool));
        assert_eq!(Tipe::dari_nama("Warga"), None);
    }

    #[test]
    fn aritmetika_menjaga_tipe() {
        assert_eq!(tipe_biner(Op::Tambah, Tipe::Int, Tipe::Int), Ok(Tipe::Int));
        assert_eq!(
            tipe_biner(Op::Bagi, Tipe::Float, Tipe::Float),
            Ok(Tipe::Float)
        );
        assert_eq!(tipe_biner(Op::Sisa, Tipe::Int, Tipe::Int), Ok(Tipe::Int));
    }

    #[test]
    fn aritmetika_tipe_campur_ditolak() {
        let e = tipe_biner(Op::Tambah, Tipe::Int, Tipe::Float).unwrap_err();
        assert_eq!(e, "tipe kedua sisi '+' tidak sama: Int dan Float");
    }

    #[test]
    fn aritmetika_pada_bool_ditolak() {
        let e = tipe_biner(Op::Kali, Tipe::Bool, Tipe::Bool).unwrap_err();
        assert_eq!(e, "operator '*' tidak dapat dipakai pada Bool dan Bool");
    }

    #[test]
    fn urutan_menghasilkan_bool() {
        assert_eq!(
            tipe_biner(Op::KurangDari, Tipe::Int, Tipe::Int),
            Ok(Tipe::Bool)
        );
        assert_eq!(
            tipe_biner(Op::LebihSama, Tipe::Float, Tipe::Float),
            Ok(Tipe::Bool)
        );
        assert!(tipe_biner(Op::KurangDari, Tipe::Bool, Tipe::Bool).is_err());
    }

    #[test]
    fn kesamaan_butuh_tipe_sama() {
        assert_eq!(tipe_biner(Op::Sama, Tipe::Bool, Tipe::Bool), Ok(Tipe::Bool));
        assert_eq!(
            tipe_biner(Op::BedaDari, Tipe::Int, Tipe::Int),
            Ok(Tipe::Bool)
        );
        let e = tipe_biner(Op::Sama, Tipe::Int, Tipe::Bool).unwrap_err();
        assert_eq!(e, "tipe kedua sisi '==' tidak sama: Int dan Bool");
    }

    #[test]
    fn logika_hanya_bool() {
        assert_eq!(tipe_biner(Op::Dan, Tipe::Bool, Tipe::Bool), Ok(Tipe::Bool));
        let e = tipe_biner(Op::Atau, Tipe::Int, Tipe::Bool).unwrap_err();
        assert_eq!(e, "operator '||' tidak dapat dipakai pada Int dan Bool");
    }

    #[test]
    fn negasi_dan_tidak() {
        assert_eq!(tipe_negasi(Tipe::Int), Ok(Tipe::Int));
        assert_eq!(tipe_negasi(Tipe::Float), Ok(Tipe::Float));
        assert_eq!(
            tipe_negasi(Tipe::Bool).unwrap_err(),
            "negasi '-' tidak dapat dipakai pada Bool"
        );
        assert_eq!(tipe_tidak(Tipe::Bool), Ok(Tipe::Bool));
        assert_eq!(
            tipe_tidak(Tipe::Int).unwrap_err(),
            "'!' tidak dapat dipakai pada Int"
        );
    }
}
