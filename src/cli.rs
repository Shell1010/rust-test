use std::env;

pub fn run() {
    // Collects values for args upon cargo run
    let args: Vec<String> = env::args().collect();

    // Sets arg from cargo run as the command variable, this will allow you to use the arg from cli in the code
    let command = args[1].clone();
    println!("{}", command);

    /*
    Doing cargo run hello, will output the path, and then later the argument hello.
    The code above will allow you to store that argument in a variable for later use.
    */

    println!("Args: {:?}", args);

}