mod tokenizers;
use tokenizers::latin1::encode;
use std::env::args;


fn main() {
    // Read the input text from the user
    let input_string = args()
    .skip(1)
    .reduce(|ch1, ch2| -> String {ch1 + &ch2})
    .expect("Error - no arguments found."); 

    let tokens = encode(input_string.as_str(), 120);
    println!("{:?}", tokens);
}
