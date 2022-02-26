mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod funcs;
mod pointers_refs;
mod strucs;
mod enums;
mod cli;
use std::io;

fn main() {
    loop {
        println!("Please input your choice for tutorial, do check their files for comments\n 1 ---> Printing\n 2 ---> Variables\n 3 ---> Types\n 4 ---> Strings\n 5 ---> Tuples\n 6 ---> Arrays\n 7 ---> Vectors\n 8 ---> Conditionals\n 9 ---> Loops\n 10 ---> Functions\n 11 ---> Pointers/References\n 12 ---> Structures\n 13 ---> Enums\n 14 ---> Command Line Interface");
        //Assigns an empty string to the variable
        let mut choice = String::new();
        io::stdin()//Reads input, else it sends exception
        .read_line(&mut choice)
        .expect("Failed to read line");
    
        let choice: u32 = match choice.trim().parse() { //Does so you don't send autism down the choice variable
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            print::run();
        } else if choice == 2 {
            vars::run();
        } else if choice == 3 {
            types::run();
        } else if choice == 4 {
            strings::run();
        } else if choice == 5 {
            tuples::run();
        } else if choice == 6 {
            arrays::run();
        } else if choice == 7 {
            vectors::run();
        } else if choice == 8 {
            conditionals::run();
        } else if choice == 9 {
            loops::run();
        } else if choice == 10{
            funcs::run();
        } else if choice == 11 {
            pointers_refs::run();
        } else if choice == 12 {
            strucs::run();
        } else if choice == 13 {
            enums::run();
        } else if choice == 14 {
            cli::run();
        }
    }
    

}
