// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let mut numbers: [i32; 7] = [1,2,3,4,5, 6, 7];
    // Data types and length have to be exact

   

    // Re-assign value
    numbers[2] = 30;

    // Get single value

    println!("{}", numbers[0]);

    // Get entire array
    println!("{:?}", numbers);

    // Get array length
    println!("{}", numbers.len());

    // Arrays are stack allocated
    println!("{}", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);


}