use loader;

fn load_file(filename: &str) {
    println!("Loading {}", filename);
    let records = loader::load_records(filename);
    println!("Size of {}: {}", filename, records.len());
}

// The main function to parse commands entered in the CLI
pub fn parse_command(line: &str) {
    let word_vec: Vec<&str> = line.split(' ').collect();

    match word_vec[0] {
        "Load" => load_file(word_vec[1]),
        _ => println!("Unknown Command."),
    }    
}