use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Mapping {
    change: i64,
    start: i64,
    end: i64
}

fn main() {
    let mut seeds = Vec::new();
    let mut seed_to_soil:Vec<Mapping> = Vec::new();
    let mut soil_to_fertilizer:Vec<Mapping> = Vec::new();
    let mut fertiliizer_to_water:Vec<Mapping> = Vec::new();
    let mut water_to_light:Vec<Mapping> = Vec::new();
    let mut light_to_temperature:Vec<Mapping> = Vec::new();
    let mut temperature_to_humidity:Vec<Mapping> = Vec::new();
    let mut humidity_to_location:Vec<Mapping> = Vec::new();
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (index, line) in lines.enumerate() {
            if let Ok(line) = line {
                println!("The line is: {}", line);
                if index == 0 {
                    seeds = line[6..].split_whitespace().map(|seed| seed.parse::<i64>().unwrap()).collect();
                    dbg!(&seeds);
                }
                if line.is_empty() {
                    println!("Skipping");
                }
            }
        }
    }
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
