use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Race {
    time: u64,
    distance: u64,
}

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
        if let Ok(lines) = read_lines("./input.txt") {
            let mut times: Vec<u64> = Vec::new();
            let mut distances: Vec<u64> = Vec::new();
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(line) = line {
                    println!("The line is {}", line);
                    if line.starts_with("Time:") {
                        times = line
                            .split(':')
                            .into_iter()
                            .last()
                            .unwrap()
                            .trim()
                            .split_whitespace()
                            .map(|value| value.parse::<u64>().unwrap())
                            .collect();
                    } else if line.starts_with("Distance:") {
                        distances = line
                            .split(":")
                            .into_iter()
                            .last()
                            .unwrap()
                            .trim()
                            .split_whitespace()
                            .map(|value| value.parse::<u64>().unwrap())
                            .collect();
                    }
                }
            }

            let _test_races = [
                Race {
                    time: 7,
                    distance: 9,
                },
                Race {
                    time: 15,
                    distance: 40,
                },
                Race {
                    time: 30,
                    distance: 200,
                },
            ];
            let _races = [
                Race {
                    time: *times.get(0).unwrap(),
                    distance: *distances.get(0).unwrap(),
                },
                Race {
                    time: *times.get(1).unwrap(),
                    distance: *distances.get(1).unwrap(),
                },
                Race {
                    time: *times.get(2).unwrap(),
                    distance: *distances.get(2).unwrap(),
                },
                Race {
                    time: *times.get(3).unwrap(),
                    distance: *distances.get(3).unwrap(),
                },
            ];

            let races_2 = [Race {
                time: 48876981,
                distance: 255128811171623,
            }];
            let mut sum = 1;
            for race in races_2 {
                let mut current_win_posibilities: u64 = 0;
                for time in 0..race.time.div_ceil(2) {
                    // for time in 0..race.time+1 {
                    let hold_time = time;
                    let remaining_time = race.time - hold_time;
                    let speed = hold_time;
                    let distance = speed * remaining_time;
                    if distance > race.distance {
                        current_win_posibilities += 1;
                        // println!("WINNER #{}", current_win_posibilities);
                        // println!("Holding down for {} ms leaving {} ms\nThe boat is traveling at {} mm/ms and will go {} mm", hold_time, remaining_time, speed, distance);
                    }
                }
                current_win_posibilities *= 2;
                if race.time.rem_euclid(2) == 0 {
                    current_win_posibilities += 1;
                }
                println!("\nRACE TRACKED\nThe race had time {} and distance {}\nThe number of win possibilities is: {}",race.time, race.distance, current_win_posibilities);
                sum *= current_win_posibilities;
            }
            println!("\n\nThe winning number is {}", sum);
            assert!(false); //Get print to work
        }
    }
}
