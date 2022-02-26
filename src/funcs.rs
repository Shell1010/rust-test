pub fn run() {
    greeting("Jeff", "gaming");
    println!("32 + 45 = {}", add(32, 45));

    let get_sum = add(5, 5);
    println!("{}", get_sum);

    // Closure
    let n3: i32 = 24;
    let add_numes = |n1: i32, n2: i32| n1 + n2;
    println!("C sum: {}", add_numes(12,34) + n3)

}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}", name, greet);
}

fn add(n1 : i32, n2: i32) -> i32 {
    n1 + n2 //Returns
}