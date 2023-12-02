use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, alpha1};
use nom::sequence::{delimited, tuple};

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            if let Ok(line) = line {
                let (input, id) = find_id(&line).expect("Id");
                let new_input = input.replace(',', ";").replace(" ", "");
                new_input.split(";").for_each(|item| {
                    let (_, (ammount, key)) = get_item(item).expect("Item");
                    match key {
                        "green" => {if ammount > max_green {
                            max_green = ammount;
                        }},
                        "blue"=> {if ammount > max_blue {
                            max_blue = ammount;
                        }},
                        "red"=> {if ammount > max_red {
                            max_red = ammount;
                        }},
                        _ => panic!("This is bad")
                    }
                });
                if max_blue <= 14 && max_green <= 13 && max_red <= 12 {
                    sum += id;
                    println!("ADDED: {}", line)
                } else {
                    println!("SKIPPED: {}", line)
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
fn find_id(input: &str) -> IResult<&str, i32> {
    delimited(tag("Game "), i32, tag(":"))(input)
}
fn get_item(input: &str) -> IResult<&str, (i32, &str)> {
    tuple((i32, alpha1))(input)
}