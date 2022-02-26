// Loops user to iterate until a condition is met

pub fn run() {

    let mut count = 0;


    // Infinite loop
    // loop {
    //     count += 1;
    //     println!("{}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }


    // While Loop fizzbuzz (if divisible by 3, print fizz, if divisible by 5, print buzz, if divisible by both, print fizzbuzz)
    while count <= 100000 {
        count += 1;
        if count % 15 == 0 {
            println!("Fizzbuzz!");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", count)
        }
    

    } 
    //For Range
    for _x in 0..100 {
        println!("Jeff")
    }


}