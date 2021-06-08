fn main() {
    // We need to change our variable "x" to be mutable, Variables are immutable by default in Rust.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
