<div align="center">

# Latihan Rust

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)  
![Bahasa](https://img.shields.io/badge/Bahasa-Indonesia-blue)
![Conventional Commits](https://img.shields.io/badge/commit-conventional-blue.svg)

</div>

Repo ini berisi latihan Rust dari dasar hingga lanjutan. Setiap folder menyimpan latihan-latihan dengan penjelasan singkat pada masing-masing file untuk membantu pemahaman konsep Rust secara bertahap dan praktikal.

## Struktur Folder

Semua file Rust ditempatkan di dalam direktori berdasarkan tingkatannya, masing-masing membahas topik spesifik. Nama file merepresentasikan konsep yang dibahas. Beberapa materi yang memerlukan penjelasan lebih dalam bisa ditemukan di folder `docs/`.

## Persyaratan

-   Rust versi terbaru (direkomendasikan via `rustup`)
-   Cargo (terinstall otomatis bersama Rust)
-   Sistem operasi yang mendukung Rust (Linux, macOS, atau Windows)

## Cara Menjalankan

1. **Clone repo ini**:

    ```bash
    git clone git@github.com:kaizansultan/latihan-rust.git
    cd latihan-rust
    ```

2. **Jalankan program Rust**:  
   Navigasi ke folder yang diinginkan, lalu jalankan:

    ```bash
    cargo run
    ```

    atau jika hanya ingin compile:

    ```bash
    cargo build
    ```

    Untuk testing:

    ```bash
    cargo test
    ```

## Konvensi Kode

Kami mengikuti konvensi penulisan kode Rust yang idiomatik:

-   Menggunakan **4 spasi** untuk indentasi
-   Maksimal **100 karakter per baris** untuk keterbacaan
-   Menggunakan gaya penulisan **snake_case** untuk variabel dan fungsi
-   Menambahkan **komentar** pada bagian penting dan fungsi
-   Setiap modul/file harus jelas dan tidak ambigu

## Kontribusi

Jika kamu ingin menambahkan latihan atau memperbaiki penjelasan, silakan buat **Pull Request**. Kami akan dengan senang hati melakukan review.

> [!IMPORTANT]
>
> Semua kontributor diharapkan untuk mengikuti [Conventional Commits](https://www.conventionalcommits.org/).

## Lisensi

Repo ini dilisensikan di bawah [MIT License](LICENSE).
