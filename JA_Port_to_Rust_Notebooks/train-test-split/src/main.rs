// use crate::utils;
use polars::prelude::*;
// use smartcore::linalg::basic::matrix::DenseMatrix;
// use smartcore::model_selection::train_test_split;

fn main() {
    let df = CsvReader::from_path(
        "/Users/squeeko/AI-for-CyberSecurity/JA_Port_to_Rust_Notebooks/data/ddos_dataset.csv",
    )
    .unwrap()
    .finish()
    .unwrap();

    println!("{:?}", df.shape());
    println!("{:?}", df.get(0));
}
