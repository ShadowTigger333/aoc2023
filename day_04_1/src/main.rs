use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                println!("The line is: {}", line);
                let line_vec: Vec<&str> = line.split(|pat| pat == ':' || pat == '|').collect();
                let winning_nums: Vec<&str> = line_vec[1].split_whitespace().collect();
                let game_nums: Vec<&str> = line_vec[2].split_whitespace().collect();
                let mut running_total = 0;
                for item in game_nums {
                    if winning_nums.contains(&item){
                        println!("{} has winning number {}", line_vec[0], item);
                        if running_total == 0 {
                            running_total += 1;
                        } else {
                            running_total *= 2;
                        }
                    }
                }
                println!("Adding {} for {}", running_total, line);
                sum += running_total;
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
