extern crate marc;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::error::Error;

const RECORD_TERMINATOR: u8 = 0x1D;

// Load marc records from a file into a vector.
pub fn load_records(filename: &String) -> Vec<marc::Record<'static>> {
    let mut result_vector: Vec<marc::Record> = Vec::new();
    let mut buffer = Vec::new();

    let file_path = filename.clone();

    let path = Path::new(file_path.as_str());
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut file = BufReader::new(file);
    while file.read_until(RECORD_TERMINATOR, &mut buffer).unwrap() != 0 {
        match marc::Record::from_vec(buffer.clone()) {
            Err(_) => (),
            Ok(record) => result_vector.push(record.clone()),
        }

        buffer.clear();
    }

    return result_vector;
}
