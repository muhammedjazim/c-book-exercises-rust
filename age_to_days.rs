use std::io::stdin;

fn main() {
    let age: i32;
    let mut input_age: String = String::new();

    println!("Please enter your age in years");
    stdin().read_line(&mut input_age).expect("Invalid age");

    age = input_age.trim().parse().expect("not a valid age");

    let days: f32 = (age*364) as f32;

    println!("You are {:.2} days old", days);
}