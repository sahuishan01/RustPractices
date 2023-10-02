fn main() {

    println!(" BASIC SCOPIGN");
    let s = String::from("hello");

    {
        println!("parent s is \"{}\"", s);// s is present in the parent scope and hence it's available inside it's child

        let t = String::from("world");
        let s = String::from("Goodbye"); // this will push inside the original "s" string
        println!("s inside child is \"{}\"", s); // new s inside the child
        println!("t inside child is \"{}\"", t);
    } // the scope for t ends here

    println!("s in parent is \"{}\"", s);
    // println!("{}", t); Will throw error because "t" is out of scope
    println!();
    println!("********************************************************");
    println!();

    println!("BORROWING: ");
    let s1 = s;
    println!("s1 is \"{}\"", s1);
    // println!("s is {}", s); // Will throw error since s is already moved at line 16
    println!();
    println!("********************************************************");

    println!();
    println!("TAKING OWNERSHIP ANG GIVING IT BACK");
    // Note:
    //      let x = 5;
    //      let y = x;
    //      Copies value that is stored inside x to y because the 5 is treated as i32 and 
    //      hence it's size is predefined but in case of string the size is not defined and
    //      hence only the pointer to the first letter is copied

    let s2 = s1.clone(); // clone is the property of string to actually create a duplicate
    println!("s1 is \"{}\" and s2 is \"{}\"", s1, s2);


    let mut s2 = take_and_give_ownership(s2);   // here we first take the ownership of s2 and
                                                    // then we create another variable with the same
                                                    // name to give back the modified string
    println!("Modified s2 is \"{}\"", s2);

    println!();
    println!("****************************************************");
    println!();

    println!("REFERENCE USAGE WITH STRING MODIFICATION:");
    let size = reference_length(&s2);
    println!("s2 size is \"{}\"", size);
    reference_modification(&mut s2);

    println!();
    println!("****************************************************");
    println!();

    println!("SLICES");
    refrence_slice(&s2[4..10]);

}


fn take_and_give_ownership(s:String) -> String{
    let mut modified_string = s;
    modified_string.push_str(",this is something new!"); // This function does not return anything
    modified_string
}

fn reference_length(s: &String) -> usize {
    println!("s2 in reference_length is \"{}\"", s);
    s.len()
}

fn reference_modification(s: &mut String) {
    s.push_str(", modified the reference");
    println!("modified string is \"{}\"", s);
}

fn refrence_slice(s: &str){
    println!("Reference slice from index 4 to 10 is \"{}\"", s);
}