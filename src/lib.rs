//! Inti kompiler Lontar. Tahap lexer, parser, dst. ditambahkan bertahap.

pub mod kesalahan;
pub mod lexer;
pub mod parser;

pub const NAMA: &str = "Lontar";
pub const VERSI: &str = env!("CARGO_PKG_VERSION");

/// Teks identitas kompiler, mis. `Lontar v0.1.0`.
pub fn salam() -> String {
    format!("{} v{}", NAMA, VERSI)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn salam_memuat_nama_dan_versi() {
        assert_eq!(salam(), format!("Lontar v{}", VERSI));
    }
}
