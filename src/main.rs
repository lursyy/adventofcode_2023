use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input/1.txt") {
        let sum: u32 = lines.flatten().map(get_calibration_value).sum();
        println!("Sum: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(line: String) -> u32 {
    let ascii_digits = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        // .collect::<String>();
        .collect::<Vec<char>>();

    let first_digit = *ascii_digits.first().unwrap();
    let last_digit = *ascii_digits.last().unwrap_or(&first_digit);

    let filtered_digits = [first_digit, last_digit].iter().collect::<String>();

    filtered_digits.parse::<u32>().unwrap()
}
