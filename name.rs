use std::io::stdin;

fn main() {
    let mut f_name: String = String::new();
    let mut l_name: String = String::new();
    println!("Please enter your first name");
    stdin().read_line(&mut f_name).expect("invalid first name");

    println!("Please enter your second name");
    stdin().read_line(&mut l_name).expect("invalid second name");

    println!("{} {}", &f_name, &l_name);
    println!("{}\n{}", &f_name, &l_name);
    print!("{}", f_name);
    print!("{}", l_name);
}