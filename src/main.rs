fn main() {
    let chars: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=<>?";
    let mut password: String; // `password` es un `String` mutable
    
    println!("Enter the length you want for the password\n");    
    
    let mut length_text = String::new();
    std::io::stdin().read_line(&mut length_text).expect("Enter valid data");
    let pass_length: u8 = length_text.trim().parse().expect("An error ocurred parsing to u8");
    
    println!("\nThe value of the length you chosen is {}", length_text);
}
