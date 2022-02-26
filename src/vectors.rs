// Vectors are resizeable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    // Data types and length have to be exact


    // Re-assign value
    numbers[2] = 30;

    //Add to vector
    numbers.push(5);
    numbers.push(7);
    numbers.push(5);
    numbers.push(7);
    numbers.push(5);
    numbers.push(7);
    numbers.push(5);
    numbers.push(7);

    // Pop off last value
    numbers.pop();

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


    // loop through vector values
    for x in numbers.iter() {
        println!("{}", x);
    }


    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}