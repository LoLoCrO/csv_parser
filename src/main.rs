use std::error::Error;
use std::fs::File;
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data.csv";
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);

    let mut sum = 0;

    for result in reader.records() {
        let record = result?;
        let value: i32 = record[0].parse()?;
        sum += value;
    }

    println!("Sum: {}", sum);

    Ok(())
}