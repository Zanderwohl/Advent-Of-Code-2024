pub mod util;

use std::collections::HashMap;
use std::error::Error;
use util::parsing;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_1_input.txt")?;
    let (a, b) = parsing::unzip_2(parsing::whitepsace_split(lines))?;
    let a = parsing::convert_strings::<i32>(&a)?;
    let b = parsing::convert_strings::<i32>(&b)?;

    let solution = solve(a.clone(), b.clone());
    println!("Solution is:\n{}", solution);

    let similarity = similar(a, b);
    println!("Smilarity is:\n{}", similarity);

    Ok(())
}

fn solve(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    let mut distance = 0;
    for idx in 0..right.len() {
        let a = left[idx];
        let b = right[idx];

        distance += (a - b).abs()
    }
    distance
}

fn similar(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut occurances: HashMap<i32, i32> = HashMap::with_capacity(left.len());

    for item in left {
        if !occurances.contains_key(&item) {
            occurances.insert(item, 0);
        }
        let count = right.iter().filter(|&n| *n == item).count() as i32;
        let so_far = occurances.get(&item).unwrap();
        occurances.insert(item, count + so_far);
    }
    occurances.into_iter().map(|(number, occurrences)| {
        number * occurrences
    }).sum()
}


#[cfg(test)]
mod test {
    use crate::{similar, solve};

    #[test]
    fn part_one() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        let solution = solve(a, b);
        assert_eq!(solution, 11);
    }

    #[test]
    fn part_two() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        let similarity = similar(a, b);
        assert_eq!(similarity, 31);
    }
}