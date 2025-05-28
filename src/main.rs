

fn main() {
    let mut str = String :: from("Ashutosh");
    str.push_str(" Sharma");
    let length= get_length(str);
    print!("{}",length);
    println!("Hello, world!");
}

fn get_length(str: String)-> usize {
    let length =str.len();
    return length;
}
