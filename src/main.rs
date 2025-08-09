fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // diff bet i32 (32-bit signed integer) and u64 (64-bit unsigned integer)
    // range:
    // i32: -2,147,483,648 to 2,147,483,647;
    // u64: 0 to 18,446,744,073,709,551,615
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Max value of i32: {}", e);
    println!("Max value of i64: {}", i);

    // ======================================
    //  Floats [Floating Point Numbers]
    //  f32 (32-bit) and f64 (64-bit)
    let pi: f64 = 3.14;
    println!("Value of Pi: {}", pi);

    //=======================================
    //  Booleans
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // =======================================
    // Characters type - char
    let letter: char = 'a';
    println!("Letter: {}", letter);
}

