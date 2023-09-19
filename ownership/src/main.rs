fn main() {
    let s = String::from("hello");

    {
        println!("parent s is {}", s);// s is present in the parent scope and hence it's available inside it's child

        let t = String::from("world");
        let s = String::from("Goodbye"); // this will push inside the original "s" string
        println!("s inside child is {}", s); // new s inside the child
        println!("t inside child is {}", t);
    } // the scope for t ends here

    println!("s in parent is {}", s);
    // println!("{}", t); Will throw error because "t" is out of scope

    let s1 = s;
    println!("s1 is {}", s1);
    // println!("s is {}", s); // Will throw error since s is already moved in the previous step 
    
    // Note: 
    //      let x = 5;
    //      let y = x;
    //      Copies value that is stored inside x to y because the 5 is treated as i32 and 
    //      hence it's size is predefined but incase of string the size is not defined and 
    //      hence only the pointer to the first letter is copied

    let s2 = s1.clone(); // clone is the property of string to actually create a duplicate
    println!("s1 is {} and s2 is {}", s1, s2);


    let s2 = take_and_give_ownership(s2);   // here we first take the ownership of s2 and then we
                                            // create another variable with the same name to give back the modified string
    println!("Modified s2 is {}", s2);
}


fn take_and_give_ownership(s:String) -> String{
    let mut modified_string = s;
    modified_string.push_str(",this is something new!"); // This function does not return anything
    modified_string
}