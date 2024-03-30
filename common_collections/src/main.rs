use std::collections::HashMap;


#[derive(Debug)]
enum CustomTypes {
    Int(i32),
    Float(f32),
    IsInt(bool),
}

fn main() {
    // Main types are vector, string and hashmaps

    // Creating empty vector
    let mut v: Vec<i32> = Vec::new(); // i32 is needed to define what type of data is going to be
                                      // stored
    let _v = vec![0, 1, 2]; // no need for defining the datatype as it will be automatically casted
                            // from the values used for creating the vector

    // appending in vec
    v.push(10);
    v.push(11);
    v.push(12);
    v.push(13);
    v.push(19);
    v.push(81);
    v.push(72);
    v.push(83);

    // accesing values individually
    // let sample = &v[10]; // will cause panic as size of v is 8
    let sample = &v.get(10);
    println!("sample is {:?}", sample);
    println!("error checked sample is {:?}", sample.unwrap_or(&5));
    let sample = &v.get(5);
    println!("sixth value is {:?}", sample.unwrap_or(&5));

    // iterating values in a vector
    for value in &v {
        print!("{value} ");
    }
    println!("");

    // storing diffeent types of data using enum
    let mut custom_vector: Vec<CustomTypes> = Vec::new();
    custom_vector.push(CustomTypes::Int(10));
    custom_vector.push(CustomTypes::Float(15.5));
    custom_vector.push(CustomTypes::IsInt(false));
    
    for value in &custom_vector {
        print!("{:?} ", value);
    }
    println!("");
    println!("*****************************************************************");
    println!("String manipulation");

    let s = "to_string method".to_string();
    println!("String using to_string is {s}");

    let s = String::from("Stirng using Sting::from");
    println!("New initilizzer is: {s}");


    println!("");
    println!("*****************************************************************");
    println!("String Slinicing");

    let slice = &s[0..5];
    println!("Slice from 0 to 4 is {slice}");

    println!("Iteration in string");
    for ch in "Hello".chars(){
        println!("{ch}");
    }

    println!("Bytes representation");
    for ch in "Hello".bytes(){
        println!("{ch}");
    }

    println!("");
    println!("*****************************************************************");
    println!("Hash Maps");

    let mut color_map = HashMap::new();
    // Inserting in map 
    color_map.insert(String::from("Blue"), 255);
    color_map.insert(String::from("Green"), 128);

    let color_code = color_map.get(&String::from("Blue")).copied().unwrap_or(299);
    println!("Feteche blue value is: {color_code}");

    println!("Iterating in map");
    for (key, value) in &color_map{
        println!("{key}: {value}");
    }

    println!("Adding key only if it does not exists");
    color_map.entry(String::from("Hello")).or_insert(292);
    color_map.entry(String::from("Blue")).or_insert(10);

    for (key, value) in &color_map{
        println!("{key}: {value}");
    }
    
    println!("Modifying value based on previous value");
    for (_i, value) in &mut color_map{
        *value *= 2;
    }
    
    println!("{:?}", color_map);


}
