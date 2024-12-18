# Konstanta

> Bab ini sebaiknya ditaruh di tengah setelah pelajari fungsi dan shadowing

Konstanta mewakili nilai yang tidak dapat diubah. Jika kamu mendeklarasikan sebuah konstanta, tidak ada cara untuk mengubah nilainya. Kata kunci untuk menggunakan konstanta adalah `const`.

Konstanta harus memiliki tipe yang ditentukan secara eksplisit. Berikut adalah sintaks untuk mendeklarasikan sebuah konstanta.

```rust
const VARIABLE_NAME:dataType = value;
```

# Konvensi Penamaan Konstanta di Rust\*\*

Konvensi penamaan untuk konstanta mirip dengan variabel. Semua karakter dalam nama konstanta biasanya ditulis dalam huruf kapital. Tidak seperti deklarasi variabel, kata kunci `let` tidak digunakan untuk mendeklarasikan konstanta.

Berikut contoh penggunaan konstanta di Rust:

```rust
fn main() {
   const USER_LIMIT:i32 = 100;    // Deklarasi konstanta integer
   const PI:f32 = 3.14;           // Deklarasi konstanta float

   println!("user limit adalah {}",USER_LIMIT);  // Menampilkan nilai konstanta
   println!("nilai pi adalah {}",PI);            // Menampilkan nilai konstanta
}
```

**Perbedaan Konstanta dan Variabel**

Di bagian ini, kita akan mempelajari faktor-faktor pembeda antara konstanta dan variabel.

- Konstanta dideklarasikan menggunakan kata kunci `const`, sedangkan variabel dideklarasikan menggunakan kata kunci `let`.
- Deklarasi variabel bisa memiliki tipe data secara opsional, sedangkan deklarasi konstanta harus menentukan tipe data. Misalnya, `const USER_LIMIT=100` akan menghasilkan kesalahan.
- Variabel yang dideklarasikan menggunakan kata kunci `let` secara default tidak bisa diubah (immutable). Namun, kamu dapat mengubahnya dengan menggunakan kata kunci `mut`. Konstanta selalu immutable.
- Konstanta hanya dapat diatur ke ekspresi konstanta dan bukan hasil dari pemanggilan fungsi atau nilai lain yang dihitung saat runtime.
- Konstanta dapat dideklarasikan dalam skop apapun, termasuk skop global, yang membuatnya berguna untuk nilai yang perlu diketahui oleh banyak bagian dari kode.

**Shadowing Variabel dan Konstanta**

Rust memungkinkan programmer untuk mendeklarasikan variabel dengan nama yang sama. Dalam hal ini, variabel baru akan menggantikan variabel sebelumnya.

Mari kita pahami ini dengan contoh:

```rust
fn main() {
   let salary = 100.00;
   let salary = 1.50;
   // membaca salary pertama
   println!("Nilai salary adalah :{}", salary);
}
```

Kode di atas mendeklarasikan dua variabel dengan nama `salary`. Deklarasi pertama diberikan nilai 100.00, sementara deklarasi kedua diberikan nilai 1.50. Variabel kedua menutupi atau menyembunyikan variabel pertama saat menampilkan output.

**Output:**

```
Nilai salary adalah :1.50
```

Rust mendukung variabel dengan tipe data yang berbeda saat shadowing.

Pertimbangkan contoh berikut:

Kode ini mendeklarasikan dua variabel dengan nama `uname`. Deklarasi pertama diberikan nilai string, sedangkan deklarasi kedua diberikan nilai integer. Fungsi `len` mengembalikan jumlah total karakter dalam nilai string.

```rust
fn main() {
   let uname = "Mohtashim";
   let uname = uname.len();
   println!("nama diubah menjadi integer: {}", uname);
}
```

**Output:**

```
nama diubah menjadi integer: 9
```

Berbeda dengan variabel, konstanta tidak dapat di-shadow. Jika variabel dalam program di atas diganti dengan konstanta, kompiler akan menampilkan kesalahan.

```rust
fn main() {
   const NAME:&str = "Mohtashim";
   const NAME:usize = NAME.len();
   //Error : `NAME` sudah didefinisikan
   println!("nama diubah menjadi integer: {}", NAME);
}
```
