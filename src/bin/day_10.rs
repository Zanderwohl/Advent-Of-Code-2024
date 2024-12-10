use std::collections::HashSet;
use std::error::Error;
use std::time::Instant;
use crate::util::parsing;

mod util;


type Num = u8;


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let lines = parsing::file_into_vec("files/day_10_input.txt")?;
    let map = parse(&lines);
    let (scores, ratings) = score_and_rate_trails(&map);
    println!("The total scores of all trailheads are:\n{}", scores);
    println!("The total ratings of all trailheads are:\n{}", ratings);

    let duration = start.elapsed();
    println!("Completed in: {:?}", duration);

    Ok(())
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Num>> {
    lines.iter()
        .map(|string| string.chars().map(|c| {
            ((c as u8) - 48) as Num
        }).collect())
        .collect()
}

pub fn score_and_rate_trails(map: &Vec<Vec<Num>>) -> (usize, usize) {
    let mut scores: usize = 0;
    let mut ratings: usize = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                let (score, rating) = score_and_rate_trail(map, x, y);
                scores += score;
                ratings += rating;
            }
        }
    }
    (scores, ratings)
}

pub fn score_and_rate_trail(map: &Vec<Vec<Num>>, x: usize, y: usize) -> (usize, usize) {
    let mut rating = Vec::new();
    rate_trail_inner(map, x, y, &mut rating);
    let mut score = HashSet::with_capacity(rating.len());
    for (x_, y_) in &rating {
        score.insert((*x_, *y_));
    }

    (score.len(), rating.len())
}

pub fn rate_trail_inner(map: &Vec<Vec<Num>>, x: usize, y: usize, mut vec: &mut Vec<(usize, usize)>) {
    let current_n = map[y][x];
    if current_n == 9 {
        vec.push((x, y));
        return;
    }
    let dirs = directions(map, x, y);
    for (next_x, next_y) in dirs {
        rate_trail_inner(map, next_x, next_y, &mut vec)
    }
}



pub fn directions(map: &Vec<Vec<Num>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let height = map.len();
    let width = map[0].len();
    let dirs: &[(isize, isize);4] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
    let positions: Vec<(usize, usize)> = dirs.iter()
        .map(|(dx, dy)| { (x as isize + dx, y as isize + dy) })
        .filter(|(x, y)| {
            *x >= 0
            && *y >= 0
            && *x < width as isize
            && *y < height as isize
        })
        .map(|(x, y)| { (x as usize, y as usize) })
        .filter(|(new_x, new_y)| map[*new_y][*new_x] == map[y][x] + 1)
        .collect();
    positions
}

#[cfg(test)]
mod tests {
    use crate::{directions, parse, score_and_rate_trail, score_and_rate_trails, Num};

    #[test]
    fn test_score_paths() {
        let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let expected_scores: Vec<(usize, usize, usize, usize)> = vec![
            (5, 20, 0, 2),
            (6, 24, 0, 4),
            (5, 10, 2, 4),
            (3, 4, 4, 6),
            (1, 1, 5, 2),
            (3, 4, 5, 5),
            (5, 5, 6, 0),
            (3, 8, 6, 6),
            (5, 5, 7, 1),
        ];
        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let map = parse(&lines);
        let (score, rating) = score_and_rate_trails(&map);
        let actual_scores: Vec<(usize, usize, usize, usize, usize, usize)> = expected_scores.iter().map(|(expected_score, expected_rating, y, x)| {
            let (actual_score, actual_rating) = score_and_rate_trail(&map, *x, *y);
            println!("({}, {}) -> (expected: {}, actual: {})", x, y, *expected_score, actual_score);
            (*expected_score, *expected_rating, *x, *y, actual_score, actual_rating)
        }).collect();
        let expected_score = 36;
        let expected_rating = 81;
        println!("Total Score: {} (Expected {})", score, expected_score);
        for (expected_score, expected_rating, x, y, actual_score, actual_rating) in actual_scores {
            assert_eq!(expected_score, actual_score);
            assert_eq!(expected_rating, actual_rating);
        }
        assert_eq!(score, expected_score);
        assert_eq!(rating, expected_rating);
    }

    #[test]
    fn test_one_path_score() {
        let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let map = parse(&lines);
        let (actual_score, actual_rating) = score_and_rate_trail(&map, 2, 0);
        assert_eq!(actual_score, 5);
    }

    #[test]
    fn test_directions() {
        let only_right: Vec<Vec<Num>> = vec![
            vec![5, 2, 5],
            vec![0, 0, 1],
            vec![5, 5, 5],
        ];
        let dirs = directions(&only_right, 1, 1);
        println!("{:?}", dirs);
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0], (2usize, 1usize));

        let all_four: Vec<Vec<Num>> = vec![
            vec![5, 1, 5],
            vec![1, 0, 1],
            vec![5, 1, 5],
        ];
        let dirs = directions(&all_four, 1, 1);
        println!("{:?}", dirs);
        assert_eq!(dirs.len(), 4);

        let all_eight: Vec<Vec<Num>> = vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ];
        let dirs = directions(&all_eight, 1, 1);
        println!("{:?}", dirs);
        assert_eq!(dirs.len(), 4);

        let from_five: Vec<Vec<Num>> = vec![
            vec![5, 6, 5],
            vec![1, 5, 1],
            vec![5, 6, 5],
        ];
        let dirs = directions(&from_five, 1, 1);
        println!("{:?}", dirs);
        assert_eq!(dirs.len(), 2);
    }
}
