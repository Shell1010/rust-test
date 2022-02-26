// Tuples group together different values of different types
// Max 12


pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Jeff", 42);

    println!("{} is from {} and is {}", person.0, person.1, person.2)

}