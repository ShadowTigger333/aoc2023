use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: u64 = 0;
    let mut symbols: Vec<(usize, i32, char)> = Vec::new();
    let mut numbers: Vec<(usize, [i32; 2], u64)> = Vec::new();
    let mut num_lines = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_index, line) in lines.enumerate() {
            num_lines += 1;
            if let Ok(line) = line {
                println!("The line is: {}", line);
                let mut prev_digit = false;
                for (column_index, character) in line.chars().enumerate() {
                    if character == '.' {
                        prev_digit = false;
                    } else if !character.is_digit(10) {
                        prev_digit = false;
                        symbols.push((line_index, column_index.try_into().unwrap(), character));
                    } else {
                        if !prev_digit {
                            prev_digit = true;
                            numbers.push((
                                line_index,
                                [column_index.try_into().unwrap(), column_index.try_into().unwrap()],
                                character.to_digit(10).unwrap().into(),
                            ));
                        } else {
                            let new_item = numbers.pop().unwrap();
                            let mut new_number = new_item.2.to_string().to_owned();
                            new_number.push_str(&character.to_string());
                            numbers.push((
                                line_index,
                                [new_item.1[0], column_index.try_into().unwrap()],
                                u64::from_str_radix(&new_number, 10).unwrap(),
                            ));
                        }
                    }
                }
            }
        }
        for item in numbers {
            if item.0 > 0 && item.0 < num_lines - 1 {
                for symbol in &symbols {
                    if symbol.0 < item.0 - 1 || symbol.0 > item.0 + 1 {
                        continue;
                    } else if symbol.1 >= item.1[0]-1 && symbol.1 <= item.1[1]+1 {
                        print!("Adding {} to {}...", item.2, sum);
                        sum += item.2;
                        println!("New total: {}", sum);
                    }
                }
            } else if item.0 == 0 {
                for symbol in &symbols {
                    if symbol.0 > item.0 + 1 {
                        continue;
                    } else if symbol.1 >= item.1[0]-1 && symbol.1 <= item.1[1]+1 {
                        print!("Adding {} to {}...", item.2, sum);
                        sum += item.2;
                        println!("New total: {}", sum);
                    }
                }
            } else if item.0 < num_lines {
                for symbol in &symbols {
                    if symbol.0 < item.0 - 1 {
                        continue;
                    } else if symbol.1 >= item.1[0]-1 && symbol.1 <= item.1[1]+1 {
                        print!("Adding {} to {}...", item.2, sum);
                        sum += item.2;
                        println!("New total: {}", sum);
                    }
                }
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
