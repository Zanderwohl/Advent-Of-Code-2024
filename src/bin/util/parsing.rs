use std::io;
use std::fs::File;
use std::path::Path;
use std::fmt::Debug;
use std::io::BufRead;

pub fn file_into_vec<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file_lines = read_lines(path)?;
    let lines = file_lines.flat_map(|line| {
        line.ok()
    }).collect();

    Ok(lines)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn whitepsace_split(lines: Vec<String>) -> Vec<Vec<String>> {
    lines.iter().map(|line| {
        line.split_whitespace().map(String::from).collect()
    }).collect()
}

pub fn comma_split(lines: &Vec<String>) -> Vec<Vec<String>> {
    generic_split(lines, ",")
}

pub fn pipe_split(lines: &Vec<String>) -> Vec<Vec<String>> {
    generic_split(lines, "|")
}

pub fn generic_split(lines: &Vec<String>, on: &str) -> Vec<Vec<String>> {
    lines.iter().map(|line| {
        line.split(on).map(String::from).collect()
    }).collect()
}

pub fn transpose<T: Debug + Clone>(vec: &Vec<Vec<T>>) -> Result<Vec<Vec<T>>, io::Error> {
    let size: usize = vec.len();
    if size == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Array is empty".to_string(),
        ))
    }
    let n: usize = vec[0].len();
    let mut transposed: Vec<Vec<T>> = Vec::with_capacity(n);
    for _ in 0..n {
        transposed.push(Vec::with_capacity(size));
    }
    for (i, sub_vec) in vec.iter().enumerate() {
        let sub_vec_len = sub_vec.len();
        if sub_vec_len != n {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Item {} in vec has len {} when it should be {}.", i, sub_vec_len, n),
            ))
        }
        for (j, _) in sub_vec.iter().enumerate() {
            transposed[j].push(vec[i][j].clone());
        }
    }
    Ok(transposed)
}

pub fn unzip_2<T: Debug + Clone>(lines: Vec<Vec<T>>) -> Result<(Vec<T>, Vec<T>), io::Error> {
    let n: usize = 2;

    let mut a = Vec::with_capacity(lines.len());
    let mut b = Vec::with_capacity(lines.len());
    for (idx, line) in lines.iter().enumerate() {
        if line.len() != n {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("All lines must have {} items, but one with {}  was found: ({}: {:?})", n, line.len(), idx, line)
            ))
        }
        a.push(line[0].clone());
        b.push(line[1].clone());
    }

    Ok((a, b))
}

pub fn convert_strings<T: std::str::FromStr>(vec: &Vec<String>) -> Result<Vec<T>, T::Err> {
    vec.iter()
        .map(|string| string.parse::<T>())
        .collect()
}

pub fn convert_strings_matrix<T: std::str::FromStr>(vec: &Vec<Vec<String>>) -> Result<Vec<Vec<T>>, T::Err> {
    vec.iter()
        .map(|inner| {
            convert_strings::<T>(inner)
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::transpose;

    #[test]
    fn test_transpose() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let a_t_expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let a_t_actual = transpose(&a).expect("Transpose failed!");
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                assert_eq!(a[i][j], a_t_actual[j][i]);
                assert_eq!(a_t_actual[j][i], a_t_expected[j][i])
            }
        }
    }
}
