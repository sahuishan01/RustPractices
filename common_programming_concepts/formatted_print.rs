use std::fmt;

// custom structs can not be printed directly, they have to inherit from Debug as follows 
struct Person<'a>{ //<'a> is for making it lifetime parameter
    name: &'a str,
    age: u8,
}

// custom display trait for custom struct
impl fmt::Display for Person<'_>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} is {} years old", self.name, self.age)
    }

}
// custom debug trait for custom struct
impl fmt::Debug for Person<'_>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "The name is {} \nThe age is {}", self.name, self.age)
    }
}

// custom binary trait for custom struct
impl fmt::Binary for Person<'_>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "The name is {} and the age in binary is {:b}", self.name, self.age)
    }
}  


// for lists 
struct List<T>(Vec<T>);

// implement custom display
impl <T: fmt::Display> fmt::Display for List<T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        // creating reference to self
        let vec = &self.0;
        write!(f, "[ ")?;
        for (count, v) in vec.iter().enumerate() {
            // adding comma for each element except the first one
            if count != 0 { write!(f, ", ")? };
            write!(f, "{}", v)?;
        }
        write!(f, " ]")
    }
}
fn main(){

    // single argument
    println!("{} days", 30);

    // positional arguments
    println!("{0} {1} {0}", "one", "two");

    // named arguments
    println!("{subject} {verb} {object}", subject="Computer Science", verb="completed", object="the course");

    // different type of formatting
    println!("Binary: {:b} Hex: {:x} Octal: {:o} Decimal {}", 10, 10, 10, 10);

    // right-justified
    println!("{number:>8}", number=10); // total space taken by the number = 8 (from left)

    // pad with a number
    println!("{number:0>width$}", number=10, width=8);

    let name = "Peter";
    let age = 27;

    let peter = Person{name, age};
    println!("{}", peter); // display
    println!("{:?}", peter); // debug
    println!("{:b}", peter); // binary

    // list 
    let list = List::<i32>(vec![1, 2, 3, 4]);
    println!("{}", list);
}