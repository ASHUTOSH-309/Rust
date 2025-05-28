

fn references(){
    let mut str: String= String :: from("Ashutosh");
    let mutable_reference = &mut str;

    // mutable borrow locks the original variable 

    println!("Before Manipulating through reference {}",mutable_reference);
    mutable_reference.push_str("Sharma"); // Inserting to the original varibale using a mutable reference 
    println!("After Manipulating by reference {}", mutable_reference);
}






