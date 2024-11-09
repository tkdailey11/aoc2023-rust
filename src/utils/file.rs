use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn read_lines_to_vec(file_path: String) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Use `map` to convert each line into a String and collect them into a vector
    let lines: Vec<String> = reader.lines()
        .collect::<Result<_, _>>()?;

    Ok(lines)
}