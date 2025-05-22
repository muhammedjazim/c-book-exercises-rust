use std::io::stdin;

fn main() {
    let mut dogs_is_string_for_proper_input = String::new();

    println!("How many dogs do you have?\n");
    stdin().read_line(&mut dogs_is_string_for_proper_input).expect("Failed reading number of dogs");

    let dogs: i32 = dogs_is_string_for_proper_input.trim().parse().expect("Not a proper numeric value");

    println!("So you have {} dog(s)", dogs);
}