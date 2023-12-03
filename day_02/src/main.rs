use std::fs;
use std::path::Path;

enum Block {
    Red(i32),
    Green(i32),
    Blue(i32),
}
fn main() {
    let file = fs::read_to_string(Path::new("./day_02/src/games.txt")).expect("file exists");
    let part_one_result = part_one(&file);
    let part_two_result = part_two(&file);
    println!("day 1 part 1 result is {part_one_result}");
    println!("day 1 part 2 result is {part_two_result}");
}

fn part_one(file: &String) -> i32 {
    let games = file.split("\n");
    games
        .into_iter()
        .map(|g| {
            let (game, results) = g.split_at(g.find(":").unwrap());
            let (_, game_id) = game.split_at(game.find(" ").unwrap());
            let results_2 = &results
                .replace(";", ",")
                .replace(":", "")
                .trim()
                .to_string();
            let mut game_results = results_2
                .split(", ")
                .into_iter()
                .map(|sample| match get_block(sample) {
                    Block::Red(n) if n <= 12 => true,
                    Block::Green(n) if n <= 13 => true,
                    Block::Blue(n) if n <= 14 => true,
                    _ => false,
                });

            return if game_results.all(|r| r) {
                game_id.trim().parse::<i32>().unwrap()
            } else {
                0
            };
        })
        .sum()
}

fn part_two(file: &String) -> i32 {
    let games = file.split("\n");
    games
        .into_iter()
        .map(|g| {
            let (_game, results) = g.split_at(g.find(":").unwrap());
            let results_2 = &results
                .replace(";", ",")
                .replace(":", "")
                .trim()
                .to_string();
            let mut red = 1;
            let mut green = 1;
            let mut blue = 1;

            let game_results = results_2.split(", ").into_iter();

            for sample in game_results {
                match get_block(sample) {
                    Block::Red(n) if n > red => red = n,
                    Block::Green(n) if n > green => green = n,
                    Block::Blue(n) if n > blue => blue = n,
                    _ => (),
                }
            }

            return red * blue * green;
        })
        .sum()
}

fn get_block(string: &str) -> Block {
    let (num, color) = string.split_at(string.find(" ").unwrap());
    match color.trim() {
        "red" => Block::Red(num.parse::<i32>().unwrap()),
        "green" => Block::Green(num.parse::<i32>().unwrap()),
        "blue" => Block::Blue(num.parse::<i32>().unwrap()),
        _ => {
            panic!("wtf");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_part_one_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_02/src/games_test.txt")).expect("file exists");
        assert_eq!(part_one(&file), 8);
    }

    #[test]
    fn day_2_part_two_gets_the_right_value() {
        let file =
            fs::read_to_string(Path::new("./../day_02/src/games_test.txt")).expect("file exists");
        assert_eq!(part_two(&file), 2286);
    }
}
