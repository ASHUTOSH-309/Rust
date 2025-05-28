
fn main(){
    references();
}


fn immutable_example() {
    let mut s = String::from("hello");
    let r = &mut s;        // r is the mutable reference for the mutable string
    r.push_str(" world"); // ✅ Works
    println!("R value is : {}", r);    // r will change the original string s 
    println!("S value is : {}", s);    // r will change the original string s 
}


fn mutable_example() {
    let s = String::from("hello");
    let r = &s;  // Immutable string , immutable reference , can't be changed, constant length 
  // r.push_str(" world"); // ❌ Error: cannot borrow `*r` as mutable
    println!("R value is : {}", r);    // r will change the original string s 
    println!("S value is : {}", s);  
}


// write  a function that  will mutate a string using an immutable reference


fn references(){
    let mut str: String= String :: from("Ashutosh");
    let mutable_reference = &mut str;

    str="String :: from("Ashutosh");

    println!("Before Manipulating through reference {}", str);
    mutable_reference.push_str("Sharma"); // Inserting to the original varibale using a mutable reference 
    println!("After Manipulating by reference {}", str);
}





