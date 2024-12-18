fn main() {
    perkenalan("Dinda", 17);
    perkenalan("Ayu", 34);
    perkenalan("Yusuf", 22);
    perkenalan("Putra", 26);
    perkenalan("Monica", 18);
}

fn perkenalan(nama: &str, umur: u8) {
    /* - tipe &str akan kita bahas di bab setelah ini
    - menggunakan integer u8 karena umur tidak mungkin negatif
      dan tidak mungkin lebih dari 255 */
    println!(
        "Halo, nama aku {}. Umurku {} tahun. Salam kenal!",
        nama, umur
    );
}