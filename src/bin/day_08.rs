use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::util::parsing;

mod util;


type Num = i32;


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let lines = parsing::file_into_vec("files/day_08_input.txt")?;
    let (nodes, (width, height)) = parse_nodes(&lines);
    let antinodes = find_antinodes(&nodes, width, height);
    println!("The number of antinodes is:\n{}", antinodes.len());
    let resonant_antinodes = find_resonant_antinodes(&nodes, width, height);
    println!("The number of resonant antinodes is:\n{}", resonant_antinodes.len());

    let duration = start.elapsed();
    println!("Completed in: {:?}", duration);

    Ok(())
}

pub fn parse_nodes(lines: &Vec<String>) -> (Vec<Node>, (isize, isize)) {
    let mut nodes = Vec::new();
    let height = lines.len();
    if height < 1 {
        panic!("Empty input")
    }
    let width = lines[0].len();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().into_iter().enumerate() {
            if c != '.' {
                nodes.push(Node {
                    x: x as isize,
                    y: y as isize,
                    freq: c.to_string(),
                })
            }
        }
    }
    (nodes, (width as isize, height as isize))
}

fn partition_nodes(nodes: &Vec<Node>) -> HashMap<String, Vec<&Node>> {
    let mut node_partitions = HashMap::new();
    for node in nodes {
        let freq = node.freq.clone();
        if !node_partitions.contains_key(&freq) {
            node_partitions.insert(freq.clone(), Vec::new());
        }
        let partition = node_partitions.get_mut(&freq).unwrap();
        partition.push(node);
    }

    node_partitions
}

pub fn find_antinodes(nodes: &Vec<Node>, width: isize, height: isize) -> HashSet<Antinode> {
    let mut antinodes = HashSet::new();
    let node_groups = partition_nodes(&nodes);
    for (_, group) in node_groups {
        let len = group.len();
        for i in 0..len {
            for j in i..len {
                if i == j {
                    continue;
                }
                let a = group[i];
                let b = group[j];
                let (x_dir, y_dir) = determine_dir(&a, &b);
                let diff_x = (a.x - b.x).abs();
                let diff_y = (a.y - b.y).abs();

                let aa = Antinode {
                    x: a.x + (diff_x) * x_dir,
                    y: a.y + (diff_y) * y_dir,
                };
                let ab = Antinode {
                    x: b.x + (diff_x) * x_dir * -1,
                    y: b.y + (diff_y) * y_dir * -1,
                };
                if aa.within_bounds(width, height) {
                    antinodes.insert(aa);
                }
                if ab.within_bounds(width, height) {
                    antinodes.insert(ab);
                }
            }
        }
    }

    antinodes
}

pub fn find_resonant_antinodes(nodes: &Vec<Node>, width: isize, height: isize) -> HashSet<Antinode> {
    let mut antinodes = HashSet::new();
    let node_groups = partition_nodes(&nodes);
    for (_, group) in node_groups {
        let len = group.len();
        for i in 0..len {
            for j in i..len {
                if i == j {
                    continue;
                }
                let a = group[i];
                let b = group[j];
                let (x_dir, y_dir) = determine_dir(&a, &b);
                let diff_x = (a.x - b.x).abs();
                let diff_y = (a.y - b.y).abs();

                let h_times = width / diff_x + 1;
                let v_times = height /diff_y + 1;
                let n = max(h_times, v_times);
                for i in -n..=n {
                    let an = Antinode {
                        x: a.x + (diff_x) * x_dir * i,
                        y: a.y + (diff_y) * y_dir * i,
                    };
                    if an.within_bounds(width, height) {
                        antinodes.insert(an);
                    }
                }
            }
        }
    }

    antinodes
}


fn within_bounds(x: isize, y: isize, width: isize, height: isize) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

fn determine_dir(a: &Node, b: &Node) -> (isize, isize) {
    let x = if a.x == b.x {
        0
    } else if a.x < b.x {
        -1
    } else {
        1
    };
    let y = if a.y == b.y {
        0
    } else if a.y < b.y {
        -1
    } else {
        1
    };
    (x, y)
}

#[derive(Clone)]
struct Node {
    x: isize,
    y: isize,
    freq: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}, {})", self.freq, self.x, self.y)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Antinode {
    x: isize,
    y: isize,
}

impl Display for Antinode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Antinode {
    pub fn within_bounds(&self, width: isize, height: isize) -> bool {
        within_bounds(self.x, self.y, width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::{find_antinodes, find_resonant_antinodes, parse_nodes};

    #[test]
    fn basic() {
        let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let (nodes, (width, height)) = parse_nodes(&lines);

        let antinodes = find_antinodes(&nodes, width, height);
        println!("N: {}", antinodes.len());
        assert_eq!(antinodes.len(), 14);

        let resonant_antinodes = find_resonant_antinodes(&nodes, width, height);
        println!("N: {}", resonant_antinodes.len());
        assert_eq!(resonant_antinodes.len(), 34);
    }
}