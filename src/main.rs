#[macro_use]
extern crate lazy_static;

mod command;

use command::Command;
use std::io::stdin;

fn on_command(command: &Command) {
    match command {
        Command::Join(id) => println!("Joining room {}", id),
    }
}

fn main() {
    loop {
        let mut line = String::new();

        if let Err(error) = stdin().read_line(&mut line) {
            eprintln!("{}", error);
            continue;
        }

        line = String::from(line.trim());

        match Command::new(&line) {
            Some(command) => match &command {
                Ok(command) => on_command(command),
                Err(error) => eprintln!("{}", error),
            },
            None => println!("Sent message: {}", line),
        };
    }
}
