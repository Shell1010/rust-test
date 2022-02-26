/*
    Primitive types --
    Intergers u8, i8 to u128, i128 - unsigned means can't be negative
    Floats: f32, f64
    Boolean: bool
    Characters (char)
    Tuples
    Arrays
*/


pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    //Add explicit type
    let z: i64 = 43234;

    //Find max size
    println!("Max i8: {}", std::i8::MAX);
    println!("Max i16: {}", std::i16::MAX);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    //Get boolean from expression
    let is_greater: bool = 10 > 5;

    //Char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z,is_active, is_greater, a1, face));

}