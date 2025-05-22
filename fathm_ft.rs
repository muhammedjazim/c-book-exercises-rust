// Converts 2 fathoms to feet
fn main() {
    let feet: i32;
    let fathoms: i32;

    fathoms=2;
    feet=6*&fathoms;

    print!("There are {} feet in {} fathoms!\n", feet, &fathoms);
    print!("Yes, I said {} feet!\n", 6*fathoms)
}