extern crate rustyline;

mod parse;
mod loader;

use parse::parse_command;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history(".marc_cli_history.txt") {
        println!("No previous history found.");
    }
    loop {
        let readline = rl.readline("MARC>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                parse_command(&line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(".marc_cli_history.txt").unwrap();
}