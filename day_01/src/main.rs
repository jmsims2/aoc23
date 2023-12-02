use std::fs;
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = fs::read_to_string(Path::new("./day_01/src/codes.txt")).expect("file exists");

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
    let format_result = result.to_string();
    println!("part 1 result is {format_result}");
}

fn part_two() {
    let file = fs::read_to_string(Path::new("./day_01/src/codes.txt")).expect("file exists");

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
    let format_result = result.to_string();
    println!("part 2 result is {format_result}");
}
