mod util;

use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::util::parsing;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_04_input.txt")?;
    let puzzle = Puzzle::new(&lines);
    let (xmas_count, x_mas_count) = puzzle.map(|puzzle| {
        (puzzle.entire_xmas_count(), puzzle.entire_x_mas_count())
    }).unwrap_or((0, 0));
    println!("Instances of 'XMAS' in puzzle:\n{}", xmas_count);
    println!("Instances of X-'MAS' in puzzle:\n{}", x_mas_count);

    Ok(())
}

fn to_u8(lines: &Vec<String>) -> Option<Vec<Vec<char>>> {
    let height = lines.len();
    if height == 0 {
        return None;
    }
    let width = lines[0].len();
    let mut grid = vec![vec![char::default(); width]; height];
    for (i, line) in lines.iter().enumerate() {
        if line.len() != width {
            return None;
        }
        let chars = line.chars();
        for (j, c) in chars.enumerate() {
            grid[i][j] = c;
        }
    }

    Some(grid)
}

struct Puzzle {
    text: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Puzzle {
    pub fn new(lines: &Vec<String>) -> Option<Self> {
        let height = lines.len() as i32;
        if height == 0 {
            return None;
        }
        let width = lines[0].len() as i32;
        to_u8(lines).map(|text| {
            Self {
                text,
                height,
                width,
            }
        })
    }

    pub fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            '.'
        } else {
            self.text[x as usize][y as usize]
        }
    }

    pub fn get_line(&self, x: i32, y: i32, dx: i32, dy: i32) -> [char; 4] {
        [
            self.get(x, y),
            self.get(x + dx, y + dy),
            self.get(x + dx * 2, y + dy * 2),
            self.get(x + dx * 3, y + dy * 3),
        ]
    }

    pub fn line_is_xmas(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        let line = self.get_line(x, y, dx, dy);
        line[0] == 'X' && line[1] == 'M' && line[2] == 'A' && line[3] == 'S'
    }

    pub fn count_at(&self, x: i32, y: i32) -> usize {
        self.line_is_xmas(x, y, -1, -1) as usize
        + self.line_is_xmas(x, y, 0, -1) as usize
        + self.line_is_xmas(x, y, 1, -1) as usize
        + self.line_is_xmas(x, y, -1, 0) as usize
        + self.line_is_xmas(x, y, 1, 0) as usize
        + self.line_is_xmas(x, y, -1, 1) as usize
        + self.line_is_xmas(x, y, 0, 1) as usize
        + self.line_is_xmas(x, y, 1, 1) as usize
    }

    pub fn entire_xmas_count(&self) -> usize {
        (0..self.height).map(|y| {
            (0..self.width).map(|x|{
                self.count_at(x, y)
            }).sum::<usize>()
        }).sum::<usize>()
    }

    pub fn get_x(&self, x: i32, y: i32) -> [[char; 3]; 3] {
        [
            [
                self.get(x - 1, y - 1),
                self.get(x, y - 1),
                self.get(x + 1, y - 1),
            ],
            [
                self.get(x - 1, y),
                self.get(x, y),
                self.get(x + 1, y),
            ],
            [
                self.get(x - 1, y + 1),
                self.get(x, y + 1),
                self.get(x + 1, y + 1),
            ]
        ]
    }

    pub fn x_is_mas(x: &[[char; 3]; 3]) -> bool {
        if x[1][1] != 'A' {
            return false
        }
        if !((x[0][0] == 'S' && x[2][2] == 'M') || (x[0][0] == 'M' && x[2][2] == 'S')) {
            return false
        }
        if !((x[0][2] == 'S' && x[2][0] == 'M') || (x[0][2] == 'M' && x[2][0] == 'S')) {
            return false
        }
        true
    }

    pub fn x_is_mas_at(&self, x: i32, y: i32) -> bool {
        let x = self.get_x(x, y);
        Self::x_is_mas(&x)
    }

    pub fn entire_x_mas_count(&self) -> usize {
        (0..self.height).map(|y| {
            (0..self.width).map(|x|{
                self.x_is_mas_at(x, y) as usize
            }).sum::<usize>()
        }).sum::<usize>()
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}\n", self.width, self.height)?;
        for line in &self.text {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Puzzle};

    #[test]
    fn structure_puzzle() {
        let puzzle = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string()
        ];
        Puzzle::new(&puzzle).map(|puzzle| {
           println!("{}", puzzle);
        });
    }

    #[test]
    fn test_basic_search() {
        let puzzle = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string()
        ];
        Puzzle::new(&puzzle).map(|puzzle| {
            let actual = puzzle.entire_xmas_count();
            assert_eq!(actual, 18);
        });
    }

    #[test]
    fn test_sparse_search() {
        let puzzle = vec![
            "....XXMAS.".to_string(),
            ".SAMXMS...".to_string(),
            "...S..A...".to_string(),
            "..A.A.MS.X".to_string(),
            "XMASAMX.MM".to_string(),
            "X.....XA.A".to_string(),
            "S.S.S.S.SS".to_string(),
            ".A.A.A.A.A".to_string(),
            "..M.M.M.MM".to_string(),
            ".X.X.XMASX".to_string()
        ];
        Puzzle::new(&puzzle).map(|puzzle| {
            let actual = puzzle.entire_xmas_count();
            assert_eq!(actual, 18);
        });
    }

    #[test]
    fn test_one_x_mas() {
        let puzzles = [
            vec![
              "M.M".to_string(),
              ".A.".to_string(),
              "S.S".to_string(),
            ],
            vec![
                "M.S".to_string(),
                ".A.".to_string(),
                "M.S".to_string(),
            ],
            vec![
                "S.M".to_string(),
                ".A.".to_string(),
                "S.M".to_string(),
            ],
            vec![
                "S.S".to_string(),
                ".A.".to_string(),
                "M.M".to_string(),
            ],
        ];
        for puzzle in puzzles {
            Puzzle::new(&puzzle).map(|puzzle| {
                let actual = puzzle.entire_x_mas_count();
                let x = puzzle.get_x(1, 1);
                println!("{:?}", x);
                assert!(puzzle.x_is_mas_at(1, 1));
                assert_eq!(actual, 1);
            });
        }

    }

    #[test]
    fn test_x_mas_dense() {
        let puzzle = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string()
        ];
        Puzzle::new(&puzzle).map(|puzzle| {
            let actual = puzzle.entire_x_mas_count();
            assert_eq!(actual, 9);
        });
    }
}