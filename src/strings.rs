pub fn run() {
    let mut hello = String::from("Hello");
    
    // Get Length
    println!("Length: {}", hello.len());

    // Push Char, adds to it
    hello.push('w');

    // Push to String, adds to it
    hello.push_str(" UwU owo");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Is empty?
    println!("Is Empty: {}", hello.is_empty());

    // Check if the string containts world
    println!("Contains 'World'? {}", hello.contains("World"));

    // Replace words in a string
    println!("Repalce: {}", hello.replace("UwU", "I'm pro"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(19);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing, if left == right, pass, else error
    assert_eq!(2, s.len());

    println!("{}", hello);

}