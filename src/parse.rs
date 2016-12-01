extern crate marc;

use loader;
use query;

use std::collections::HashMap;

fn load_file(filename: String, variable_name: String, records_storage: &mut HashMap<String, Vec<marc::Record<'static>>>) {
    println!("Loading {}", filename);
    let records = loader::load_records(&filename);
    records_storage.insert(variable_name.clone(), records.clone());
}

fn print_size(filename: String, records_storage: &mut HashMap<String, Vec<marc::Record<'static>>>) {
    match records_storage.get(filename.as_str()) {
        Some(record_vec) => println!("Size of {}: {}", filename, record_vec.len()),
        None => println!("No entry {} found...", filename),
    }
}

fn query_field(field_tag: String, variable_name: String, records_storage: &mut HashMap<String, Vec<marc::Record<'static>>>) {
    query::query_field(field_tag, variable_name, records_storage);
}

fn query_subfield(identifier: String, field_tag: String, variable_name: String, records_storage: &mut HashMap<String, Vec<marc::Record<'static>>>) {
    query::query_subfield(identifier, field_tag, variable_name, records_storage);
}

// The main function to parse commands entered in the CLI
pub fn parse_command(line: &str, records_storage: &mut HashMap<String, Vec<marc::Record<'static>>>) {
    let word_vec: Vec<&str> = line.split(' ').collect();

    match word_vec[0] {
        "Load" => load_file(word_vec[1].to_string(), word_vec[2].to_string(), records_storage), // Load in a .mrc file and save it as the 2nd argument in records_storage
        "Size" => print_size(word_vec[1].to_string(), records_storage), // Print the size of a record vector. 
        "Field" => query_field(word_vec[1].to_string(), word_vec[2].to_string(), records_storage), // Print out the field specified from the variable specified
        "Subfield" => query_subfield(word_vec[1].to_string(), word_vec[2].to_string(), word_vec[3].to_string(), records_storage), // Print out the subfield specified from the variable specified
        _ => println!("Unknown Command."),
    }    
}