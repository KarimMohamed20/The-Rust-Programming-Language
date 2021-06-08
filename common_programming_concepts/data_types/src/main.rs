fn main() {
    // Unsigned variable (x) can be assigned to only positive numbers
    let x: u8 = 255;
    println!("This is unsigned variable: {}", x);

    // Signed variable (y) can be assigned to positive and negative numbers
    let y: i8 = -128;
    println!("This is signed variable: {}", y);
    

    // Tuples
    let tup: (i32, u8, f64) = (-1000, 127, 10.0);

    // Access to the values inside tuple using 1st way
    let (x,y,z) = tup;
    println!("X: {}\nY: {}\nZ: {}",x,y,z);
    
    // Access to the values inside tuple using 2nd way
    println!("X: {}\nY: {}\nZ: {}",tup.0,tup.1,tup.2);
}
