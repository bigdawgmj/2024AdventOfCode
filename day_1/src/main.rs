use csv::ReaderBuilder;
use std::path::{Path};
use std::{
    env,
    fs::read_to_string,
    ffi::OsString,
    error::Error,
};

fn get_path() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected a file path, but none given")),
        Some(data_path) => Ok(data_path),
    }
}

fn process_csv(data_path: OsString) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(&data_path);
    // let file = File::open(file_path)?;

    let file_content = read_to_string(file_path)?;

    let normalized_content = file_content.replace("   ",",");

    let file = std::io::Cursor::new(normalized_content);

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    // let mut historian1: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
    // let mut historian2: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
    let mut historian1: Vec<i32> = Vec::new();
    let mut historian2: Vec<i32> = Vec::new();

    for result in reader.records() {
        let record = result?;
        // println!("Record: {:?}\n", record);
        historian1.push(record.get(0).unwrap().parse::<i32>()?);
        historian2.push(record.get(1).unwrap().parse::<i32>()?);
    }
    
    // sort the two lists
    historian1.sort();
    historian2.sort();
    
    // compare each value
    let mut differences = Vec::new();
    for i in 0..historian1.len() {
        differences.push((historian1[i] - historian2[i]).abs());
    }
    let sum: i32 = differences.iter().sum();
    println!("Total Distance: {}", sum);

    let mut similarity_score = 0;
    for i in 0..historian1.len() {
        similarity_score = similarity_score + historian1[i] * (historian2.iter().filter(|&n| *n == historian1[i]).count() as i32);
    }
    println!("Similarity Score: {}", similarity_score);

    Ok(()) // Return success
}

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");
    // take two lists (same length)
    let data_path = get_path()?;

    process_csv(data_path)?;

    Ok(())
}
