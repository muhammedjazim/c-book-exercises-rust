use std::io::stdin;

fn main() {
    let mut name: String = String::new();
    let mut address: String = String::new();
    
    println!("Please enter your name:");
    stdin().read_line(&mut name).expect("invalid name");

    println!("Please enter your address:");
    stdin().read_line(&mut address).expect("invalid address");

    print!("The name is {}\n", name);
    print!("address is {}\n", address);
}