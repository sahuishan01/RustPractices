// custom message type
enum MType{
    Sent,
    Received,
}

// enum for definitions inside of a Message (useful when combined with structs)
enum Message{
    Type (MType), // for type of message
    Text (String), // for message text
    Color (i32, i32, i32), // for display color of the text (arbitrary value)
}

// implementing display for each type of Message Property using
impl Message{
    fn call(&self) -> String {
        match self { // first match to check what type of property we are trying to access
            Message::Type(m_type) => {
                format!("Type is: {}", match m_type{ // checking the type of message for MType
                    MType::Sent => String::from("Sent"),
                    MType::Received => String::from("Received")
                })
            }   // display if the enum type is Message::Type
            Message::Text(text) => {
                format!("{}", text)
            }   // display if the enum type is Message::Text
            Message::Color(r,g,b) => {
                format!("RGB is {{{}, {}, {}}}", r,g,b)
            }   // display if the enum type is Message::Color
        }
    }
}

fn main() {
    println!("CUSTOM ENUM");
    let m = Message::Text(String::from("hello"));
    let c = Message::Color(255, 255,30);
    let t = Message::Type(MType::Sent);
    println!("{}", m.call());
    println!("{}", c.call());
    println!("{}", t.call());

    println!();
    println!("************************************************");
    println!();

    println!("OPTIONS");
    // Options are alternatives for None(null), we can use them and assign their values as None,
    // which will block us from accessing a null value

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // options provide a number of methods to obtain values from itself.
    // Note: Option<i32> can not be added with i32 as they are not of same type

    // Some common methods of extracting values from Option are as follows

    println!("some_number is {}", some_number.unwrap()); // will throw an error if the value is None
    println!("some_char is {} ", some_char.unwrap_or('h')); // if value is none, it will
                                                                   // display the value obtained
                                                                   // from 'or'
    println!("absent_number is {}", absent_number.unwrap_or_default()); // if value is None,
                                                                        // it returns a default
                                                                        // value which is 0 for i32

    println!("Check some_number exists? {}", some_number.is_some());
    println!("Check absent_number doesn't exists? {}", absent_number.is_none());


    println!();
    println!("************************************************");
    println!();

    println!("USING OPTION WITH MATCH");
    let value:Option<i32> = Some(10);
    let value2: Option<i32> = None;
    println!("value is {}", add_one(value).unwrap());
    println!("value2 is {}", add_one(value2).unwrap());

    println!();
    println!("************************************************");
    println!();

    println!("HANDLING ONLY USEFUL CASES WITH MATCH");
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {} with match case.", max),
        _ => (),
    } // the _ will return nothing for any case other than Somme

    // The following can also be achieved by using 'if let'
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max); // we can use this method if we need to
                                                             // check for only one instance of an
                                                             // enum
    }
    else {
        println!("Max is not defined")
    }

}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => Some(1), // Will return default value as 1
        Some(val) => Some(val + 1)
    }
}
