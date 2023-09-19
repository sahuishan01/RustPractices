fn main() {
    // if statement

    let number = 10;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // ternary operation

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // let number = if condition { 5 } else { "6" }; // error due to different types of variables
    // println!("The value of number is: {}", number);

    // loop

    let mut counter = 0;
    let result = loop {
        println!("counter is: {}", counter);
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result is: {:?}", result);

    // while

    let arr = [1324, 324, 3243, 1235, 2343, 532, 4, 324, 324];
    let mut index = 0;

    while index < arr.len() / 2 {
        println!("value is: {}", arr[index]);
        index += 1;
    }

    // for loop
    for element in arr {
        println!("the value is: {element}");
    }

    // reverse
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
