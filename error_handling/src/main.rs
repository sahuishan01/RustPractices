use std::fs::File;
use std::io::ErrorKind;

fn manual_panic(){
   panic!("crash and burn using manual panice") ;
}

fn main() {
    println!("Hello, world!");
    // manual_panic();
    // let v = vec![1, 2, 4];
    // v[10];

    // Error handling with Result
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Erro handling with unwrap_or_else

    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
