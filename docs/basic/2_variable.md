# Variabel

## Pengertian

Sebuah variabel adalah penyimpanan bernama yang dapat dimanipulasi oleh program. Secara sederhana, variabel membantu program untuk menyimpan nilai. Variabel di Rust diasosiasikan dengan tipe data tertentu. Tipe data menentukan ukuran dan tata letak memori variabel, rentang nilai yang dapat disimpan dalam memori tersebut, serta serangkaian operasi yang dapat dilakukan pada variabel.

## Aturan untuk Menamai Variabel

Dalam bagian ini, kita akan mempelajari berbagai aturan untuk menamai variabel.

- Nama variabel dapat terdiri dari huruf, angka, dan karakter garis bawah (\_).
- Nama variabel harus diawali dengan huruf atau garis bawah.
- Huruf kapital dan huruf kecil dianggap berbeda karena Rust bersifat sensitivitas terhadap huruf besar dan kecil.

## Sintaksis

Tipe data adalah opsional saat mendeklarasikan variabel di Rust. Tipe data akan diambil dari nilai yang ditetapkan pada variabel.

Sintaks untuk mendeklarasikan variabel adalah sebagai berikut.

```rust
let variable_name = value;            // tanpa spesifikasi tipe
let variable_name:dataType = value;   // tipe spesifikasi
```

**Ilustrasi**

```rust
fn main() {
   let fees = 25_000;
   let salary:f64 = 35_000.00;
   println!("fees adalah {} dan salary adalah {}", fees, salary);
}
```

Output dari kode di atas akan menjadi:

```console
fees adalah 25000 dan salary adalah 35000.
```

## Immutable

Secara default, variabel adalah **_immutable_** − hanya bisa dibaca di Rust. Dengan kata lain, nilai variabel tidak dapat diubah setelah nilai diikat ke nama variabel.

Mari kita pahami ini dengan contoh.

```rust
fn main() {
   let fees = 25_000;
   println!("fees adalah {} ", fees);
   fees = 35_000;
   println!("fees yang diubah adalah {}", fees);
}
```

Outputnya akan ditampilkan sebagai berikut:

```
error[E0384]: re-assignment of immutable variable `fees`
 --> main.rs:6:3
   |
 3 | let fees = 25_000;
   | ---- penugasan pertama ke `fees`
...
 6 | fees=35_000;
   | ^^^^^^^^^^^ penugasan ulang variabel immutable

error: aborting due to previous error(s)
```

Pesan kesalahan menunjukkan penyebab kesalahan – Anda tidak dapat menetapkan nilai dua kali ke variabel immutable `fees`. Ini adalah salah satu dari banyak cara di mana Rust memungkinkan programmer untuk menulis kode dan memanfaatkan keamanan serta kemudahan dalam concurrency.

## Mutable

Variabel bersifat immutable secara default. Tambahkan awalan `mut` pada nama variabel untuk menjadikannya mutable. Nilai dari variabel mutable dapat diubah.

Sintaks untuk mendeklarasikan variabel mutable adalah sebagai berikut:

```rust
let mut variable_name = value;
let mut variable_name:dataType = value;
```

Mari kita pahami ini dengan contoh.

```rust
fn main() {
   let mut fees:i32 = 25_000;
   println!("fees adalah {} ", fees);
   fees = 35_000;
   println!("fees yang diubah adalah {}", fees);
}
```

Output dari cuplikan kode ini adalah sebagai berikut:

```
fees adalah 25000
fees yang diubah adalah 35000
```
