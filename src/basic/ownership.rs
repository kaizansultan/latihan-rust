pub fn print() {
    let s1 = String::from("halo");
    take_ownership(s1); // s1 is moved

    let x = 5;
    make_copy(x);
    // x is moved, but because i32 is Copy
    // x is still valid here

    let _s2 = give_ownership();
    let _s3 = take_and_give(String::from("Jupiter"));
}
// nothing special happens because s1's value
// was moved

fn take_ownership(str: String) {
    println!("{str}");
}
// str goes out of scope, it's dropped
// the backing memroy is freed

fn make_copy(int: i32) {
    println!("{int}");
}
// nothing special happens

fn give_ownership() -> String {
    let str = String::from("Warung Mekar");
    str
}

fn take_and_give(str: String) -> String {
    str
}
