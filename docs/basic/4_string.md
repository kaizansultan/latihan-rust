Tipe data _String_ di Rust dapat diklasifikasikan sebagai berikut:

- String Literal (`&str`)
- String Object (`String`)

### String Literal

String literal (`&str`) digunakan ketika nilai string sudah diketahui saat _compile time_. String literal adalah sekumpulan karakter yang ditulis secara langsung (hardcoded) ke dalam variabel. Contohnya: `let company="Tutorials Point"`. String literal terdapat dalam modul `std::str`. String literal juga dikenal sebagai _string slices_.

Contoh berikut mendeklarasikan dua string literal: `company` dan `location`.

```rust
fn main() {
   let company: &str = "TutorialsPoint";
   let location: &str = "Hyderabad";
   println!("company is : {} location : {}", company, location);
}
```

String literal bersifat statis secara default. Artinya, string literal dijamin akan tetap valid selama program berjalan. Kita juga dapat secara eksplisit mendeklarasikan variabel sebagai statis seperti ditunjukkan di bawah ini:

```rust
fn main() {
   let company: &'static str = "TutorialsPoint";
   let location: &'static str = "Hyderabad";
   println!("company is : {} location : {}", company, location);
}
```

Program di atas akan menghasilkan output berikut:

```
company is : TutorialsPoint location : Hyderabad
```

### String Object

Tipe _String object_ disediakan oleh _Standard Library_. Berbeda dengan string literal, tipe _String object_ bukan bagian dari bahasa inti. Tipe ini didefinisikan sebagai struktur publik di perpustakaan standar (`pub struct String`). _String_ adalah koleksi yang bisa bertambah (growable). Tipe ini bersifat mutable dan berkodekan UTF-8. Tipe _String object_ dapat digunakan untuk merepresentasikan nilai string yang diberikan saat _runtime_. String object dialokasikan di heap.

#### Sintaks

Untuk membuat _String object_, kita bisa menggunakan salah satu sintaks berikut:

- `String::new()`

Sintaks di atas membuat string kosong.

- `String::from()`

Sintaks ini membuat string dengan nilai default yang dilewatkan sebagai parameter ke metode `from()`.

Contoh berikut mengilustrasikan penggunaan _String object_.

```rust
fn main() {
   let empty_string = String::new();
   println!("length is {}", empty_string.len());

   let content_string = String::from("TutorialsPoint");
   println!("length is {}", content_string.len());
}
```

Contoh di atas membuat dua string: _String object_ kosong menggunakan metode `new` dan _String object_ dari string literal menggunakan metode `from`.

**Output:**

```
length is 0
length is 14
```

### Metode Umum - String Object

| No. | Metode               | Tanda Tangan                                                   | Deskripsi                                                              |
| --- | -------------------- | -------------------------------------------------------------- | ---------------------------------------------------------------------- |
| 1   | `new()`              | `pub const fn new() -> String`                                 | Membuat _String_ kosong baru.                                          |
| 2   | `to_string()`        | `fn to_string(&self) -> String`                                | Mengonversi nilai yang diberikan menjadi _String_.                     |
| 3   | `replace()`          | `pub fn replace<'a, P>(&'a self, from: P, to: &str) -> String` | Mengganti semua kecocokan pola dengan string lain.                     |
| 4   | `as_str()`           | `pub fn as_str(&self) -> &str`                                 | Mengekstrak _string slice_ yang berisi seluruh string.                 |
| 5   | `push()`             | `pub fn push(&mut self, ch: char)`                             | Menambahkan karakter yang diberikan ke akhir _String_.                 |
| 6   | `push_str()`         | `pub fn push_str(&mut self, string: &str)`                     | Menambahkan _string slice_ yang diberikan ke akhir _String_.           |
| 7   | `len()`              | `pub fn len(&self) -> usize`                                   | Mengembalikan panjang _String_ dalam byte.                             |
| 8   | `trim()`             | `pub fn trim(&self) -> &str`                                   | Mengembalikan _string slice_ dengan spasi awal dan akhir dihapus.      |
| 9   | `split_whitespace()` | `pub fn split_whitespace(&self) -> SplitWhitespace`            | Membagi _string slice_ berdasarkan spasi dan mengembalikan iterator.   |
| 10  | `split()`            | `pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>`        | Mengembalikan iterator dari substring _string slice_, dipisahkan pola. |
| 11  | `chars()`            | `pub fn chars(&self) -> Chars`                                 | Mengembalikan iterator dari karakter _string slice_.                   |

Ilustrasi: `new()`

Sebuah objek string kosong dibuat menggunakan metode `new()` dan nilainya diatur menjadi "hello".

```rust
fn main(){
   let mut z = String::new();
   z.push_str("hello");
   println!("{}",z);
}
```

**Output:**

Program di atas menghasilkan output berikut −

```
hello
```

Ilustrasi: `to_string()`

Untuk mengakses semua metode objek _String_, konversi string literal ke tipe objek menggunakan fungsi `to_string()`.

```rust
fn main(){
   let name1 = "Hello TutorialsPoint , Hello!".to_string();
   println!("{}",name1);
}
```

**Output:**

Program di atas menghasilkan output berikut −

```
Hello TutorialsPoint , Hello!
```

Ilustrasi: `replace()`

Fungsi `replace()` menerima dua parameter − parameter pertama adalah pola string yang ingin dicari, dan parameter kedua adalah nilai baru yang akan menggantikan pola tersebut. Pada contoh di atas, "Hello" muncul dua kali dalam string `name1`.

Fungsi `replace` mengganti semua kemunculan string "Hello" dengan "Howdy".

```rust
fn main(){
   let name1 = "Hello TutorialsPoint , Hello!".to_string();  //String object
   let name2 = name1.replace("Hello","Howdy");                //find and replace
   println!("{}",name2);
}
```

**Output:**

Program di atas menghasilkan output berikut −

```
Howdy TutorialsPoint , Howdy!
```

Ilustrasi: `as_str()`

Fungsi `as_str()` mengekstrak _string slice_ yang berisi seluruh string.

```rust
fn main() {
   let example_string = String::from("example_string");
   print_literal(example_string.as_str());
}
fn print_literal(data: &str ){
   println!("displaying string literal {}",data);
}
```

**Output:**

Program di atas menghasilkan output berikut −

```
displaying string literal example_string
```

Ilustrasi: `push()`

Fungsi `push()` menambahkan karakter yang diberikan ke akhir _String_.

```rust
fn main(){
   let mut company = "Tutorial".to_string();
   company.push('s');
   println!("{}",company);
}
```

**Output:**

Program di atas menghasilkan output berikut −

```
Tutorials
```

Ilustrasi: `push_str()`

Fungsi `push_str()` menambahkan _string slice_ yang diberikan ke akhir _String_.

```rust
fn main(){
   let mut company = "Tutorials".to_string();
   company.push_str(" Point");
   println!("{}",company);
}
```

**Output:**

Program di atas menghasilkan output berikut −

```
Tutorials Point
```

Ilustrasi: `len()`

Fungsi `len()` mengembalikan jumlah total karakter dalam string (termasuk spasi).

```rust
fn main() {
   let fullname = " Tutorials Point";
   println!("length is {}",fullname.len());
}
```

**Output:**

```
length is 20
```

Ilustrasi: `trim()`

Fungsi `trim()` menghapus spasi di awal dan akhir string. Catatan: fungsi ini tidak akan menghapus spasi di tengah.

```rust
fn main() {
   let fullname = " Tutorials Point \r\n";
   println!("Before trim ");
   println!("length is {}",fullname.len());
   println!();
   println!("After trim ");
   println!("length is {}",fullname.trim().len());
}
```

**Output:**

```
Before trim
length is 24

After trim
length is 15
```

Ilustrasi: `split_whitespace()`

Fungsi `split_whitespace()` membagi input string menjadi beberapa string. Fungsi ini mengembalikan iterator, sehingga kita dapat mengiterasi melalui token seperti yang ditunjukkan di bawah ini:

```rust
fn main(){
   let msg = "Tutorials Point has good tutorials".to_string();
   let mut i = 1;

   for token in msg.split_whitespace(){
      println!("token {} {}",i,token);
      i+=1;
   }
}
```

**Output:**

```
token 1 Tutorials
token 2 Point
token 3 has
token 4 good
token 5 tutorials
```

Ilustrasi: `split() string`

Metode `split()` mengembalikan iterator dari substring _string slice_, yang dipisahkan oleh karakter yang cocok dengan pola. Keterbatasan metode `split()` adalah bahwa hasilnya tidak dapat disimpan untuk digunakan nanti. Metode `collect()` dapat digunakan untuk menyimpan hasil yang dikembalikan oleh `split()` sebagai vektor.

```rust
fn main() {
   let fullname = "Kannan,Sudhakaran,Tutorialspoint";

   for token in fullname.split(","){
      println!("token is {}",token);
   }

   //simpan dalam vektor
   println!("\n");
   let tokens: Vec<&str> = fullname.split(",").collect();
   println!("firstName is {}",tokens[0]);
   println!("lastname is {}",tokens[1]);
   println!("company is {}",tokens[2]);
}
```

Contoh di atas membagi string `fullname` setiap kali menemukan koma (`,`).

**Output:**

```
token is Kannan
token is Sudhakaran
token is Tutorialspoint

firstName is Kannan
lastname is Sudhakaran
company is Tutorialspoint
```

Ilustrasi: `chars()`

Karakter individu dalam string dapat diakses menggunakan metode `chars()`. Berikut contoh untuk memahaminya.

```rust
fn main(){
   let n1 = "Tutorials".to_string();

   for n in n1.chars(){
      println!("{}",n);
   }
}
```

**Output:**

```
T
u
t
o
r
i
a
l
s
```

### Konkatenasi String dengan Operator +

Nilai string dapat ditambahkan ke string lain. Ini disebut konkatenasi atau interpolasi. Hasil dari konkatenasi string adalah objek string baru. Operator `+` secara internal menggunakan metode `add`. Sintaks fungsi `add` menerima dua parameter. Parameter pertama adalah _self_ – objek string itu sendiri, dan parameter kedua adalah referensi objek string kedua. Ini ditunjukkan di bawah ini −

```rust
//fungsi add
add(self, &str) -> String {
   // mengembalikan objek String
}
```

Ilustrasi: Konkatenasi String

```rust
fn main(){
   let n1 = "Tutorials".to_string();
   let n2 = "Point".to_string();

   let n3 = n1 + &n2; // referensi n2 dilewatkan
   println!("{}",n3);
}
```

**Output:**

```
TutorialsPoint
```

Ilustrasi: _Type Casting_

Contoh berikut mengilustrasikan konversi angka menjadi objek string −

```rust
fn main(){
   let number = 2020;
   let number_as_string = number.to_string();

   // konversi angka ke string
   println!("{}",number_as_string);
   println!("{}",number_as_string == "2020");
}
```

**Output:**

```
2020
true
```

Ilustrasi: _Format! Macro_

Cara lain untuk menambahkan dua objek string bersama-sama adalah menggunakan fungsi makro bernama `format!`. Penggunaan `format!` ditunjukkan di bawah ini.

```rust
fn main(){
   let n1 = "Tutorials".to_string();
   let n2 = "Point".to_string();
   let n3 = format!("{} {}",n1,n2);
   println!("{}",n3);
}
```

**Output:**

```
Tutorials Point
```
