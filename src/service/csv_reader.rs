use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    read_file_with_skip_lines(file_path, 0)
}

pub fn read_file_with_skip_lines(
    file_path: &str,
    lines_amount_to_skip: usize,
) -> io::Result<Vec<String>> {
    let result = read_lines(file_path)?
        .skip(lines_amount_to_skip)
        .flatten()
        .collect();
    Ok(result)
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}