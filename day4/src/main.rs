use std::str::Lines;

#[derive(Debug, Clone)]
struct Game {
    index: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    depth: u32,
}

fn part_one() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let games = lines.enumerate().map(|(i, line)| {
        let split = line.split(":").collect::<Vec<&str>>()[1];
        let nums = split
            .split("|")
            .map(|x| {
                x.split(" ")
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<_>>();

        return Game {
            index: i,
            winning_numbers: nums[0]
                .iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
            numbers: nums[1]
                .iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
            depth: 0,
        };
    });

    let score = games
        .map(|game| {
            let mut curr_score = 0;
            for num in game.numbers.iter() {
                if game.winning_numbers.contains(num) {
                    if curr_score == 0 {
                        curr_score = 1;
                    } else {
                        curr_score *= 2;
                    }
                }
            }
            return curr_score;
        })
        .sum::<u32>();

    println!("Score: {}", score);
}

fn part_two() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let original_games = lines
        .enumerate()
        .map(|(i, line)| {
            let split = line.split(":").collect::<Vec<&str>>()[1];
            let nums = split
                .split("|")
                .map(|x| {
                    x.split(" ")
                        .filter(|x| !x.is_empty())
                        .collect::<Vec<&str>>()
                })
                .collect::<Vec<_>>();

            return Game {
                index: i,
                winning_numbers: nums[0]
                    .iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                numbers: nums[1]
                    .iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                depth: 0,
            };
        })
        .collect::<Vec<Game>>();

    let mut games = original_games.clone();
    let mut i = 0;

    while i < games.len() {
        let game = &games[i];
        let mut curr_score = 0;
        for num in game.numbers.iter() {
            if game.winning_numbers.contains(num) {
                curr_score += 1;
            }
        }
        if curr_score > 0 && game.depth < 2 {
            let games_to_add = original_games.clone()[game.index + 1
                ..std::cmp::min(game.index + curr_score + 1, original_games.len()) as usize]
                .iter()
                .map(|game| game.clone())
                .map(|mut game| {
                    game.depth = game.depth + 1;
                    return game;
                })
                .collect::<Vec<Game>>();
            games.extend(games_to_add);
        }

        i += 1;
    }

    dbg!(games.len());
}

fn main() {
    // partone();
    part_two();
}
