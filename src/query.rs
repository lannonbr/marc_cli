extern crate marc;

use marc::*;

use std::collections::HashMap;

pub fn query_field(field_tag: String, variable_name: String, records_storage: &mut HashMap<String, Vec<marc::Record<'static>>>) {
    match records_storage.get(variable_name.as_str()) {
        Some(record_vec) => {
            for record in record_vec.iter() {
                for field in record.field(Tag::from(field_tag.as_str())).iter() {
                    println!("{}", field.get_data::<str>());
                }
            }
        },
        None => println!("No variable {} found", variable_name),
    }
}