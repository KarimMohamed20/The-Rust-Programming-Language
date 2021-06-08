fn main() {
    another_function(30,-30);
    println!("This should print a value from the function: {}",five());
}

fn another_function(x: u8,y:i8) {
    println!("This is the value of x: {}",x);
    println!("This is the value of y: {}",y);
}

fn five() -> u8 {
    5
}