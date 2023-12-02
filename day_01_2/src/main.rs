use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use aho_corasick::AhoCorasick;

fn main() {
    // File input.txt must exist in the current path
    let mut sum = 0u64;
 
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let patterns = &[
                    "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
                    "eight", "8", "nine", "9",
                ];
                let ac = AhoCorasick::new(patterns).unwrap();
                let matches: Vec<usize> = ac
                .find_overlapping_iter(&line)
                .map(|mat| mat.pattern().as_usize())
                .collect();
                
                let tens = match matches.first()  {
                    Some(0) => 1,
                    Some(1) => 1,
                    Some(2) => 2,
                    Some(3) => 2,
                    Some(4) => 3,
                    Some(5) => 3,
                    Some(6) => 4,
                    Some(7) => 4,
                    Some(8) => 5,
                    Some(9) => 5,
                    Some(10) => 6,
                    Some(11) => 6,
                    Some(12) => 7,
                    Some(13) => 7,
                    Some(14) => 8,
                    Some(15) => 8,
                    Some(16) => 9,
                    Some(17) => 9,
                    _ => 999
                };
                let ones = match matches.last()  {
                    Some(0) => 1,
                    Some(1) => 1,
                    Some(2) => 2,
                    Some(3) => 2,
                    Some(4) => 3,
                    Some(5) => 3,
                    Some(6) => 4,
                    Some(7) => 4,
                    Some(8) => 5,
                    Some(9) => 5,
                    Some(10) => 6,
                    Some(11) => 6,
                    Some(12) => 7,
                    Some(13) => 7,
                    Some(14) => 8,
                    Some(15) => 8,
                    Some(16) => 9,
                    Some(17) => 9,
                    _ => 999
                };
                sum +=  tens* 10;
                sum += ones;
                println!("This lines value was: {}{}\t{}", tens, ones, line);
                // println!("The sum is: {}", sum);
            }
        }
    }
    println!("The total sum is: {}", sum)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
