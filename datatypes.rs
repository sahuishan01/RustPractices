use std::fmt;
use std::mem;

// reversing a pair
fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (integer, boolean) = pair;
    (boolean, integer)  // can be returned without the return keyword
}

//matrix
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: Matrix) -> Matrix{
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        
        write!(f, " {{ {} {} }}\n {{ {} {} }}", self.0, self.1, self.2, self.3)
    }
}

fn main(){
    
    //boolean
    let is_true = true;
    println!("Boolean true value is{}", is_true);

    //integer (it automatically detects the type if not specified)
    let int1 = 1; // i32 (default_type)
    let mut int2 = 512;
    println!("non mutable int value is {} and mutable int value is {}", int1, int2);
    int2 = 213213213; // i64 (inferred value)
    println!("non mutable int value is {} and mutated int value is {}", int1, int2);

    // numerical operations
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    // println!("1 - 2 = {}", 1u32 - 2); // will give error because u is unsigned


    // scientific notations
    println!("1e6 is {} and -2e-5 is {}", 1e6, -2e-5);


    // boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    // bitwise operations
    println!("0011 and 1011 is {:04b}", 0b0011u8 & 0b1011u8);
    println!("0011 or 1011 is {:04b}", 0b0011u8 | 0b1011u8);
    println!("0011 xor 1011 is {:04b}", 0b0011u8 ^ 0b1011u8);
    println!("0011 left shift 2 is {:04b}", 0b0011u8 << 2);
    println!("1100 right shift 2 is {:04b}", 0b1100u8 >> 2);

    // working with pairs
    let pair = (1i32, true);
    println!(" pair is {:?} and the reversed pair is {:?}", pair, reverse(pair));

    // working with tuples
    let tup = (1,2,"ishan", "sahu", true, 1.3234);
    println!("elements can be accessed as {}", tup.2); // accessing elements by index
    println!("tuple with debug can be printed as a whole {:?}", tup); // only if length is less than equal to 12

    let tup2 = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    println!("elements can be accessed as {}", tup2.2); // accessing elements by index
    // println!("tuple with debug can be printed as a whole {:?}", tup2);// This will throw an error

    
    let m1 = Matrix(10.2, 10.3, 10.4, 10.5);
    println!("printing matrix with debug: {:?}", m1);
    println!("printing default matrix: \n{}", m1);
    println!("printing transposed matrix: \n{}", transpose(m1));


    // fixed size array
    let arr1: [f64; 5] = [1.111, 213.213213, 21321.21323, 21312.4454, 5643.634]; // array with different values
    println!("array is {:?}", arr1);
    println!("number of elements in the array is {}", arr1.len());
    println!("space occupied by array is {} bytes", mem::size_of_val(&arr1));
    
    let arr2: [i32; 500] = [5; 500]; // values of same type
    println!("array is {:?}", arr2);

    // printing part of array as slice
    println!("slice from 0 to 2 is {:?}", &arr1[0..3]);

    // looping in an array
    println!("Starting to print array values using loop");
    for i in 0..arr1.len() + 1 {
        match arr1.get(i) { // checking for value in the array
            Some(val) => println!("Value is {}", val), // some takes all value inputs returned by match
            None => println!("Too far!!!"), // In case Some fails
        }
    }
    println!("Starting to print slice values using loop");
    for i in 0..arr1[0 .. arr1.len()-2].len() {
        match arr1.get(i) { // checking for value in the array
            Some(val) => println!("Value is {}", val), // some takes all value inputs returned by match
            None => println!("Too far!!!"), // In case Some fails
        }
    }
}