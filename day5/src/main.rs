use std::cmp::max;
use std::env;
use std::fs;

const DAY: &str = "day5";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let ingredient_str = read_lines(&test_file_path);
    let (mut fresh, ingredient_ids) = parse_ingredient_db(&ingredient_str);
    part1(&fresh, &ingredient_ids);
    part2(&mut fresh);

    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let ingredient_str = read_lines(&file_path);
    let (mut fresh, ingredient_ids) = parse_ingredient_db(&ingredient_str);
    part1(&fresh, &ingredient_ids);
    part2(&mut fresh);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    return fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
}

fn parse_range(s: &str) -> Option<(u64, u64)> {
    let (start_str, end_str) = s.split_once('-')?;
    Some((start_str.parse().ok()?, end_str.parse().ok()?))
}

fn parse_ingredient_db(ingredient_str: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut fresh: Vec<(u64, u64)> = Vec::new();
    let mut ingredient_ids: Vec<u64> = Vec::new();

    let mut parse_ranges: bool = true;
    for cur in ingredient_str.lines() {
        if parse_ranges {
            if cur.is_empty() {
                parse_ranges = false;
                continue;
            }
            let (from, to) = parse_range(cur).unwrap();
            fresh.push((from, to));
        } else {
            ingredient_ids.push(cur.parse::<u64>().unwrap())
        }
    }
    return (fresh, ingredient_ids);
}

fn part1(fresh_db: &Vec<(u64, u64)>, ingredient_ids: &Vec<u64>) {
    let mut fresh_cnt = 0;
    for cur in ingredient_ids.iter() {
        for range in fresh_db.iter() {
            if range.0 <= *cur && *cur <= range.1 {
                fresh_cnt += 1;
                break;
            }
        }
    }
    println!("part1: {}", fresh_cnt);
}

fn part2(fresh_db: &mut Vec<(u64, u64)>) {
    // let mut sorted: Vec<_> = fresh_db.into_iter().collect();
    fresh_db.sort_by_key(|&(first, _)| first);

    let mut max_ptr: u64 = 0;
    let mut fresh_cnt: u64 = 0;
    for cur in fresh_db.iter() {
        if cur.1 < max_ptr {
            continue;
        }
        let start = max(cur.0, max_ptr);

        fresh_cnt += (cur.1 - start) + 1;
        max_ptr = cur.1 + 1;
    }

    println!("part2: {}", fresh_cnt);
}
