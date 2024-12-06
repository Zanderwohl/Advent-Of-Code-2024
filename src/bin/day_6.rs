mod util;

use std::cmp::PartialEq;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::util::parsing;
use crate::util::vecstuff::deep_copy_matrix;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_6_input.txt")?;
    let map = parse(&lines);
    let new_map = run_map(&map);
    let guess = count_visited_map(&new_map);
    println!("The number of unique spaces the guard visited was:\n{}", guess);

    let obstacle_locations = find_obstacle_locations(&map, &new_map, guess);

    Ok(())
}

fn run_map(map: &Map) -> Map {
    let mut running = true;
    let mut modified_map = map.clone();
    while running {
        (running, _) = step_map(&mut modified_map);
    }
    modified_map
}

fn count_visited_map(map: &Map) -> usize {
    map.cells.iter().map(|row| {
        row.iter().map(|cell| {
            (*cell).same(&Cell::Visited(VisitHistory::default())) as usize
        }).sum::<usize>()
    }).sum::<usize>()
}

fn print_map(map: &Map) {
    for line in &map.cells {
        for c in line {
            print!("{}", c);
        }
        println!()
    }
}

fn step_map(mut map: &mut Map) -> (bool, bool) {
    let y_len = map.cells.len();
    if y_len == 0 {
        return (false, false)
    }
    let (x, y) = (map.guard.x as usize, map.guard.y as usize);

    //if map.cells[y][x].same(&Cell::Unvisited) {
        map.cells[y][x] = Cell::Visited(VisitHistory::default());
    //}
    match &mut map.cells[y][x] {
        Cell::Visited(history) => {
            history.add(&map.guard.dir)
        }
        _ => {}
    }

    let (dx, dy) = map.guard.next_action();
    let next_cell = get_map(&map, (x as isize) + dx, (y as isize) + dy);
    match next_cell {
        None => (false, false),
        Some(next_cell) => {
            let running = match next_cell {
                Cell::Unvisited => {
                    set_guard(&mut map, (x as isize) + dx, (y as isize) + dy)
                },
                Cell::Visited(visit_history) => {
                    set_guard(&mut map, (x as isize) + dx, (y as isize) + dy)
                },
                Cell::Crate => {
                    map.guard.turn();
                    true
                },
                Cell::Obstruction => {
                    map.guard.turn();
                    true
                }
                _ => panic!("Can't handle multiple guards!"),
            };
            (running, false)
        }
    }
}

fn within_map(map: &Map, x: isize, y: isize) -> bool {
    let len = map.cells.len();
    x >= 0 && y >= 0 && (y as usize) < len && len != 0 && (x as usize) < map.cells[0].len()
}

fn get_map(map: &Map, x: isize, y: isize) -> Option<Cell> {
    if within_map(map, x, y) {
        Some(map.cells[y as usize][x as usize].clone())
    } else {
        None
    }
}

fn set_guard(mut map: &mut Map, x: isize, y: isize) -> bool {
    let success = within_map(&mut map, x, y);
    if success {
        map.guard.x = x;
        map.guard.y = y;
    }
    success
}

fn set_map(map: &mut Map, x: isize, y: isize, cell: &Cell) -> bool {
    if within_map(map, x, y) {
        map.cells[y as usize][x as usize] = cell.clone();
        true
    } else {
        false
    }
}

fn coarse_candidate_obstacles(original_run: &Map, unique_positions: usize) -> Vec<(usize, usize)> {
    let mut candidates = Vec::with_capacity(unique_positions);
    for (y, row) in original_run.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if (*cell).same(&Cell::Visited(Default::default())) {
                candidates.push((x, y));
            }
        }
    }
    candidates
}

fn find_obstacle_locations(fresh_map: &Map, original_run: &Map, unique_positions: usize) -> usize {
    let candidates = coarse_candidate_obstacles(original_run, unique_positions);
    println!("");

    0
}

struct Map {
    cells: Vec<Vec<Cell>>,
    guard: Guard,
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Map {
            cells: deep_copy_matrix(&self.cells),
            guard: self.guard.clone(),
        }
    }
}

fn parse(lines: &Vec<String>) -> Map {
    let mut guard: Option<Guard> = None;
    let cells: Vec<Vec<Cell>> = lines.iter().enumerate().map( | (y, line)| {
        line.chars().enumerate().map( | (x, c)| {
            match c {
                '.' => Cell::Unvisited,
                'X' => Cell::Visited(Default::default()),
                '#' => Cell::Crate,
                '^' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        dir: GuardDir::Up
                    });
                    Cell::Unvisited
                },
                '>' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        dir: GuardDir::Right
                    });
                    Cell::Unvisited
                },
                'v' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        dir: GuardDir::Down
                    });
                    Cell::Unvisited
                },
                '<' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        dir: GuardDir::Left
                    });
                    Cell::Unvisited
                },
                'O' => Cell::Obstruction,
                _ => Cell::Unvisited,
            }
        }).collect()
    }).collect();
    let guard = guard.expect("No guard found!");
    Map {
        cells,
        guard,
    }
}

#[derive(Clone)]
struct Guard {
    dir: GuardDir,
    x: isize,
    y: isize,
}

#[derive(Clone)]
enum GuardDir {
    Up,
    Right,
    Down,
    Left,
}

impl Guard {
    pub fn next_action(&self) -> (isize, isize) {
        match &self.dir {
            GuardDir::Up => (0, -1),
            GuardDir::Left => (-1, 0),
            GuardDir::Right => (1, 0),
            GuardDir::Down => (0, 1),
            _ => panic!("Can't happen"),
        }
    }

    pub fn turn(&mut self) {
        self.dir = match self.dir {
            GuardDir::Up => GuardDir::Right,
            GuardDir::Right => GuardDir::Down,
            GuardDir::Down => GuardDir::Left,
            GuardDir::Left => GuardDir::Up,
        }
    }

    pub fn visited_before(&self, history: &VisitHistory) -> bool {
        match self.dir {
            GuardDir::Up => history.up,
            GuardDir::Right => history.right,
            GuardDir::Down => history.down,
            GuardDir::Left => history.left,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    Unvisited,
    Visited(VisitHistory),
    Crate,
    Obstruction,
}

// If there is something directly in front of you, turn right 90 degrees.
// Otherwise, take a step forward.

#[derive(Clone, PartialEq)]
struct VisitHistory {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

impl Default for VisitHistory {
    fn default() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
}

impl VisitHistory {
    pub fn add(&mut self, dir: &GuardDir) {
        match dir {
            GuardDir::Up => self.up = true,
            GuardDir::Right => self.right = true,
            GuardDir::Down => self.down = true,
            GuardDir::Left => self.left = true,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Unvisited => write!(f, "."),
            Cell::Visited(_) => write!(f, "X"),
            Cell::Crate => write!(f, "#"),
            Cell::Obstruction => write!(f, "O"),
        }
    }
}

impl Cell {

    pub fn same(&self, b: &Cell) -> bool {
        matches!((self, b), (Cell::Visited(_), Cell::Visited(_)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{count_visited_map, parse, print_map, run_map};

    #[test]
    fn simple() {
        let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let map = parse(&lines);
        print_map(&map);
        let new_map = run_map(&map);
        println!("---");
        print_map(&new_map);

        let guess = count_visited_map(&new_map);

        assert_eq!(guess, 41);
    }
}