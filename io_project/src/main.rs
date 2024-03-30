use std::io;
use std::env;
use std::fs;

fn parse_arguments(args: &[String]) -> (&str, String, &str){
    let mut back_trace = "0";
    let mut file_path = String::from("");
    let mut search_string = "";
    if args.len() > 3 {
        back_trace = &args[1];
        file_path = args[2].clone();
        search_string = &args[3];
    }
    else if args.len() > 2 {
        back_trace = &args[1];
        file_path = args[2].clone();
    }
    else if args.len() > 1 {
        back_trace = &args[1];
    }
    (back_trace, file_path, search_string)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (back_trace, mut file_path, search_string) = parse_arguments(&args);
    env::set_var("RUST_BACKTRACE", back_trace);
    // println!("Current dir is: {:?}", env::current_dir());
    println!("search string is: {search_string}");
    loop {
        match file_path.trim(){
            "" => {},
            "z" => {
                    println!("Exiting...");
                    break;
            }
            _ => {
                match fs::read_to_string(file_path.trim()){
                    Ok(content) => println!("Text is:\n{content}"),
                    _ => println!("Could not find the file"), 
                }
            }
        }
        file_path.clear();
        println!("Enter a file name to read file contents or enter 'z' to exit");
        io::stdin().read_line(&mut file_path).expect("failed to read line");
    }
}
