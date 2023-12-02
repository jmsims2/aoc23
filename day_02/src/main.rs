use std::fs;
use std::path::Path;

enum Block {
    Red(i32),
    Green(i32),
    Blue(i32),
}
fn main() {
    let file = fs::read_to_string(Path::new("./day_02/src/games.txt")).expect("file exists");
    let games = file.split("\n");
    let result: i32 = games
        .into_iter()
        .map(|g| {
            let (game, results) = g.split_at(g.find(":").unwrap());
            println!("game {game}");
            println!("results {results}");
            let (_, game_id) = game.split_at(game.find(" ").unwrap());
            println!("game_id {game_id}");
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
        .sum();

    println!("day 2 result is {result}");
}

fn get_block(string: &str) -> Block {
    println!("argument {string}");
    let (num, color) = string.split_at(string.find(" ").unwrap());
    match color.trim() {
        "red" => Block::Red(num.parse::<i32>().unwrap()),
        "green" => Block::Green(num.parse::<i32>().unwrap()),
        "blue" => Block::Blue(num.parse::<i32>().unwrap()),
        wtf => {
            println!("what are you {wtf}");
            panic!("wtf");
        }
    }
}
