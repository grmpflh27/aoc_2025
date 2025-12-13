use num::integer::div_floor;
use num::zero;
use std::env;
use std::fs;

const DAY: &str = "day1";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let dial_ops_str = read_lines(&test_file_path);
    let dial_ops = parse_dial_ops(&dial_ops_str);

    part1(dial_ops.clone());
    part2(dial_ops.clone());

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let dial_ops_str = read_lines(&file_path);
    let dial_ops = parse_dial_ops(&dial_ops_str);

    part1(dial_ops.clone());
    part2(dial_ops.clone());
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    return fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct DialOp {
    direction: Direction,
    clicks: i32,
}

fn parse_dial_ops(dial_ops_str: &str) -> Vec<DialOp> {
    let mut dial_ops: Vec<DialOp> = Vec::new();

    for line in dial_ops_str.lines() {
        let (dir_str, clicks_str) = line.split_at(1);

        let direction: Direction = match dir_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("unexpected value"),
        };

        let clicks: i32 = clicks_str.parse::<i32>().unwrap();
        dial_ops.push(DialOp { direction, clicks });
    }
    return dial_ops;
}

fn part1(dial_ops: Vec<DialOp>) {
    let mut pos: i32 = 50;
    let mut zeroCnt: i32 = 0;
    for cur in dial_ops.iter() {
        let mut delta = 0;
        if cur.direction == Direction::Left {
            delta = pos - cur.clicks;
        } else {
            delta = pos + cur.clicks;
        }
        pos = (delta).rem_euclid(100);

        zeroCnt += i32::from(pos == 0);
    }
    println!("part1: {}", zeroCnt);
}

fn part2(dial_ops: Vec<DialOp>) {
    let mut pos: i32 = 50;
    let mut zeroCnt: i32 = 0;

    for cur in dial_ops.iter() {
        if cur.direction == Direction::Left {
            let reversed = (100 - pos) % 100;
            zeroCnt += (reversed + cur.clicks) / 100;
            pos = (pos - cur.clicks).rem_euclid(100);
        } else {
            zeroCnt += (pos + cur.clicks) / 100;
            pos = (pos + cur.clicks) % 100;
        }
    }
    println!("part1: {}", zeroCnt);
}
