use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut copies: [u64; 220] = [1; 220];
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (index, line) in lines.enumerate() {
            if let Ok(line) = line {
                // println!("The line is: {}", line);
                let line_vec: Vec<&str> = line.split(|pat| pat == ':' || pat == '|').collect();
                let potential_winners: Vec<&str> = line_vec[1].split_whitespace().collect();
                let game_nums: Vec<&str> = line_vec[2].split_whitespace().collect();
                let mut matches = 0;
                for item in game_nums {
                    if potential_winners.contains(&item) {
                        // println!("{} has winning number {}", line_vec[0], item);
                        matches += 1;
                    }
                }
                for id in index+1..index+matches+1 {
                    copies[id] += copies[index];
                }
            }
        }
    }
    println!("The copies sum is: {}", copies.iter().sum::<u64>());
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
