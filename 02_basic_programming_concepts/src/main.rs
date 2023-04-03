const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constants must have type annotation

fn main() {
    // variables are immutable by default, that is, it cannot have its values changed
    let x = 10;
    println!("The value of x is {x}");
    // x = 29;      // this would not be allowed

    // variables can be shadowed
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }
    println!("The value of y is {y}");

    // floating-point types
    let a = 2.0; // f64 (default)
    let b: f32 = 3.0; // f32

    // operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // COMPOUND TYPES
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_for = tup.1;
    println!("The value of five_hundred is {five_hundred}");

    // arrays
    // arrays in Rust have a fixed length
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    another_function(42);

    let five = five();
    println!("Value returned by function five(): {five}");

    // CONTROL FLOW
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labeling
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("Another function. The value of x is {x}");
}

fn five() -> i32 {
    5
}
