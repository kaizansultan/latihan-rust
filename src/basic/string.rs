pub fn print_string() {
    let mut s = String::from("\nhello");
    s.push_str(", bang");
    println!("{s}");

    let x = 6;
    let y = x;
    println!("{x}, {y}");

    let s1 = String::from("Kantru");
    let s2 = s1; // s1 is no longer valid
    println!("{s2}");

    #[allow(unused_assignments)]
    let mut s3 = String::from("Kantru");
    s3 = String::from("Tole"); // old s3 dropped (moved)
    println!("{s3}");

    let s4 = String::from("Mamuk");
    let s5 = s4.clone(); // deep copy (copied)
    println!("s1 = {s4}, s2 = {s5}");
}
