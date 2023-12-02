use std::fs;
use std::path::Path;

fn main() {
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
    println!("result is {format_result}");
}
