use loader;

// The main function to parse commands entered in the CLI
pub fn parse_command(line: &str) {

    //For now, it will try loading a .mrc file only.
    println!("Loading {}", line);
    let records = loader::load_records(line);
    println!("Size of {}: {}", line, records.len());
}