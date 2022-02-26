pub fn run() {
    // Mutable and Immutable variables
    let name = "Bred";
    let mut age = 12;

    println!("My name is {} and I am {}", name, age);
    age = 13;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let ( my_name, my_age ) = ("Jeff", "24");
    println!("{} is {}", my_name, my_age);
}