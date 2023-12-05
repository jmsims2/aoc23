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
    let file = fs::read_to_string(Path::new("./day_03/src/parts.txt")).expect("file exists");
    let part_one_result = part_one(&file);
    let part_two_result = part_two(&file);
    println!("day 3 part 1 result is {part_one_result}");
    println!("day 3 part 2 result is {part_two_result}");
}

fn part_one(file: &String) -> i32 {
    let rows: Vec<&str> = file.split("\n").collect();
    let row_length = rows[0].len();
    let row_count = rows.len();
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
                    while end < chars.len() && chars[end].is_numeric() {
                        value.push(chars[end]);
                        end += 1;
                    }
                    row_position = end;
                    found_numbers.push(Number {
                        row: row_index,
                        start: start,
                        end: end - 1,
                        value: value.parse::<i32>().unwrap(),
                    })
                } else {
                    row_position += 1;
                }
            }
            found_numbers
        })
        .collect();

    let mut result: Vec<i32> = Vec::new();
    for number in &numbers {
        let symbol_search_matrix = get_symbol_search_matrix(number, row_length, row_count);
        for (row, position) in symbol_search_matrix {
            for symbol in &symbols {
                if symbol.row == row && symbol.position == position {
                    result.push(number.value);
                    break;
                }
            }
        }
    }

    result.into_iter().sum()
}

fn part_two(file: &String) -> i32 {
    let rows: Vec<&str> = file.split("\n").collect();
    let row_length = rows[0].len();
    let row_count = rows.len();
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
                    while end < chars.len() && chars[end].is_numeric() {
                        value.push(chars[end]);
                        end += 1;
                    }
                    row_position = end;
                    found_numbers.push(Number {
                        row: row_index,
                        start: start,
                        end: end - 1,
                        value: value.parse::<i32>().unwrap(),
                    })
                } else {
                    row_position += 1;
                }
            }
            found_numbers
        })
        .collect();

    let mut result: Vec<i32> = Vec::new();
    for symbol in &symbols {
        let mut nums: Vec<i32> = Vec::new();
        let number_search_matrix = get_number_search_matrix(symbol, row_length, row_count);
        for number in &numbers {
            let num_range = number.start..=number.end;
            for (row, position) in number_search_matrix.to_owned() {
                if row == number.row && num_range.contains(&position) {
                    nums.push(number.value);
                    break;
                }
            }
        }

        if nums.len() == 2 {
            result.push(nums[0] * nums[1]);
        }
    }

    result.into_iter().sum()
}

fn get_symbol_search_matrix(
    number: &Number,
    row_length: usize,
    row_count: usize,
) -> Vec<(usize, usize)> {
    let mut search_matrix: Vec<(usize, usize)> = Vec::new();
    let col_start = if number.start == 0 {
        0
    } else {
        number.start - 1
    };
    let col_end: usize = if number.end == row_length - 1 {
        number.end
    } else {
        number.end + 1
    };
    let row_start = if number.row == 0 {
        number.row
    } else {
        number.row - 1
    };
    let row_end = if number.row == row_count - 1 {
        number.row
    } else {
        number.row + 1
    };

    for row in row_start..=row_end {
        for col in col_start..=col_end {
            search_matrix.push((row, col));
        }
    }

    search_matrix
}

fn get_number_search_matrix(
    symbol: &Symbol,
    row_length: usize,
    row_count: usize,
) -> Vec<(usize, usize)> {
    let mut search_matrix: Vec<(usize, usize)> = Vec::new();
    let col_start = if symbol.position == 0 {
        0
    } else {
        symbol.position - 1
    };
    let col_end: usize = if symbol.position == row_length - 1 {
        symbol.position
    } else {
        symbol.position + 1
    };
    let row_start = if symbol.row == 0 {
        symbol.row
    } else {
        symbol.row - 1
    };
    let row_end = if symbol.row == row_count - 1 {
        symbol.row
    } else {
        symbol.row + 1
    };

    for row in row_start..=row_end {
        for col in col_start..=col_end {
            search_matrix.push((row, col));
        }
    }

    search_matrix
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

    #[test]
    fn day_3_part_two_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_03/src/parts_test.txt")).expect("file exists");
        assert_eq!(part_two(&file), 467835)
    }
}
