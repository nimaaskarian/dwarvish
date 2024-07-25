use std::{io::{self, Write}, process::Command};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let mut parts = input.split_whitespace();
        let command = parts.next();
        if let Some(command) = command {
            let mut child = Command::new(command)
                .args(parts)
                .spawn()
                .unwrap();
            child.wait();
        }
    }
}
