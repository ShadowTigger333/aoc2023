use itertools::{Itertools, PeekingNext};
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
            // Consumes the iterator, returns an (Optional) String
            let mut vecs: Vec<Vec<i64>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    println!("The line is {}", line);
                    vecs.push(
                        line.split_whitespace()
                            .map(|item| item.parse::<i64>().unwrap())
                            .collect_vec(),
                    );
                }
            }

            let mut sum: i64 = 0;
            let mut nums: Vec<i64> = Vec::new();
            let mut differences: Vec<i64> = Vec::new();
            let mut edge: Vec<i64> = Vec::new();
            for vec_nums in vecs {
                nums.clear();
                nums = vec_nums;
                nums.reverse();
                edge.clear();

                let mut found_solution = false;
                while !found_solution {
                    differences.clear();
                    for (left, right) in nums.iter().tuple_windows() {
                        differences.push(right - left);
                    }
                    edge.push(*nums.last().unwrap());

                    if differences.iter().all(|&item| item == 0) {
                        println!("Adding {} to sum", edge.iter().sum::<i64>());
                        sum += edge.iter().sum::<i64>();
                        found_solution = true;
                    }
                    std::mem::swap(&mut nums, &mut differences);
                }
            }
            println!("The sum is: {}", sum);
        }
        assert!(false); //Get print to work
    }
}
