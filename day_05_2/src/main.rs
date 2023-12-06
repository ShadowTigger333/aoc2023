#![feature(iter_array_chunks)]

use std::ops::Range;
use itertools::Itertools;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Mapping {
    change: i64,
    start: i64,
    end: i64,
}

impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Change: {}, Start: {}, End: {})",
            self.change, self.start, self.end
        )
    }
}

type Mappings = Vec<Vec<Mapping>>;
fn main() {
    println!(
        "The code is in the test while on my work computer (tests will run but not the executable)"
    );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut seeds = Vec::new();
        let mut seed_to_soil: Vec<Mapping> = Vec::new();
        let mut soil_to_fertilizer: Vec<Mapping> = Vec::new();
        let mut fertiliizer_to_water: Vec<Mapping> = Vec::new();
        let mut water_to_light: Vec<Mapping> = Vec::new();
        let mut light_to_temperature: Vec<Mapping> = Vec::new();
        let mut temperature_to_humidity: Vec<Mapping> = Vec::new();
        let mut humidity_to_location: Vec<Mapping> = Vec::new();

        if let Ok(lines) = read_lines("./input.txt") {
            // Consumes the iterator, returns an (Optional) String
            for (index, line) in lines.enumerate() {
                if let Ok(line) = line {
                    // println!("The line is: {}", line);
                    if index == 0 {
                        seeds = line[6..]
                            .split_whitespace()
                            .filter_map(|item: &str| item.parse::<i64>().ok())
                            .array_chunks::<2>()
                            .collect::<Vec<[i64; 2]>>();
                    } else if (3..49).contains(&index) {
                        let items: Vec<i64> = line
                            .split_whitespace()
                            .map(|item: &str| item.parse::<i64>().unwrap())
                            .collect();
                        let new_mapping = Mapping {
                            change: items[0] - items[1],
                            start: items[1],
                            end: items[1] + items[2],
                        };
                        seed_to_soil.push(new_mapping);
                    } else if (51..93).contains(&index) {
                        let items: Vec<i64> = line
                            .split_whitespace()
                            .map(|item: &str| item.parse::<i64>().unwrap())
                            .collect();
                        let new_mapping = Mapping {
                            change: items[0] - items[1],
                            start: items[1],
                            end: items[1] + items[2],
                        };
                        soil_to_fertilizer.push(new_mapping);
                    } else if (95..139).contains(&index) {
                        let items: Vec<i64> = line
                            .split_whitespace()
                            .map(|item: &str| item.parse::<i64>().unwrap())
                            .collect();
                        let new_mapping = Mapping {
                            change: items[0] - items[1],
                            start: items[1],
                            end: items[1] + items[2],
                        };
                        fertiliizer_to_water.push(new_mapping);
                    } else if (141..171).contains(&index) {
                        let items: Vec<i64> = line
                            .split_whitespace()
                            .map(|item: &str| item.parse::<i64>().unwrap())
                            .collect();
                        let new_mapping = Mapping {
                            change: items[0] - items[1],
                            start: items[1],
                            end: items[1] + items[2],
                        };
                        water_to_light.push(new_mapping);
                    } else if (174..196).contains(&index) {
                        let items: Vec<i64> = line
                            .split_whitespace()
                            .map(|item: &str| item.parse::<i64>().unwrap())
                            .collect();
                        let new_mapping = Mapping {
                            change: items[0] - items[1],
                            start: items[1],
                            end: items[1] + items[2],
                        };
                        light_to_temperature.push(new_mapping);
                    } else if (198..213).contains(&index) {
                        let items: Vec<i64> = line
                            .split_whitespace()
                            .map(|item: &str| item.parse::<i64>().unwrap())
                            .collect();
                        let new_mapping = Mapping {
                            change: items[0] - items[1],
                            start: items[1],
                            end: items[1] + items[2],
                        };
                        temperature_to_humidity.push(new_mapping);
                    } else if (215..242).contains(&index) {
                        let items: Vec<i64> = line
                            .split_whitespace()
                            .map(|item: &str| item.parse::<i64>().unwrap())
                            .collect();
                        let new_mapping = Mapping {
                            change: items[0] - items[1],
                            start: items[1],
                            end: items[1] + items[2],
                        };
                        humidity_to_location.push(new_mapping);
                        // } else {
                        //     println!("Skipping");
                    }
                }
            }
            let mappings = vec![
                seed_to_soil,
                soil_to_fertilizer,
                fertiliizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            ];
            let mapped_seeds:Vec<Range<i64>> = seeds.iter().map(|seed| (seed[0]..seed[0]+seed[1])).collect();

            let transformed_seeds = &mappings.iter().fold(mapped_seeds, |seeds, mappings| {
                seeds
                    .iter()
                    .flat_map(|seed_range| {
                        let mut mapped_ranges = Vec::new();

                        let final_end = mappings
                            .into_iter()
                            .filter(|mapping| {
                                mapping.end > seed_range.start && mapping.start < seed_range.end
                            })
                            .sorted_by_key(|mapping| mapping.start)
                            .fold(seed_range.start, |current, mapping| {
                                if current < mapping.start {
                                    mapped_ranges.push(current..mapping.start);
                                }

                                let common_start = current.max(mapping.start);
                                let common_end = seed_range.end.min(mapping.end);
                                mapped_ranges.push(
                                    common_start + mapping.change..
                                    common_end + mapping.change
                                );

                                common_end
                            });
                        if final_end < seed_range.end {
                            mapped_ranges.push(final_end..seed_range.end);
                        }

                        mapped_ranges
                    })
                    .collect()
            });
        println!("The min location is {}", transformed_seeds.iter().min_by_key(|range| range.start).unwrap().start)
        }
        assert!(false); //Get print to work
    }
}
