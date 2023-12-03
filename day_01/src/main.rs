use std::fs;
use std::path::Path;

fn main() {
    let file = fs::read_to_string(Path::new("./day_01/src/codes.txt")).expect("file exists");
    let part_one_result = part_one(&file);
    let part_two_result = part_two(&file);
    println!("day 1 part 1 result is {part_one_result}");
    println!("day 1 part 2 result is {part_two_result}");
}

fn part_one(file: &String) -> i32 {
    let codes = file.split("\n").collect::<Vec<_>>();

    let mut result = 0;
    for code in codes {
        let mut digits = Vec::new();
        let values = code.chars().into_iter();
        for value in values {
            if value.is_digit(10) {
                digits.push(value);
            }
        }
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let mut string = "".to_owned();
        string.push(*first);
        string.push(*last);
        result = result + string.parse::<i32>().unwrap();
    }
    result
}

fn part_two(file: &String) -> i32 {
    let codes = file.split("\n").collect::<Vec<_>>();

    let mut result = 0;
    for code in codes {
        let mut digits = Vec::new();
        let transformed_code = code
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let values = transformed_code.chars().into_iter();
        for value in values {
            if value.is_digit(10) {
                digits.push(value);
            }
        }
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let mut string = "".to_owned();
        string.push(*first);
        string.push(*last);
        result = result + string.parse::<i32>().unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_01/src/codes_test_1.txt")).expect("file exists");
        assert_eq!(part_one(&file), 142);
    }

    #[test]
    fn part_two_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_01/src/codes_test_2.txt")).expect("file exists");
        assert_eq!(part_two(&file), 281);
    }
}
