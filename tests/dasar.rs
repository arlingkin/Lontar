use std::process::Command;

#[test]
fn biner_mencetak_nama_dan_versi() {
    let keluaran = Command::new(env!("CARGO_BIN_EXE_lontar"))
        .output()
        .expect("gagal menjalankan biner lontar");
    assert!(keluaran.status.success());
    let teks = String::from_utf8(keluaran.stdout).unwrap();
    assert_eq!(teks.trim(), lontar::salam());
}
