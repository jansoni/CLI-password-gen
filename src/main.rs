extern crate rand;

use rand::Rng;
use std::env;

const NUMS_AND_CHARS: [char; 94]  = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_','-','+','=','{','[', ']','}','|','\\',':',';','"','\'','<','>', ',', '.', '?', '/' 
];

fn generate_random_password(len: u32) -> String {
    let mut ans = String::new();

    for _ in 0..len {
        let random_number: usize = rand::thread_rng().gen_range(0..93);
        ans += &NUMS_AND_CHARS[random_number].to_string();
    }
    return ans;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pass_len: u32 = args[1].parse().expect("Give number");

    let password: String = generate_random_password(pass_len);

    println!("{}", password);
}
