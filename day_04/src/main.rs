use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let file = fs::read_to_string(Path::new("./day_04/src/scratch.txt")).expect("file exists");
    let part_one_result = part_one(&file);
    //let part_two_result = part_two(&file);
    println!("day 4 part 1 result is {part_one_result}");
    //println!("day 4 part 2 result is {part_two_result}");
}

fn part_one(file: &String) -> i32 {
    let cards = file.split("\n");
    cards
        .map(|c| {
            let mut card = c.split_terminator(&[':', '|']).collect::<Vec<&str>>();
            let card_numbers = card
                .pop()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            let winning_numbers = card
                .pop()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            let mut points = 0;
            for winning_number in winning_numbers {
                for card_number in card_numbers.to_owned() {
                    if card_number == winning_number {
                        if points > 0 {
                            points = points * 2;
                        } else {
                            points = 1;
                        }
                        break;
                    }
                }
            }
            points
        })
        .collect::<Vec<i32>>()
        .into_iter()
        .sum()
}

fn part_two(file: &String) -> i32 {
    let cards = file.split("\n");
    cards
        .map(|c| {
            let mut card = c.split_terminator(&[':', '|']).collect::<Vec<&str>>();
            let card_numbers = card
                .pop()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            let winning_numbers = card
                .pop()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            let mut points = 0;
            for winning_number in winning_numbers {
                for card_number in card_numbers.to_owned() {
                    if card_number == winning_number {
                        points += 1;
                        break;
                    }
                }
            }
            points
        })
        .collect::<Vec<i32>>()
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (index, points)| {
            if points > 0 {
                *acc.entry((index + 1) as i32).or_insert(0) += 1;
                let start = index as i32 + 1;
                let end = start + points;
                println!("card {} start {} end {}", index + 1, start, end);
                for card in start..end {
                    *acc.entry(card + 1).or_insert(0) += 1;
                }
            }
            println!("HashMap {:?}", acc);
            acc
        })
        .into_iter()
        .map(|(_, cards)| cards)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_4_part_one_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_04/src/scratch_test.txt")).expect("file exists");
        assert_eq!(part_one(&file), 13)
    }

    #[test]
    fn day_4_part_two_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_04/src/scratch_test.txt")).expect("file exists");
        assert_eq!(part_two(&file), 30)
    }
}
