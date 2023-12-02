use std::fs;

fn partone() -> i32 {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines().collect::<Vec<&str>>();

    return lines.iter().filter_map(|line| {
        let split = line.split_at(line.find(":").unwrap());
        let id = split.0.replace("Game ", "");

        let grabs = split.1.split(";").collect::<Vec<&str>>();

        let valid = grabs.iter().all(|grab| {
            let mut num_reds = 0;
            let mut num_blues = 0;
            let mut num_greens = 0;

            let grab_split = grab.split(",").collect::<Vec<&str>>();
            grab_split.iter().for_each(|g| {
                let val = g.replace(":", "");
                match [
                    val.contains("red"),
                    val.contains("blue"),
                    val.contains("green"),
                ] {
                    [true, false, false] => {
                        let add = val.replace("red", "").trim().parse::<i32>().unwrap();
                        num_reds += add;
                    },
                    [false, true, false] => {
                        let add = val.replace("blue", "").trim().parse::<i32>().unwrap();
                        num_blues += add;

                    },
                    [false, false, true] => {
                        let add = val.replace("green", "").trim().parse::<i32>().unwrap();
                        num_greens += add;
                    },
                    _ => (),
                }

            });

            num_reds <= 12 && num_blues <= 14 && num_greens <= 13
        });

        if valid {
            Some(id.parse::<i32>().unwrap())
        } else {
            None
        }
    }).sum()
}

fn main() {
    println!("partone {}", partone());
}
