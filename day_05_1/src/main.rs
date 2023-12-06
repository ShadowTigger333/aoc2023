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
fn main() {
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
                        .map(|seed| seed.parse::<i64>().unwrap())
                        .collect();
                    dbg!(&seeds);
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
        let mut seed_transform: Vec<i64> = Vec::new();
        for seed in seeds {
            let soil = transform(seed, &seed_to_soil);
            println!("Seed {} transforming to soil {}", seed, soil);

            let fertilizer = transform(soil, &soil_to_fertilizer);
            println!("Soil {} transforming to fertilizer {}", soil, fertilizer);

            let water = transform(fertilizer, &fertiliizer_to_water);
            println!("Fertilizer {} transforming to water {}", fertilizer, water);

            let light = transform(water, &water_to_light);
            println!("Water {} transforming to light {}", water, light);

            let temperature = transform(light, &light_to_temperature);
            println!(
                "Light {} transforming to temperature {}",
                light, temperature
            );

            let humidity = transform(temperature, &temperature_to_humidity);
            println!("Transforming {} to {}", temperature, humidity);

            let location = transform(humidity, &humidity_to_location);
            println!("Transforming {} to {}", humidity, location);

            seed_transform.push(location);
        }
        seed_transform.sort();
        let min_location = seed_transform.first().unwrap();
        println!("The min location is {}", min_location);
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

fn transform(item: i64, mappings: &Vec<Mapping>) -> i64 {
    let mut transformed_item = item;
    for mapping in mappings.into_iter() {
        if (mapping.start..mapping.end).contains(&item) {
            transformed_item = item + mapping.change;
        }
    }
    transformed_item
}

#[cfg(test)]
mod tests {
    use crate::{transform, Mapping, read_lines};

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
                            .map(|seed| seed.parse::<i64>().unwrap())
                            .collect();
                        dbg!(&seeds);
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
            let mut seed_transform: Vec<i64> = Vec::new();
            for seed in seeds {
                let soil = transform(seed, &seed_to_soil);
                println!("Seed {} transforming to soil {}", seed, soil);
    
                let fertilizer = transform(soil, &soil_to_fertilizer);
                println!("Soil {} transforming to fertilizer {}", soil, fertilizer);
    
                let water = transform(fertilizer, &fertiliizer_to_water);
                println!("Fertilizer {} transforming to water {}", fertilizer, water);
    
                let light = transform(water, &water_to_light);
                println!("Water {} transforming to light {}", water, light);
    
                let temperature = transform(light, &light_to_temperature);
                println!(
                    "Light {} transforming to temperature {}",
                    light, temperature
                );
    
                let humidity = transform(temperature, &temperature_to_humidity);
                println!("Transforming {} to {}", temperature, humidity);
    
                let location = transform(humidity, &humidity_to_location);
                println!("Transforming {} to {}", humidity, location);
    
                seed_transform.push(location);
            }
            seed_transform.sort();
            let min_location = seed_transform.first().unwrap();
            println!("The min location is {}", min_location);
            assert!(*min_location == 35);
        }
    }
}