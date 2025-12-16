use std::env;
use std::fs;

const DAY: &str = "day3";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let banks_str = read_lines(&test_file_path);
    let banks = parse_banks(&banks_str);

    part1(banks.clone());
    part2(banks.clone());

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let banks_str = read_lines(&file_path);
    let banks = parse_banks(&banks_str);

    part1(banks.clone());
    part2(banks.clone());
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    return fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
}

fn parse_banks(banks_str: &str) -> Vec<Vec<u32>> {
    let mut banks: Vec<Vec<u32>> = Vec::new();

    for cur in banks_str.lines() {
        let batteries: Vec<u32> = cur.chars().map(|i| i.to_digit(10).unwrap()).collect();
        banks.push(batteries);
    }
    return banks;
}

fn get_joltage_part_1(bank: &Vec<u32>) -> u32 {
    let len_bank = bank.len();
    let max_val = bank.iter().max().unwrap();
    let idx_first_max = bank.iter().position(|&x| x == *max_val).unwrap();

    if idx_first_max < len_bank - 1 {
        let rest = &bank[idx_first_max + 1..];
        let rest_max_val = rest.iter().max().unwrap();
        return max_val * 10 + rest_max_val;
    } else {
        let rest = &bank[..idx_first_max];
        let rest_max_val = rest.iter().max().unwrap();
        return *rest_max_val * 10 + max_val;
    }
}

fn part1(banks: Vec<Vec<u32>>) {
    let joltage: u32 = banks.iter().map(|cur| get_joltage_part_1(cur)).sum();
    println!("part1: {}", joltage);
}

fn get_joltage_part_2(bank: &Vec<u32>) -> u64 {
    let BANKS_REQUIRED: usize = 12;
    let mut cur_joltage: Vec<u32> = Vec::new();

    let mut rest: &[u32] = &bank;

    while cur_joltage.len() < BANKS_REQUIRED {
        let start_idx = get_starting_battery_idx(rest, BANKS_REQUIRED - cur_joltage.len());
        cur_joltage.push(rest[start_idx]);
        rest = &rest[start_idx + 1..];
    }
    let total = cur_joltage
        .iter()
        .fold(0u64, |acc, &digit| acc * 10 + digit as u64);
    return total;
}

fn get_starting_battery_idx(bank: &[u32], required_size: usize) -> usize {
    let len_bank: usize = bank.len();
    let mut max_val = bank.iter().max().unwrap();
    let mut idx_max = bank.iter().position(|&x| x == *max_val).unwrap();

    while idx_max > len_bank - required_size {
        let rest = &bank[..idx_max];
        max_val = rest.iter().max().unwrap();
        idx_max = rest.iter().position(|&x| x == *max_val).unwrap();
    }

    return idx_max;
}

fn part2(banks: Vec<Vec<u32>>) {
    let joltage: u64 = banks.iter().map(|cur| get_joltage_part_2(cur)).sum();
    println!("part1: {}", joltage);
}
