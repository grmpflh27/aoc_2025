use std::env;
use std::fs;

const DAY: &str = "day6";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let problems_str = read_lines(&test_file_path);
    let (nums, operators) = parse_formulas(&problems_str);
    part1(&nums, &operators);
    let part2_nums = parse_vertical_nums(&problems_str, &operators);
    part2(&part2_nums, &operators);

    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let problems_str = read_lines(&file_path);
    let (nums, operators) = parse_formulas(&problems_str);
    part1(&nums, &operators);
    let part2_nums = parse_vertical_nums(&problems_str, &operators);
    part2(&part2_nums, &operators);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    return fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
}

fn parse_formulas(problems_str: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    let mut lines = problems_str.lines();
    let first_line = lines.next().unwrap();
    let first_ops: Vec<u64> = first_line
        .split_whitespace()
        .map(|i| i.parse::<u64>().unwrap())
        .collect();
    for op in first_ops {
        nums.push(vec![op])
    }

    let mut parse_ops: bool = true;
    for cur in lines {
        if parse_ops {
            let ops: Vec<u64> = cur
                .split_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect();
            if cur.is_empty() {
                parse_ops = false;
                continue;
            }
            for (idx, op) in ops.iter().enumerate() {
                nums[idx].push(*op);
            }
        } else {
            operators = cur.split_whitespace().collect();
        }
    }
    return (nums, operators);
}

fn parse_vertical_nums(problems_str: &str, operators: &Vec<&str>) -> Vec<Vec<u64>> {
    let mut nums: Vec<Vec<u64>> = vec![vec![]; operators.len()];

    let mut lines = problems_str.lines().rev();
    let last_line = lines.next().unwrap();

    let mut indices: Vec<usize> = last_line
        .char_indices()
        .filter_map(|(i, c)| if !c.is_whitespace() { Some(i) } else { None })
        .collect();

    indices.push(last_line.len() + 1);

    let mut start_idx = indices.remove(0);
    let mut end_idx = indices.remove(0);

    let mut idx: usize = 0;

    loop {
        let rev_lines: Vec<&str> = problems_str.lines().rev().collect();

        let num_operands = end_idx - (start_idx + 1);
        let mut snippets: Vec<String> = vec![String::new(); num_operands];
        for line in rev_lines.iter().skip(2) {
            let snippet = &line[start_idx..end_idx - 1];
            for (char_idx, c) in snippet.chars().into_iter().enumerate() {
                if c == ' ' {
                    continue;
                }
                snippets[char_idx].insert(0, c);
            }
        }
        nums[idx] = snippets.iter().map(|i| i.parse::<u64>().unwrap()).collect();

        if indices.len() == 0 {
            break;
        }

        idx += 1;
        start_idx = end_idx;
        end_idx = indices.remove(0)
    }
    return nums;
}

fn part1(nums: &Vec<Vec<u64>>, operators: &Vec<&str>) {
    let len_formulas = operators.len();

    let mut grand_total: u64 = 0;

    for idx in 0..len_formulas {
        match operators[idx] {
            "*" => {
                grand_total += nums[idx]
                    .iter()
                    .copied()
                    .reduce(|acc, x| acc * x)
                    .unwrap_or_default()
            }
            "+" => grand_total += nums[idx].iter().sum::<u64>(),
            _ => panic!("not supported"),
        }
    }
    println!("part1: {}", grand_total);
}

fn part2(nums: &Vec<Vec<u64>>, operators: &Vec<&str>) {
    let len_formulas = operators.len();

    let mut grand_total: u64 = 0;

    for idx in 0..len_formulas {
        match operators[idx] {
            "*" => {
                grand_total += nums[idx]
                    .iter()
                    .copied()
                    .reduce(|acc, x| acc * x)
                    .unwrap_or_default()
            }
            "+" => grand_total += nums[idx].iter().sum::<u64>(),
            _ => panic!("not supported"),
        }
    }
    println!("part2: {}", grand_total);
}
