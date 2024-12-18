# Tipe Data

Sistem Tipe merepresentasikan berbagai jenis nilai yang didukung oleh bahasa pemrograman.

Sistem Tipe memeriksa validitas nilai yang diberikan sebelum nilai tersebut disimpan atau dimanipulasi oleh program. Ini memastikan bahwa kode berfungsi sesuai yang diharapkan. Sistem Tipe juga memungkinkan pemberian petunjuk kode yang lebih kaya dan dokumentasi otomatis.

Rust adalah bahasa dengan tipe statis. Setiap nilai dalam Rust memiliki tipe data tertentu. Compiler dapat secara otomatis menyimpulkan tipe data dari variabel berdasarkan nilai yang diberikan padanya.

Pada contoh di atas, tipe data variabel akan disimpulkan dari nilai yang diberikan. Misalnya, Rust akan menetapkan tipe data string pada variabel `company_string`, tipe data float pada `rating_float`, dan sebagainya.

Macro `println!` membutuhkan dua argumen:

- Sintaks khusus `{ }`, yang berfungsi sebagai placeholder
- Nama variabel atau konstanta

Placeholder akan digantikan oleh nilai variabel.

## Tipe Skalar

Tipe skalar merepresentasikan satu nilai tunggal. Contohnya adalah `10`, `3.14`, `'c'`. Rust memiliki empat tipe skalar utama:

- Integer
- Floating-point
- Boolean
- Karakter

Kita akan mempelajari setiap tipe ini di bagian berikutnya.

### Integer

Integer adalah bilangan tanpa komponen pecahan alias bilangan bulat.

Integer dapat diklasifikasikan lebih lanjut menjadi _Signed_ dan _Unsigned_. Integer bertanda (signed) dapat menyimpan nilai negatif dan positif. Integer tanpa tanda (unsigned) hanya dapat menyimpan nilai positif. Berikut adalah deskripsi rinci tipe integer:

| No  | Ukuran  | Signed | Unsigned |
| --- | ------- | ------ | -------- |
| 1   | 8 bit   | i8     | u8       |
| 2   | 16 bit  | i16    | u16      |
| 3   | 32 bit  | i32    | u32      |
| 4   | 64 bit  | i64    | u64      |
| 5   | 128 bit | i128   | u128     |
| 6   | Arch    | isize  | usize    |

Ukuran integer bisa bergantung pada arsitektur (arch). Artinya, ukuran tipe data akan ditentukan oleh arsitektur mesin. Integer dengan ukuran arch akan memiliki 32 bit pada mesin x86 dan 64 bit pada mesin x64. Integer tipe arch umumnya digunakan saat melakukan pengindeksan pada koleksi.

**Ilustrasi**

```rust
fn main() {
   let result = 10;    // secara default i32
   let age:u32 = 20;
   let sum:i32 = 5 - 15;
   let mark:isize = 10;
   let count:usize = 30;
   println!("nilai result adalah {}", result);
   println!("sum adalah {} dan age adalah {}", sum, age);
   println!("mark adalah {} dan count adalah {}", mark, count);
}
```

Hasil output akan seperti berikut:

```console
nilai result adalah 10
sum adalah -10 dan age adalah 20
mark adalah 10 dan count adalah 30
```

Kode di atas akan menghasilkan kesalahan kompilasi jika nilai `age` diganti dengan nilai bertipe floating-point.

#### Rentang Integer

Setiap varian bertanda dapat menyimpan bilangan dari -(2^(n-1)) hingga 2^(n-1)-1, di mana n adalah jumlah bit yang digunakan oleh varian tersebut. Misalnya, i8 dapat menyimpan bilangan dari -(2^7) hingga 2^7-1.

Setiap varian tanpa tanda dapat menyimpan bilangan dari 0 hingga (2^n)-1. Misalnya, u8 dapat menyimpan bilangan dari 0 hingga (2^8)-1, yang sama dengan 0 hingga 255.

#### Overflow Integer

Overflow integer terjadi ketika nilai yang diberikan ke variabel integer melebihi rentang yang didefinisikan oleh Rust untuk tipe data tersebut. Berikut contohnya:

```rust
fn main() {
   let age:u8 = 255;
   // hanya nilai 0 hingga 255 yang diperbolehkan untuk u8

   let weight:u8 = 256;   // nilai overflow adalah 0
   let height:u8 = 257;   // nilai overflow adalah 1
   let score:u8 = 258;    // nilai overflow adalah 2

   println!("age adalah {} ", age);
   println!("weight adalah {}", weight);
   println!("height adalah {}", height);
   println!("score adalah {}", score);
}
```

Rentang valid variabel unsigned u8 adalah 0 hingga 255. Dalam contoh di atas, variabel diberi nilai yang lebih besar dari 255 (batas atas untuk variabel integer di Rust). Saat dijalankan, kode di atas akan menghasilkan peringatan: literal melebihi rentang untuk variabel `weight`, `height`, dan `score`. Nilai overflow setelah 255 akan dimulai dari 0, 1, 2, dan seterusnya. Hasil akhirnya tanpa peringatan adalah sebagai berikut:

```
age adalah 255
weight adalah 0
height adalah 1
score adalah 2
```

### Float

Tipe data Float di Rust dapat diklasifikasikan sebagai f32 dan f64. Tipe f32 adalah float presisi tunggal, sedangkan f64 memiliki presisi ganda. Tipe default adalah f64. Pertimbangkan contoh berikut untuk memahami lebih lanjut tentang tipe data float.

```rust
fn main() {
   let result = 10.00;        // f64 secara default
   let interest:f32 = 8.35;
   let cost:f64 = 15000.600;  // presisi ganda

   println!("nilai result adalah {}", result);
   println!("interest adalah {}", interest);
   println!("cost adalah {}", cost);
}
```

Outputnya akan ditampilkan sebagai berikut:

```
interest adalah 8.35
cost adalah 15000.6
```

## Pengubahan Tipe Otomatis

Pengubahan tipe otomatis tidak diperbolehkan di Rust. Pertimbangkan cuplikan kode berikut. Nilai integer ditetapkan ke variabel float `interest`.

```rust
fn main() {
   let interest:f32 = 8;   // integer ditetapkan ke variabel float
   println!("interest adalah {}", interest);
}
```

Compiler akan mengeluarkan kesalahan tipe yang tidak cocok seperti yang ditunjukkan di bawah ini.

```
error[E0308]: mismatched types
   --> main.rs:2:22
   |
 2 | let interest:f32=8;
   |    ^ expected f32, found integral variable
   |
   = note: expected type `f32`
      found type `{integer}`
error: aborting due to previous error(s)
```

## Pemisah Angka

Untuk memudahkan pembacaan angka besar, kita dapat menggunakan pemisah visual \_ garis bawah untuk memisahkan digit. Contohnya, 50.000 dapat ditulis sebagai 50_000. Ini ditunjukkan dalam contoh berikut.

```rust
fn main() {
   let float_with_separator = 11_000.555_001;
   println!("nilai float {}", float_with_separator);

   let int_with_separator = 50_000;
   println!("nilai int {}", int_with_separator);
}
```

Outputnya adalah sebagai berikut:

```
nilai float 11000.555001
nilai int 50000
```

## Boolean

Tipe Boolean memiliki dua nilai yang mungkin – `true` atau `false`. Gunakan kata kunci `bool` untuk mendeklarasikan variabel boolean.

**Ilustrasi**

```rust
fn main() {
   let isfun:bool = true;
   println!("Apakah Pemrograman Rust Menyenangkan? {}", isfun);
}
```

Output dari kode di atas akan menjadi:

```
Apakah Pemrograman Rust Menyenangkan? true
```

## Char

Tipe data karakter di Rust mendukung angka, alfabet, Unicode, dan karakter khusus. Gunakan kata kunci `char` untuk mendeklarasikan variabel tipe data karakter. Tipe `char` Rust merepresentasikan Nilai Skalar Unicode, yang berarti ia dapat merepresentasikan lebih banyak hal daripada hanya ASCII. Nilai Skalar Unicode berkisar dari U+0000 hingga U+D7FF dan U+E000 hingga U+10FFFF inklusif.

Mari kita lihat contoh untuk memahami lebih lanjut tentang tipe data karakter.

```rust
fn main() {
   let special_character = '@'; // default
   let alphabet:char = 'A';
   let emoji:char = '😁';

   println!("karakter khusus adalah {}", special_character);
   println!("alfabet adalah {}", alphabet);
   println!("emoji adalah {}", emoji);
}
```

Output dari kode di atas akan menjadi:

```
karakter khusus adalah @
alfabet adalah A
emoji adalah 😁
```
