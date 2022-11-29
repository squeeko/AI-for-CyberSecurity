use polars::prelude::*;

fn main() {
    let df = CsvReader::from_path("/Users/squeeko/AI-for-CyberSecurity/JA_Port_to_Rust_Notebooks/data/north_korea_missile_test_database.csv")
        .unwrap()
        .finish()
        .unwrap();

    println!("{}", df);
}
