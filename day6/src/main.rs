use std::fs::read_to_string;

#[derive(Debug)]
struct Race {
    distance: i64,
    time: i64,
}

fn parse_line_with_spaces(line: &str) -> Vec<i64> {
    line.split(":").collect::<Vec<&str>>()[1]
        .trim_start()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_line_wihout_spaces(line: &str) -> Vec<i64> {
    vec![line.split(":").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<i64>().unwrap()]
}

fn parse_line(line: &str, ignore_spaces: bool) -> Vec<i64> {
    if !ignore_spaces {
        parse_line_with_spaces(line)
    } else {
        parse_line_wihout_spaces(line)
    }
}

fn parse_document(input: &str, ignore_spaces: bool) -> Vec<Race> {
    let lines = input.lines().collect::<Vec<&str>>();
    let times = parse_line(lines[0], ignore_spaces);
    let distances = parse_line(lines[1], ignore_spaces);

    times
        .iter()
        .enumerate()
        .map(|(i, time)| Race {
            distance: distances[i],
            time: *time,
        })
        .collect::<Vec<Race>>()
}

fn process_races(races: &Vec<Race>) -> i64 {
    // charge for 2 seconds travel 4 ml in 2 seconds
    let mut all_counts: Vec<i64> = vec![];
    for race in races {
        let mut count: i64 = 0;
        let mut current_speed = 0;
        for i in 1..race.time - 1 {
            if i >= race.time {
                break;
            }
            current_speed += 1;

            let distance = current_speed * (race.time - i);
            if distance > race.distance {
                count += 1;
                println!("charged for {} seconds, will travel {}m in remaining {} seconds", i, distance, (race.time - i));
            }
        }

        all_counts.push(count);
    }

    all_counts.iter().product()
}

fn part_one() {
    let content = read_to_string("./input.txt").unwrap();
    let races = parse_document(&content, false);
    let answer = process_races(&races);
    println!("answer: {}", answer);
}

fn part_two() {
    let content = read_to_string("./input.txt").unwrap();
    let races = parse_document(&content, true);
    let answer = process_races(&races);
    println!("answer: {}", answer);

}

fn main() {
    part_two();
}
