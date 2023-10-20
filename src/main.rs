extern crate csv;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::time::Instant;
use Haochong_Week_8_mini::calculate_median;

fn main() -> Result<(), Box<dyn Error>> {
    // Record the start time
    let start_time = Instant::now();
    // Load the CSV file
    let csv_file = "25ktopomapseriesindex.csv"; // Update with your CSV file path
    let file = File::open(csv_file)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);

    let mut shape_leng_values: Vec<f64> = Vec::new();
    let mut shape_area_values: Vec<f64> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let shape_leng: f64 = record[2].parse()?; // Change index to match your data
        let shape_area: f64 = record[3].parse()?; // Change index to match your data
        shape_leng_values.push(shape_leng);
        shape_area_values.push(shape_area);
    }

    // Calculate and print the medians
    let shape_leng_median = calculate_median(&shape_leng_values);
    let shape_area_median = calculate_median(&shape_area_values);

    println!("Shape_Leng Median: {}", shape_leng_median);
    println!("Shape_Area Median: {}", shape_area_median);

    let end_time = Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time);

    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

