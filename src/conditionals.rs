


pub fn run() {
    let age = 12;
    let check_id = true;
    let knows_person = true;

    // if/else statements - && and - || or
    if age >= 21 && check_id || knows_person {
        println!("Age is legal")
    } else if age < 21 && check_id{
        println!("Age is illegal")
    } else {
        println!("gib id")
    }
    

    // shorthand if/else
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age)
}