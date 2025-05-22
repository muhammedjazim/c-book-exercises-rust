use std::io::stdin;

fn main() {
    let mut inches_input: String = String::new();
    println!("Please enter the length in inches\n");

    stdin().read_line(&mut inches_input).expect("Failed to read the input in inches");

    let inches: f64 = inches_input.trim().parse().expect("Not a valid number");

    let cms: f64 = &inches * 2.54;

    println!("{:.2} inches is {:.2} cms", inches, cms);
}