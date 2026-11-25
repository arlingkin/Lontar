//! Kesalahan kompilasi berbahasa Indonesia dengan posisi baris dan kolom.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Kesalahan {
    pub pesan: String,
    pub baris: usize,
    pub kolom: usize,
}

impl Kesalahan {
    pub fn baru(pesan: impl Into<String>, baris: usize, kolom: usize) -> Self {
        Kesalahan {
            pesan: pesan.into(),
            baris,
            kolom,
        }
    }
}

impl fmt::Display for Kesalahan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "kesalahan di baris {}, kolom {}: {}",
            self.baris, self.kolom, self.pesan
        )
    }
}

impl std::error::Error for Kesalahan {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tampilan_memuat_posisi_dan_pesan() {
        let k = Kesalahan::baru("karakter tidak dikenal '@'", 3, 5);
        assert_eq!(
            k.to_string(),
            "kesalahan di baris 3, kolom 5: karakter tidak dikenal '@'"
        );
    }
}
