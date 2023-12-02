use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::IResult;
use nom::bytes::streaming::take_while_m_n;
use nom::character::is_digit;
use nom::character::streaming::{alpha0, anychar};
use nom::combinator::map_res;

fn main() {
    // File input.txt must exist in the current path
    let mut sum = 0u32;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
                let tens = find_first_digit(&line).expect("Tens");
                let rev_str = line.chars().rev().collect::<String>();
                let ones = find_first_digit(&rev_str).expect("Ones");
                
                sum += tens.1 * 10;
                sum += ones.1;
                print!("This lines value was: {}{}\t", tens.1, ones.1);
                println!("The sum is: {}", sum);
            }
        }
    }
    println!("The total sum is: {}", sum)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_first_digit(input: &str) -> IResult<&str, u32> {
    let (input, res) = alpha0(input)?;
    println!("Non digit res: {}", res);
    map_res(
        anychar,
        get_u32
    )(input)
}

fn get_u32(input: char) -> Result<u32, &'static str> {
    input.to_digit(10).ok_or("Parsing Error")
}