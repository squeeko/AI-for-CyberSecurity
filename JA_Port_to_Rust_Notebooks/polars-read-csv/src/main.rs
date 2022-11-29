use polars::io::mmap::MmapBytesReader;
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let file = std::fs::File::open("/Users/squeeko/AI-for-CyberSecurity/JA_Port_to_Rust_Notebooks/data/north_korea_missile_test_database.csv")
        .unwrap();
    let file = Box::new(file) as Box<dyn MmapBytesReader>;
    let df = CsvReader::new(file)
        .with_delimiter(b'|')
        .has_header(true)
        .with_chunk_size(10)
        .batched(None)
        .unwrap();

    Ok(())
}
