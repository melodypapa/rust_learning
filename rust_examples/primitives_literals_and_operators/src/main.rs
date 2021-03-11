fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // mutable = true;

    let mutable = true;

    println!("logical = {}", logical);
    println!("a_float = {}", a_float);
    println!("an_integer = {}", an_integer);
    println!("default_float = {}", default_float);
    println!("default_integer = {}", default_integer);
    println!("inferred_type = {}", inferred_type);
    println!("mutable = {}", mutable);
}
