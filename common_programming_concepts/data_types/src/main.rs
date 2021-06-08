use std::io;

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
    let (x, y, z) = tup;
    println!("X: {}\nY: {}\nZ: {}", x, y, z);

    // Access to the values inside tuple using 2nd way
    println!("X: {}\nY: {}\nZ: {}", tup.0, tup.1, tup.2);

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First: {}\nSecond:{}", a[0], a[1]);


    array_index();
}

fn array_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
