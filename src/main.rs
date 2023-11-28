use std::io::Write;

mod ext;
use ext::rust::do_rust;

fn main() {
    let extensions = vec![do_rust];

    let mut input_string = String::new();

    'shell_loop: loop {
        // Print the prompt
        print!("$ ");
        std::io::stdout().flush().unwrap();

        // Read the input
        input_string.clear();
        std::io::stdin().read_line(&mut input_string).unwrap();
        input_string = input_string.trim().to_string();

        // Ignore any text after #
        if input_string.contains("#") {
            input_string = input_string.split("#").collect::<Vec<&str>>()[0].to_string();
        }

        // Run the input
        let input_vec: Vec<&str> = input_string.split(" ").collect();

        // If the input is empty, continue the loop.
        if input_vec.len() == 0 || input_vec[0] == "" {
            continue 'shell_loop;
        }

        // Check for built-in commands
        if input_vec[0] == "exit" {
            break 'shell_loop;
        }

        // Check if binary exists in path
        if which::which(input_vec[0]).is_ok() {
            let mut child = std::process::Command::new(input_vec[0])
                .args(&input_vec[1..])
                .spawn()
                .unwrap();

            child.wait().unwrap();
            continue 'shell_loop;
        }

        // Run "magic" extensions
        for extension in &extensions {
            if extension(&input_vec) {
                continue 'shell_loop;
            }
        }

        // If we get here, the command was not found.
        println!("Command not found: {}", input_string);
    }
}
