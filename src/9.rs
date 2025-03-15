// This program generates a random number between 1 and 6.
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let num: u8 = rng.gen_range(1, 7);
    println!("Your random number is {}", num);
}
