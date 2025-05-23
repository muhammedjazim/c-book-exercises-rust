use std::io::stdin;

fn main() {
    let weight: f32;
    let value: f32;

    print!("Are you worth your weight in rhodium?\n");
    print!("Let's check it out.\n");
    print!("Please enter your weight in pounds: ");

    let mut input_string: String = String::new();
    stdin().read_line(&mut input_string).expect("Invalid input");
    weight = input_string.trim().parse().expect("Invalid input");

    value = 770.0 * weight * 14.5833;
    print!("Your weight in rhodium is worth ${:.2}.\n", value);
    print!("You are easily worth that! If rhodium prices drop,\n");
    print!("eat more to maintain your value.\n");
}