use std::collections::HashSet;
use std::env;
use std::fs;

const DAY: &str = "day4";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let diagram_str = read_lines(&test_file_path);
    let mut diagram: Diagram = parse_diagram(&diagram_str);

    let count = diagram.count_accessible_papers();
    println!("part1: {}", count);
    let drop_cnt = diagram.drop_accessible();
    println!("part2: {}", drop_cnt);

    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let diagram_str = read_lines(&file_path);
    let mut diagram: Diagram = parse_diagram(&diagram_str);

    println!("FOR REAL");
    let count = diagram.count_accessible_papers();
    println!("part1: {}", count);
    let drop_cnt = diagram.drop_accessible();
    println!("part2: {}", drop_cnt);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    return fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

#[derive(Debug, Clone)]
struct Diagram {
    papers: HashSet<Pos>,
    width: usize,
    length: usize,
}

impl Diagram {
    fn get_neighbors(&self, paper: &Pos) -> Vec<Pos> {
        let mut offsets: Vec<Pos> = vec![
            Pos(0, 1),
            Pos(1, 1),
            Pos(1, 0),
            Pos(1, -1),
            Pos(0, -1),
            Pos(-1, -1),
            Pos(-1, 0),
            Pos(-1, 1),
        ];

        return offsets
            .iter()
            .map(|cur| Pos(paper.0 + cur.0, paper.1 + cur.1))
            .filter(|cur| {
                cur.0 >= 0 && cur.0 < self.width as i32 && cur.1 >= 0 && cur.1 < self.length as i32
            })
            .collect();
    }

    fn count_neigh_papers(&self, paper: &Pos) -> u32 {
        let neighbors: Vec<Pos> = self.get_neighbors(paper);
        let paper_cnt: u32 = neighbors
            .iter()
            .filter_map(|cur| {
                if self.papers.contains(cur) {
                    Some(1)
                } else {
                    Some(0)
                }
            })
            .sum::<u32>();
        return paper_cnt;
    }

    fn get_accessible_papers(&self) -> Vec<Pos> {
        let mut papers: Vec<Pos> = Vec::new();
        for paper in self.papers.iter() {
            let cnt = self.count_neigh_papers(&paper);
            if cnt < 4 {
                papers.push(paper.clone());
            }
        }
        return papers;
    }

    fn count_accessible_papers(&self) -> u32 {
        let accessible = self.get_accessible_papers();
        return accessible.len() as u32;
    }

    fn drop_accessible(&mut self) -> u32 {
        let mut dropped: u32 = 0;
        loop {
            let accessible = self.get_accessible_papers();
            let to_remove_set: HashSet<Pos> = accessible.into_iter().collect();
            let to_remove_len = to_remove_set.len();
            if to_remove_len == 0 {
                break;
            }

            dropped += to_remove_len as u32;
            self.papers.retain(|x| !to_remove_set.contains(x));
        }
        return dropped;
    }
}

fn parse_diagram(diagram_str: &str) -> Diagram {
    let mut paper_pos: Vec<Pos> = Vec::new();

    let mut lines = diagram_str.lines();
    let width: usize = lines.next().map_or(0, |first| first.len());
    let length: usize = lines.count() + 1;

    for (cur_y, line) in diagram_str.lines().enumerate() {
        let paper_idxs_in_row: Vec<_> = line
            .char_indices()
            .filter_map(|(i, ch)| if ch == '@' { Some(i) } else { None })
            .collect();
        paper_pos.extend(
            paper_idxs_in_row
                .iter()
                .map(|cur_x| Pos(*cur_x as i32, cur_y as i32))
                .collect::<Vec<Pos>>(),
        );
    }

    let papers_pos_set: HashSet<Pos> = paper_pos.into_iter().collect();
    return Diagram {
        papers: papers_pos_set,
        width,
        length,
    };
}

// fn part1(banks: Vec<Vec<u32>>) {
//     let joltage: u32 = banks.iter().map(|cur| get_joltage_part_1(cur)).sum();
//     println!("part1: {}", joltage);
// }

// fn part2(banks: Vec<Vec<u32>>) {
//     let joltage: u64 = banks.iter().map(|cur| get_joltage_part_2(cur)).sum();
//     println!("part1: {}", joltage);
// }
