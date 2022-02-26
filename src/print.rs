pub fn run(){
    //Print to Console like this
    println!("This is how you print!");

    //Basic format printing
    println!("Number: {}\nString: {}", 1, "Jeff");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "London", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity= "Games");

    // Placeholder traits
    println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}", 10, 10, 10);

    // Placeholder for debug trait - use this if you wanna print entire arrays/tuples/vectors
    println!("{:?}", (12, true, "Jeff", "ok", 1));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}