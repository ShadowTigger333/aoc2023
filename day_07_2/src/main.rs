use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    FiveOfAKind([Card; 5]),
    FourOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    ThreeOfAKind([Card; 5]),
    TwoPair([Card; 5]),
    OnePair([Card; 5]),
    HighCard([Card; 5]),
}

fn determine_hand(input: &str) -> HandType {
    let mut cards: [Card; 5] = [Card::Ace; 5];
    for (index, character) in input.char_indices() {
        cards[index] = determine_card(character);
    }

    let unique_cards = cards.iter().cloned().unique().collect_vec();
    let mut card_frequency = Vec::<usize>::new();
    let mut joker_count = 0;
    for unique_card in unique_cards.iter() {
        let count = cards.iter().filter(|&elem| elem == unique_card).count();
        if *unique_card != Card::Joker {
            card_frequency.push(count);
        } else {
            joker_count = count;
        }
    }
    // dbg!(&unique_cards);
    card_frequency.sort();
    // dbg!(&card_frequency);
    // dbg!(joker_count);
    let high_count = card_frequency.pop().unwrap_or(0);
    card_frequency.push(high_count + joker_count);
    // dbg!(&card_frequency);


    return match card_frequency[..] {
        [5] => HandType::FiveOfAKind(cards),
        [1, 4] => HandType::FourOfAKind(cards),
        [2, 3] => HandType::FullHouse(cards),
        [1, 1, 3] => HandType::ThreeOfAKind(cards),
        [1, 2, 2] => HandType::TwoPair(cards),
        [1, 1, 1, 2] => HandType::OnePair(cards),
        _ => HandType::HighCard(cards),
    };
}

fn determine_card(input: char) -> Card {
    return match input {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        'J' => Card::Joker,
        _ => Card::Ace,
    };
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
            let mut hands: BTreeMap<HandType, u32> = BTreeMap::new();
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(line) = line {
                    println!("The line is {}", line);
                    let mut line_iter = line.split_whitespace().into_iter();
                    let this_hand = determine_hand(line_iter.next().unwrap());
                    let this_bid = line_iter.next().unwrap().parse::<u32>().unwrap();
                    hands.insert(this_hand, this_bid);
                }
            }
            let mut sum: u32 = 0;
            let len_hands = hands.len();
            for (index, hand) in hands.into_iter().enumerate() {
                let bid_multiplier:u32 = u32::try_from(len_hands - index).unwrap();
                sum += hand.1 * bid_multiplier
            }
            println!("The sum is {}", sum);
            assert!(false); //Get print to work
        }
    }
}
