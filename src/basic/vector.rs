pub fn print() {
    let mut v: Vec<i32> = Vec::new();

    // kalo langsung dikasih nilai ga perlu nulis type nya
    let _v_no_type = vec![12, 23, 24];

    v.push(32);
    v.push(14);
    v.push(87);

    println!("elemen kedua {}", &v[1]);

    // akan ngeprint error
    // println!("elemen keseratus {}", v[100]);

    // ngeprint None
    println!("elemen keseratus {:?}", v.get(100));
    // Some(14)
    println!("elemen keseratus {:?}", v.get(1));

    println!("semua elemen: {:?}", v);

    for i in &v {
        print!("{i} ");
    }

    for i in &mut v {
        *i /= 2;
        print!("{i} ");
    }
}
