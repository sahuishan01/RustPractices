use std::mem::size_of_val;
use std::io::{self, Write};
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn basic_function() {
    println!("This is a basic function");
}

fn take_argument_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn return_value_function(x: i32) -> i32 {
    x + 1
}

fn main() {
    // variables and mutability
    println!("Variables and mutability");
    println!("\tDefining mutable variables");
    let mut x = 5;
    println!("\t\tThe value of x is: {}", x);
    x += 10;
    println!("\t\tModified value of x is: {x}");
    println!("\tShadowing");
    let y = 10;
    println!("\t\tThe value of y is: {}", y);
    let y = "Hello World!";
    println!("The value of y is: {}", y);
    println!("Above the value of y is changed from 10 to \"Hello World!\" using shadowing");
    // Difference between shadowing and mutability
        // Shadowing prevents us from changing a value unintentionally    
        // Shadowing allows us to change the type of the variable
            // Example
                // let mut spaces = "   ";
                // spaces = spaces.len(); // Will throw an error as the spaces variable is of type &str
                let spaces = "   ";
                let spaces = spaces.len(); // Will not throw an error as the spaces variable is of type u32
                println!("\t\tThe length of spaces is: {}", spaces);
    println!("");
    
    
    // Data types
    println!("Data types");

        println!("\t Scalars");
            let x = 10;
            println!("\t\tThe value of x is: {} and it's type is {}", x, type_of(x));
            let y = 10.0;
            println!("\t\tThe value of y is: {} and it's type is {}", y, type_of(y));
            let z = true;
            println!("\t\tThe value of z is: {} and it's type is {}", z, type_of(z));
            let a = 'a';
            println!("\t\tThe value of a is: {} and it's type is {}", a, type_of(a));

        println!("\tScalar type sizes");
            println!("\t\tSize of i8 is: {} bytes", size_of_val(&i8::MAX));
            println!("\t\tSize of i16 is: {} bytes", size_of_val(&i16::MAX));
            println!("\t\tSize of i32 is: {} bytes", size_of_val(&i32::MAX));
            println!("\t\tSize of i64 is: {} bytes", size_of_val(&i64::MAX));
            println!("\t\tSize of i128 is: {} bytes", size_of_val(&i128::MAX));
            println!("\t\tSize of isize is: {} bytes", size_of_val(&isize::MAX));
            println!("\t\tSize of u8 is: {} bytes", size_of_val(&u8::MAX));
            println!("\t\tSize of u16 is: {} bytes", size_of_val(&u16::MAX));
            println!("\t\tSize of u32 is: {} bytes", size_of_val(&u32::MAX));
            println!("\t\tSize of u64 is: {} bytes", size_of_val(&u64::MAX));
            println!("\t\tSize of u128 is: {} bytes", size_of_val(&u128::MAX));
            println!("\t\tSize of usize is: {} bytes", size_of_val(&usize::MAX));
            println!("\t\tSize of f32 is: {} bytes", size_of_val(&f32::MAX));
            println!("\t\tSize of f64 is: {} bytes", size_of_val(&f64::MAX));
            println!("\t\tSize of char is: {} bytes", size_of_val(&char::MAX));
            println!("\t\tSize of bool is: {} bytes", size_of_val(&bool::default()));
        println!("\tNumeric Operations");
            let x = 7;
            let y = 3;
            println!("\t\tThe value of x is: {x} and y is: {y}");
            println!("\t\tThe value of x + y is: {}", x+y);
            println!("\t\tThe value of x - y is: {}", x-y);
            println!("\t\tThe value of x * y is: {}", x*y);
            println!("\t\tThe value of x / y is: {}", x/y); // this is due to the fact that both x and y are integers
            println!("\t\tThe values of x / 3.0 is: {}", x as f64/3.0); // this is due to the fact that x is converted to float
            println!("\t\tThe value of x % y is: {}", x%y); // Gives the remainder of x/y
        println!("\tCompounds type");
            println!("\t\tNote: Tuples are immutable and can store different types of variables");
            let tuple1 = (1, "ishan", 2.0, true);
            println!("\t\t\tThe value of tuple1 is: {:?}", tuple1);
            println!("\t\tNote: Arrays are immutable and can store same types of variables");
            let mut array1 = [1, 2, 3, 4, 5];
            println!("\t\t\tThe value of array1 is: {:?}", array1);
            let mut index: u8;
            loop {
                print!("\t\t\tEnter the index of the array you want to change\t");
                io::stdout().flush().unwrap();
                let mut temp_string = String::new();
                io::stdin()
                    .read_line(&mut temp_string)
                    .expect("Failed to read line");
                index = match temp_string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\t\t\tPlease enter a number!");
                        continue;
                    }
                };
                if index >= array1.len() as u8 {
                    println!("\t\t\tPlease enter a number less than {}", array1.len());
                    continue;
                }
                break;
            }
            loop{
                print!("\t\t\tEnter the new value\t");
                io::stdout().flush().unwrap();
                let mut temp_string = String::new();
                io::stdin()
                    .read_line(&mut temp_string)
                    .expect("Failed to read line");
                array1[index as usize] = match temp_string.trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error: {}", err);
                        println!("\t\t\tPlease enter a number in range of i32!");
                        continue;
                    }
                };
                break;
            }
            println!("\t\tThe updated value of array1 is: {:?}", array1);
    
    println!("");

    // functions
    // statements and expressions
    println!("Functions");
        print!("\tCalling a basic function: ");
        basic_function();
        print!("\tCalling a function with argument: ");
        take_argument_function(10);
        print!("\tCalling a function with return value: ");
        println!("The value returned by the function is: {}", return_value_function(10));
    println!("");

    // control flow
    print!("Control flow");
    // if statement
    println!("\tDivisibility check by 4 or 3 or 2, using if statement");
    let number: i64;
    loop {
        let mut temp_string = String::new();
        io::stdin()
            .read_line(&mut temp_string)
            .expect("Failed to read line");
        number = match temp_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\t\tPlease enter a number!");
                continue;
            }
        };
        break;
    }

    if number % 4 == 0 {
        println!("\t\tnumber is divisible by 4");
    } else if number % 3 == 0 {
        println!("\t\tnumber is divisible by 3");
    } else if number % 2 == 0 {
        println!("\t\tnumber is divisible by 2");
    } else {
        println!("\t\tnumber is not divisible by 4, 3, or 2");
    }
    println!("");

    // ternary operation
    println!("\\tTernary operation");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("\t\tThe value of number is: {}", number);

    // let number = if condition { 5 } else { "6" }; // error due to different types of variables
    // println!("The value of number is: {}", number);

    // loop
    println!("\tLoop with return value");
    let mut counter = 0;
    let result = loop {
        println!("\t\tcounter is: {}", counter);
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("\tresult is: {:?}", result);

    // while

    let arr = [1324, 324, 3243, 1235, 2343, 532, 4, 324, 324];
    let mut index = 0;
    println!("\tWhile loop");
    while index < arr.len() / 2 {
        println!("\t\tvalue is: {}", arr[index]);
        index += 1;
    }

    // for loop
    println!("\tFor loop");
    for element in arr {
        println!("\t\tthe value is: {element}");
    }

    // reverse
    println!("\tReverse for loop");
    for number in (1..4).rev() {
        println!("\t\t{number}");
    }
    let mut number = 3;
    println!("\t breaking out of parent loop");
    'outer: loop {
        println!("\t\tStarting Inner loop");
        'inner: loop {
            number += 1;
            println!("\t\t\tThe value of number is: {}", number);
            if number % 5 == 0 {
                println!("\t\t\tBreaking out of inner loop");
                break 'inner;
            }
            if number % 7 == 0 && number % 3 == 0 {
                println!("\t\t\tBreaking out of outer loop");
                break 'outer;
            }
        }
    }


}
