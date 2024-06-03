use std::io;
use rand::Rng;

const CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=<>?";
fn main() {
    let password: String;
    let mut pass_length: u8 = input_length();

    while pass_length < 8 {
        println!("The value you used wasn't valid or greater than 7");
        pass_length = input_length();
    }

    // println!("\nThe value of the length you chosen is {}", pass_length);

    password = gen_pass(pass_length);

    println!("The PASSWORD RESULT is: {}", password);
}

fn gen_pass(password_length: u8) -> String {
    let mut generated_password: String = String::new();
    let mut n: u8 = 0;
    let mut rng = rand::thread_rng();

    while n < password_length {
        let index: usize = rng.gen_range(0..CHARS.len() + 1);
        // println!("{index}");
        generated_password.push(CHARS.chars().nth(index).unwrap());
        n += 1;
    }

    return generated_password;
}

fn input_length() -> u8 {
    println!("\nEnter the length you want for the password (equal or greater than 8)");

    let mut length_str = String::new();

    io::stdin()
        .read_line(&mut length_str)
        .expect("Enter valid data");

    if length_str.is_empty() {
        return 0;
    }

    let pass_length: u8 = length_str
        .trim()
        .parse()
        .expect("An error ocurred parsing to u8");
    return pass_length;
}
