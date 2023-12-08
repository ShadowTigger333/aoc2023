use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
            let mut nodes: HashMap<String, (String, String)> = HashMap::new();
            let mut found_way = false;
            let mut directions: String = "".to_string();
            let re = Regex::new(r"^(\w{3}).{4}(\w{3}).{2}(\w{3}).$").unwrap();
            // Consumes the iterator, returns an (Optional) String
            for (index, line) in lines.into_iter().enumerate() {
                if let Ok(line) = line {
                    // println!("The line is {}", line);
                    if index == 0 {
                        directions += &line;
                        // println!("Directions: {}", directions);
                    } else if line.is_empty() {
                        continue;
                    } else {
                        let capture = re.captures(&line).unwrap();
                        let key = capture.get(1).unwrap().as_str().to_string();
                        let v1 = capture.get(2).unwrap().as_str().to_string();
                        let v2 = capture.get(3).unwrap().as_str().to_string();
                        // println!("Key: {} -> Value: ({}, {})", key, v1, v2);
                        nodes.insert(key, (v1, v2));
                    }
                }
            }
            let mut steps: u32 = 0;
            let mut current_node = nodes.get("AAA").unwrap();
            while found_way == false {
                let current_index = steps.rem_euclid(u32::try_from(directions.len()).unwrap());
                let current_direction = directions
                    .chars()
                    .nth(current_index.try_into().unwrap())
                    .unwrap();
                // println!(
                //     "\nThe current step is: {} and the current direction is {}",
                //     steps + 1,
                //     current_direction
                // );
                // println!(
                //     "The current node is: ({}, {})",
                //     current_node.0, current_node.1
                // );
                let next_step = match current_direction {
                    'L' => &current_node.0,
                    'R' => &current_node.1,
                    _ => panic!("This shouldn't happen"),
                };
                // println!("The next key is {}", next_step);
                if next_step == "ZZZ" {
                    found_way = true;
                } else {
                    current_node = nodes.get(next_step).unwrap();
                }
                steps += 1;
            }
            println!("The total number of steps is {}", steps);
            assert!(false); //Get print to work
        }
    }
}
