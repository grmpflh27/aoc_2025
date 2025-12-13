use std::collections::HashSet;
use std::env;
use std::fs;

const DAY: &str = "day2";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let ranges_str = read_lines(&test_file_path);
    let ranges = parse_ranges(&ranges_str);

    part1(ranges.clone());
    part2(ranges.clone());

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let ranges_str = read_lines(&file_path);
    let ranges = parse_ranges(&ranges_str);

    part1(ranges.clone());
    part2(ranges.clone());
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    return fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
}

#[derive(Debug, Clone)]
struct ProductIdRange {
    start: String,
    end: String,
}

impl ProductIdRange {
    fn get_range(&self) -> Vec<String> {
        let start = self.start.parse::<u64>().unwrap();
        let end = self.end.parse::<u64>().unwrap();
        let startEnd: Vec<_> = (start..end + 1).collect();
        return startEnd.iter().map(|i| i.to_string()).collect();
    }

    fn get_invalid(&self, part1: bool) -> Vec<String> {
        let mut invalid: Vec<String> = Vec::new();

        for cur in self.get_range().iter() {
            if part1 {
                if self.is_invalid_part1(cur) {
                    invalid.push(cur.clone());
                }
            } else {
                if self.is_invalid_part2(cur) {
                    invalid.push(cur.clone());
                }
            }
        }
        return invalid;
    }

    fn is_invalid_part1(&self, product_id: &String) -> bool {
        if product_id.len() % 2 == 1 {
            return false;
        }

        let mid: usize = product_id.len() / 2;
        let (first_half, second_half) = product_id.split_at(mid);

        if (first_half == second_half) {
            return true;
        }
        return false;
    }

    fn is_invalid_part2(&self, product_id: &String) -> bool {
        let digits = product_id.len();
        if digits < 2 {
            return false;
        }

        // half
        if digits % 2 == 0 {
            let mid: usize = product_id.len() / 2;
            let (first_half, second_half) = product_id.split_at(mid);

            if (first_half == second_half) {
                return true;
            }
        }

        // all identical
        if matches!(digits, 3 | 5 | 7 | 9) {
            let unique: HashSet<char> = product_id.chars().collect();
            if unique.len() == 1 {
                return true;
            }
        }

        // triplets
        if matches!(digits, 6 | 9) {
            let aThird: usize = product_id.len() / 3;
            let (first_third, rest) = product_id.split_at(aThird);
            let (second_third, last_third) = rest.split_at(aThird);

            if (first_third == second_third && second_third == last_third) {
                return true;
            }
        }

        // quintuplets
        if matches!(digits, 10) {
            let chunks: Vec<&str> = product_id
                .as_bytes()
                .chunks(2)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect();

            let unique: HashSet<&str> = chunks.into_iter().collect::<HashSet<_>>();

            if unique.len() == 1 {
                return true;
            }
        }

        return false;
    }
}

fn parse_ranges(ranges_str: &str) -> Vec<ProductIdRange> {
    let mut ranges: Vec<ProductIdRange> = Vec::new();
    let parts: Vec<&str> = ranges_str.split(',').collect();

    for cur in parts {
        let start_end: Vec<_> = cur.split('-').collect();
        ranges.push(ProductIdRange {
            start: start_end[0].to_string(),
            end: start_end[1].to_string(),
        })
    }
    return ranges;
}

fn part1(ranges: Vec<ProductIdRange>) {
    let mut all_invalid: Vec<String> = Vec::new();
    for cur in ranges {
        let cur_invalid = cur.get_invalid(true);
        all_invalid.extend(cur_invalid);
    }

    println!("{:?}", all_invalid);
    let part1: u64 = all_invalid
        .iter()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("part1: {}", part1);
}

fn part2(ranges: Vec<ProductIdRange>) {
    let mut all_invalid: Vec<String> = Vec::new();
    for cur in ranges {
        let cur_invalid = cur.get_invalid(false);
        all_invalid.extend(cur_invalid);
    }

    println!("{:?}", all_invalid);
    let part2: u64 = all_invalid
        .iter()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("part2: {}", part2);
}
