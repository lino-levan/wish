use std::io::Write;

mod ext;
use ext::rust::do_rust;

fn main() {
    let mut input_string = String::new();

    while input_string != "exit" {
        // Print the prompt
        print!("$ ");
        std::io::stdout().flush().unwrap();

        // Read the input
        input_string.clear();
        std::io::stdin().read_line(&mut input_string).unwrap();
        input_string = input_string.trim().to_string();

        // Run the input
        let input_vec: Vec<&str> = input_string.split(" ").collect();
        do_rust(&input_vec.to_vec());
    }
}
