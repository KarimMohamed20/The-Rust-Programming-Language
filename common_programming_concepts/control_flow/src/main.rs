fn main() {
    let number = 0;

    if number < 5 && number != 0 {
        println!("Number is too small");
    } else if number == 0 {
        println!("Number is zero");
    } else {
        println!("Number is too large");
    }
    print_if_true(true);

    let mut counter = 5;
    loop {
        counter += 1;
        if counter == 10 {
            println!("Counter = 10");
            break;
        }
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }


    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn print_if_true(value: bool) {
    let number = if value { 5 } else { 6 };

    if value == true {
        println!("This is the number: {}", number);
    }
}
