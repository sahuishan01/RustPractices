use std::io;  // for getting the input from user
use rand::Rng; // for generating random number
use std::cmp::Ordering;
fn main() {
    println!("Welcome to Guess The Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Enter a number that you want to guess");
        let mut guess = String::new(); // initializing an empty string for storing user input
        io::stdin().read_line(&mut guess).expect("Failed to read line"); //reading user input as string, and throwing any error if it occurs while getting input
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue
            }
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!!!!!"); break;},
        }
    }

}
