use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hash;

const DAY: &str = "day8";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let junction_box_str = read_lines(&test_file_path);

    let mut canvas = JunctionBoxCanvas::from(&junction_box_str);
    canvas.make_connections(10);

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let junction_box_str = read_lines(&file_path);

    let mut canvas = JunctionBoxCanvas::from(&junction_box_str);
    canvas.make_connections(1000);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    return fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point3D(u64, u64, u64);

impl Point3D {
    fn distance_squared(&self, other: &Point3D) -> u64 {
        let dx = self.0.wrapping_sub(other.0);
        let dy = self.1.wrapping_sub(other.1);
        let dz = self.2.wrapping_sub(other.2);
        dx.wrapping_mul(dx) + dy.wrapping_mul(dy) + dz.wrapping_mul(dz)
    }
}

struct JunctionBoxCanvas {
    distances: HashMap<u64, (Point3D, Point3D)>,
    circuits: HashMap<Point3D, usize>,
    connections_made: usize,
}

impl From<&String> for JunctionBoxCanvas {
    fn from(junction_box_str: &String) -> Self {
        let pts = parse_point3ds(&junction_box_str);

        let distances = calc_distances(pts.clone());
        let circuits: HashMap<Point3D, usize> =
            pts.iter().enumerate().map(|(i, pt)| (*pt, i)).collect();

        return JunctionBoxCanvas {
            distances,
            circuits,
            connections_made: 0,
        };
    }
}

fn parse_point3ds(junction_box_str: &str) -> Vec<Point3D> {
    let mut points: Vec<Point3D> = Vec::new();
    for line in junction_box_str.lines() {
        let parts: Vec<u64> = line.split(',').map(|i| i.parse::<u64>().unwrap()).collect();
        points.push(Point3D(parts[0], parts[1], parts[2]))
    }

    return points;
}

fn calc_distances(coords: Vec<Point3D>) -> HashMap<u64, (Point3D, Point3D)> {
    let pairs: Vec<(Point3D, Point3D)> = coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (*a, *b))
        .collect();

    let mut distances: HashMap<u64, (Point3D, Point3D)> = HashMap::new();
    for pair in pairs.iter() {
        let distance = pair.0.distance_squared(&pair.1);
        distances.insert(distance, pair.clone());
    }

    return distances;
}

impl JunctionBoxCanvas {
    fn register_min_distance(&mut self) {
        let (min_key, min_val) = self
            .distances
            .iter()
            .min_by(|(&k1, _), (&k2, _)| k1.cmp(&k2))
            .map(|(&k, &v)| (k, v))
            .expect("map not empty");

        self.distances.remove(&min_key);

        if self.circuits[&min_val.0] == self.circuits[&min_val.1] {
            return;
        }

        // check if any already registered as circuit
        let id_1 = self.circuits[&min_val.0];
        let id_2 = self.circuits[&min_val.1];
        let size_1 = self.get_circuits_size(id_1);
        let size_2 = self.get_circuits_size(id_2);

        if size_1 >= size_2 {
            self.circuits.insert(min_val.1, self.circuits[&min_val.0]);
        } else {
            self.circuits.insert(min_val.0, self.circuits[&min_val.1]);
        }

        self.connections_made += 1;
        self.get_circuits_cnts();
    }

    fn get_circuits_size(&self, circuit_id: usize) -> usize {
        return self.get_circuits_cnts()[&circuit_id];
    }

    fn get_circuits_cnts(&self) -> HashMap<usize, usize> {
        let circuit_ids: Vec<usize> = self.circuits.values().map(|i| *i).collect();
        let counts: HashMap<usize, usize> = circuit_ids.into_iter().counts();
        return counts;
    }

    fn get_top_cnts(&self, top_n: usize) -> Vec<(usize, usize)> {
        let counts = self.get_circuits_cnts();

        let top: Vec<_> = counts
            .into_iter()
            .sorted_by(|a, b| b.1.cmp(&a.1)) // desc by count
            .take(top_n)
            .collect();
        return top;
    }

    fn make_connections(&mut self, connection_count: usize) {
        while self.connections_made < connection_count {
            self.register_min_distance();
        }

        let topN = self.get_top_cnts(3);
        let top_n_counts_product = topN
            .iter()
            .map(|i| i.1)
            .collect::<Vec<usize>>()
            .iter()
            .copied()
            .reduce(|acc, x| acc * x)
            .unwrap_or_default();
        println!("{:?}", top_n_counts_product);
    }
}
