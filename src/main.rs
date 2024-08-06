use std::{env::{self},io::{self, Write}, path::{self, Path, PathBuf}, process::Command};

use home::{home_dir};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        command_handler(input);
    }
}

fn command_handler(input: String) -> io::Result<()>{
    let mut parts = input.split_whitespace().filter(|s| !s.is_empty());
    if let Some(command) = parts.next() {
        match command {
            "cd" => {
                if let Some(dir) = parts.next() {
                    let dir = path::absolute(dir)?;
                    if PathBuf::from(&dir).is_dir() {
                        env::set_current_dir(&dir)?;
                    } else {
                        eprintln!("Blast it! The directory '{}' be missing!", dir.to_str().unwrap());
                    }
                } else {
                    env::set_current_dir(home_dir().unwrap());
                }
            }
            command => {
                run_command(command, parts)
            }
        }
    }
    Ok(())
}

fn run_command<'a>(command: &'a str, parts: impl Iterator<Item=&'a str>) {
    let mut child = Command::new(command)
        .args(parts)
        .spawn()
        .unwrap();
    child.wait();
}
