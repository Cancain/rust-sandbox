/*
Primitive types --
u = unsigned, can't be negative
Integers: u8, i8 , u16, i16, u32, i32, u64, i64, u128, i128 (numbers of bits they take in memory)
floats: f32, f64
boolean (bool)
characters (char)
Tuples
Arrays
*/

//Rust is a statically typed language, which means tha it must know the types of all variables att compile time, however,
//the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    //Default is "i32"
    let x = 1;

    //Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 12983189371823;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));
}
