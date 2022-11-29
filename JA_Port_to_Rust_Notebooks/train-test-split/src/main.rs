extern crate csv;

use std::io::prelude::*;
use std::process;

fn main() -> Result<(), csv::Error> {
    let mut rdr = csv::Reader::from_path("/Users/squeeko/AI-for-CyberSecurity/JA_Port_to_Rust_Notebooks/data/north_korea_missile_test_database.csv")?;
    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("Error reading CSV from file: {}", err);
                process::exit(1);
            }
        }
    }
    Ok(())
}
