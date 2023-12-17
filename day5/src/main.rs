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
                if position <= nums[1] + range && position >= nums[1] {
                    position = position - nums[1] + nums[0];
                    break;
                }
            }
        }
        positions.push(position);
    }

    positions.sort();
}

// this is fucking slow
fn part_two() {
    let content = read_to_string("input.txt").unwrap();
    let lines = content.lines();

    let seed_ranges = lines.clone().collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .iter()
        .as_slice()
        .chunks(2)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            (chunk[0], chunk[1])
        })
        .collect::<Vec<(usize, usize)>>();

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

    groups.reverse();

    let locations = groups[0].clone();
    let location_ranges = locations.iter().map(|sg| {
        let nums = sg
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        (nums[1], nums[1] + nums[2])
    });

    let remaining_groups = groups[1..].to_vec();

    let mut locations_with_path = location_ranges
        .filter_map(|location| {
            let mut current_range = location.clone();
            for group in remaining_groups.iter() {
                let ranges = group
                    .iter()
                    .map(|sg| {
                        sg.split(" ")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>();

                if let Some(found_range) = ranges.iter().find(|r| {
                    let range = r[2];
                    let start = r[1];
                    let end = start + range;

                    std::cmp::max(start, location.0) <= std::cmp::min(end, location.1)
                }) {
                    current_range = (found_range[0], found_range[1] + found_range[2]);
                } else {
                    return None;
                }
            }
            let maybe_seed = seed_ranges.iter().find(|(start, end)| {
                std::cmp::max(start, &current_range.0) <= std::cmp::min(end, &current_range.1)
            });

            if let Some(seed) = maybe_seed {
                return Some((location, seed.clone()));
            }
            None
        })
        .collect::<Vec<((usize, usize), (usize, usize))>>();

    let mut ans = locations.iter().map(|l| {
        l.split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    })
    .filter_map(|l| {
        if let Some((location, seed)) = locations_with_path.iter().find(|(location, _seed)| {
            std::cmp::max(location.1, l[1]) <= std::cmp::min(location.1, l[1] + l[2]) 
        }) {
            dbg!(l);
            dbg!(seed);
            // dbg!(seed.0 < l[1]); 
            return Some(100);
        }
        None

    })
    .collect::<Vec<usize>>();


    println!("{:?}", ans);


}

fn main() {
    part_two();
}
