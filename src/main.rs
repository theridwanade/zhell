use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let (cmd, args) = match input.split_once(" ") {
            Some((c, a)) => (c, a.trim()),
            None => (input, ""),
        };

        if cmd == "exit" {
            break;
        }
    }
}
