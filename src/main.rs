extern crate rustyline;
extern crate marc;

mod parse;
mod loader;

use parse::parse_command;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::collections::HashMap;

fn main() {
    let mut records_storage: HashMap<String, Vec<marc::Record<'static>>> = HashMap::new();

    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history(".marc_cli_history.txt") {
        println!("No previous history found.");
    }
    loop {
        let readline = rl.readline("MARC>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                parse_command(&line, &mut records_storage);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C. Quitting...");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D. Quitting...");
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