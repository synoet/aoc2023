use std::fs::read_to_string;

fn part_one() {
    let content = read_to_string("input.txt").unwrap();
    let lines = content.lines();

    let seeds: Vec<usize> = lines.clone().collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let break_pos = lines
        .clone()
        .enumerate()
        .filter(|(_i, s)| s.is_empty())
        .map(|(i, _s)| i)
        .collect::<Vec<usize>>();

    let mut groups: Vec<Vec<&str>> = vec![];

    for i in 0..break_pos.len() {
        let mut end = lines.clone().collect::<Vec<_>>().len();
        if i < break_pos.len() - 1 {
            end = break_pos[i + 1];
        }

        let group = &lines.clone().collect::<Vec<&str>>()[break_pos[i] + 2..end];
        groups.push(group.to_vec());
    }

    let mut positions: Vec<usize> = vec![];
    for seed in seeds.iter() {
        let mut position = *seed;
        for i in 0..groups.len() {
            for sg in groups[i].iter() {
                let nums = sg
                    .split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let range = nums[2];
                if position <= nums[1] + range  && position >= nums[1] {
                    position = position - nums[1] + nums[0];
                    break;
                }
            }
        }
        positions.push(position);
    }

    positions.sort();
    dbg!(positions[0]);
}

fn main() {
    part_one();
}
