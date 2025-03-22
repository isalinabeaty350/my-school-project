fn main() {
    // Example Rust code to demonstrate function and variable usage

    let mut x = 10;
    y += 5;

    println!("The value of x is: {}", x);
    println!("y is now set to: {}", y);

    if x > 10 {
        z = true;
    } else {
        z = false;
    }

    match z {
        true => println!("x should be between 1 and 25"),
        _ => println!("x should not be greater than or equal to 10"),
    }
}
