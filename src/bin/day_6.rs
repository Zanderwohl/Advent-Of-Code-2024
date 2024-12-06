mod util;

use std::cmp::PartialEq;
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

    Ok(())
}

fn run_map(map: &Map) -> Map {
    let mut running = true;
    let mut modified_map = map.clone();
    while running {
        running = step_map(&mut modified_map);
    }
    modified_map
}

fn count_visited_map(map: &Map) -> usize {
    map.cells.iter().map(|row| {
        row.iter().map(|cell| {
            (*cell).same(&Cell::Visited) as usize
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

fn step_map(mut map: &mut Map) -> bool {
    let y_len = map.cells.len();
    if y_len == 0 {
        return false
    }

    let (x, y) = map.guard;
    let x = x as usize;
    let y = y as usize;

    let guard = map.cells[y][x].clone();
    map.cells[y][x] = Cell::Visited;
    let (dx, dy, turn) = match guard {
        Cell::GuardUp => (0, -1, &Cell::GuardRight),
        Cell::GuardDown => (0, 1, &Cell::GuardLeft),
        Cell::GuardLeft => (-1, 0, &Cell::GuardUp),
        Cell::GuardRight => (1, 0, &Cell::GuardDown),
        _ => panic!("Can't happen"),
    };
    let next_cell = get_map(&map, (x as isize) + dx, (y as isize) + dy);
    match next_cell {
        None => return false,
        Some(next_cell) => {
            let running = match next_cell {
                Cell::Unvisited => set_guard(&mut map, (x as isize) + dx, (y as isize) + dy, &guard),
                Cell::Visited => set_guard(&mut map, (x as isize) + dx, (y as isize) + dy, &guard),
                Cell::Crate => {
                    map.cells[y][x] = turn.clone();
                    true
                },
                _ => panic!("Can't handle multiple guards!"),
            };
            return running
        }
    }

    true
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

fn set_guard(mut map: &mut Map, x: isize, y: isize, cell: &Cell) -> bool {
    let success = set_map(&mut map, x, y, cell);
    if success {
        map.guard = (x, y);
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
            if (*cell).same(&Cell::Visited) {
                candidates.push((x, y));
            }
        }
    }
    candidates
}

struct Map {
    cells: Vec<Vec<Cell>>,
    guard: (isize, isize),
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
    let cells: Vec<Vec<Cell>> = lines.iter().map( | line| {
        line.chars().map( | c| {
            match c {
                '.' => Cell::Unvisited,
                'X' => Cell::Visited,
                '#' => Cell::Crate,
                '^' => Cell::GuardUp,
                '>' => Cell::GuardRight,
                'v' => Cell::GuardDown,
                '<' => Cell::GuardLeft,
                'O' => Cell::Obstruction,
                _ => Cell::Unvisited,
            }
        }).collect()
    }).collect();
    let mut guard: Option<(isize, isize)> = None;
    for y in 0..lines.len() {
        let x_len = cells[0].len();
        for x in 0..x_len {
            if cells[y][x].is_guard() {
                guard = Some((x as isize, y as isize));
            }
        }
    }
    let guard = guard.expect("No guard found!");
    Map {
        cells,
        guard,
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    Unvisited,
    Visited,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
    Crate,
    Obstruction,
}
// If there is something directly in front of you, turn right 90 degrees.
// Otherwise, take a step forward.

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Unvisited => write!(f, "."),
            Cell::Visited => write!(f, "X"),
            Cell::GuardUp => write!(f, "^"),
            Cell::GuardDown => write!(f, "v"),
            Cell::GuardLeft => write!(f, "<"),
            Cell::GuardRight => write!(f, ">"),
            Cell::Crate => write!(f, "#"),
            Cell::Obstruction => write!(f, "O"),
        }
    }
}

impl Cell {
    pub fn is_guard(&self) -> bool {
        match self {
            Cell::GuardUp => true,
            Cell::GuardDown => true,
            Cell::GuardLeft => true,
            Cell::GuardRight => true,
            _ => false,
        }
    }

    pub fn same(&self, b: &Cell) -> bool {
        matches!((self, b), (Cell::Visited, Cell::Visited))
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