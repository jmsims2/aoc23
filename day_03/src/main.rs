use std::fs;
use std::path::Path;

#[derive(Debug)]

struct Number {
    row: usize,
    start: usize,
    end: usize,
    value: i32,
}

#[derive(Debug)]
struct Symbol {
    row: usize,
    position: usize,
}

fn main() {
    let file = fs::read_to_string(Path::new("./day_03/src/parts_test.txt")).expect("file exists");
    let result = part_one(&file);
}

fn part_one(file: &String) -> i32 {
    let rows: Vec<&str> = file.split("\n").collect();
    let symbols: Vec<Symbol> = rows
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(index, value)| {
                    if !value.is_numeric() && value != '.' {
                        return Some(Symbol {
                            row: row_index,
                            position: index.clone(),
                        });
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<Symbol>>()
        })
        .collect();

    let numbers: Vec<Number> = rows
        .into_iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            let mut found_numbers = Vec::new();
            let mut row_position = 0;
            let chars = row.chars().collect::<Vec<char>>();
            while row_position < chars.len() {
                if chars[row_position].is_numeric() {
                    let mut value = "".to_owned();
                    let start = row_position;
                    let mut end = row_position;
                    while chars[end].is_numeric() {
                        value.push(chars[end]);
                        end += 1;
                    }
                    row_position = end;
                    found_numbers.push(Number {
                        row: row_index,
                        start: start,
                        end: end,
                        value: value.parse::<i32>().unwrap(),
                    })
                } else {
                    row_position += 1;
                }
            }
            found_numbers
        })
        .collect();

    println!("symbols {:?}", symbols);
    println!("numbers {:?}", numbers);
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_3_part_one_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_03/src/parts_test.txt")).expect("file exists");
        assert_eq!(part_one(&file), 4361)
    }
}
