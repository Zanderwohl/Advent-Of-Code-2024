mod util;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

use util::parsing;
use crate::util::parsing::{convert_strings, whitepsace_split};

type Num = u64;


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let lines = parsing::file_into_vec("files/day_11_input.txt")?;
    let numbers = whitepsace_split(lines);

    let changed: Result<Vec<Num>, _> = convert_strings(&numbers[0]);
    let changed = changed?;
    let mut map = to_hashmap(&changed);
    for n in 1..=75 {
        map = advance_faster(&map);
        if n == 25 || n == 75 {
            println!("After {} blink(s) the number of stones is:\n{}", n, count_stones(&map));
        }
    }
    
    let duration = start.elapsed();
    println!("Completed in: {:?}", duration);

    Ok(())
}

pub fn advance(stones: &Vec<Num>) -> Vec<Num> {
    let mut changed = Vec::with_capacity(stones.len() * 2);

    'top: for stone in stones {
        let stone = *stone;
        if stone == 0 {
            changed.push(1);
            continue 'top;
        }
        let stone_str = stone.to_string();
        let len = stone_str.len();
        if stone_str.len() % 2 == 0 {
            let n = len / 2;
            let first_half = &stone_str[0..n];
            let second_half = &stone_str[n..];
            changed.push(first_half.parse::<Num>().unwrap());
            changed.push(second_half.parse::<Num>().unwrap());
            continue 'top;
        }
        changed.push(stone * 2024);
    }

    changed
}

pub fn to_hashmap(stones: &Vec<Num>) -> HashMap<Num, Num> {
    let mut map: HashMap<Num, Num> = HashMap::new();
    for stone in stones {
        map.entry(*stone).and_modify(|mut value| { *value += 1}).or_insert(1);
    }
    map
}

pub fn advance_faster(stones: &HashMap<Num, Num>) -> HashMap<Num, Num> {
    let mut new_stones = HashMap::with_capacity(stones.len() * 2);

    let keys = stones.keys();
    'top: for key in keys {
        let stone_label_number = *key;
        let n_stones_like_this = *(stones.get(key).unwrap());
        if stone_label_number == 0 {
            new_stones.entry(1)
                .and_modify(|value| { *value += n_stones_like_this }).or_insert(n_stones_like_this);
            continue 'top;
        }
        let stone_str = stone_label_number.to_string();
        let len = stone_str.len();
        if stone_str.len() % 2 == 0 {
            let n = len / 2;
            let first_half = (&stone_str[0..n]).parse::<Num>().unwrap();
            let second_half = (&stone_str[n..]).parse::<Num>().unwrap();
            new_stones.entry(first_half)
                .and_modify(|mut count| *count += n_stones_like_this)
                .or_insert(n_stones_like_this);
            new_stones.entry(second_half)
                .and_modify(|mut count| *count += n_stones_like_this)
                .or_insert(n_stones_like_this);
            continue 'top;
        }
        new_stones.entry(stone_label_number * 2024)
            .and_modify(|mut count| *count += n_stones_like_this)
            .or_insert(n_stones_like_this);

    }
    new_stones
}

pub fn count_stones(stones: &HashMap<Num, Num>) -> Num {
    stones.values().copied().sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{advance, advance_faster, count_stones, to_hashmap, Num};

    #[test]
    fn basic() {
        let step_0: Vec<Num> = vec![125, 17];
        let step_1: Vec<Num> = vec![253000, 1, 7];
        let step_2: Vec<Num> = vec![253, 0, 2024, 14168];
        let step_3: Vec<Num> = vec![512072, 1, 20, 24, 28676032];
        let step_4: Vec<Num> = vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032];
        let step_5: Vec<Num> = vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32];
        let step_6: Vec<Num> = vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2];

        println!("{:?}", step_0);
        let steps = vec![step_0.clone(), step_1, step_2, step_3, step_4, step_5, step_6];
        for i in 1..(steps.len()) {
            let starting_step = &steps[i - 1];
            let actual = advance(starting_step);
            let expected_step = &steps[i];
            println!("{:?}", actual);
            assert_eq!(actual.len(), expected_step.len());
            if i == 6 {
                assert_eq!(22, actual.len());
            }
        }

        let mut test_case = step_0;
        for i in 1..=25 {
            test_case = advance(&test_case);
        }
        assert_eq!(test_case.len(), 55312);
    }

    #[test]
    fn test_to_hashmap() {
        let step_0: Vec<Num> = vec![125, 17, 17];
        let mut step_0_expected = HashMap::new();
        step_0_expected.insert(125,  1);
        step_0_expected.insert(17,  2);
        let step_0_actual = to_hashmap(&step_0);
        assert_eq!(step_0_actual.keys().len(), step_0_expected.keys().len());
        assert_eq!(step_0_actual.get(&17).unwrap(), step_0_expected.get(&17).unwrap());
        assert_eq!(step_0_actual.get(&125).unwrap(), step_0_expected.get(&125).unwrap());
    }

    #[test]
    fn hashmapped() {
        let step_0: Vec<Num> = vec![125, 17];
        let mut map = to_hashmap(&step_0);
        println!("{:?}", map);

        for i in 1..=25 {
            map = advance_faster(&map);
            if i < 5 {
                println!("{:?}", map);
            }
            if i == 1 {
                assert_eq!(3, count_stones(&map));
            }
            if i == 2 {
                assert_eq!(4, count_stones(&map));
            }
            if i == 3 {
                assert_eq!(5, count_stones(&map));
            }
            if i == 4 {
                assert_eq!(9, count_stones(&map));
            }
            if i == 5 {
                assert_eq!(13, count_stones(&map));
            }
            if i == 6 {
                assert_eq!(22, count_stones(&map));
            }
            if i == 25 {
                println!("{:?}", map);
                assert_eq!(55312, count_stones(&map));
            }
        }
    }
}