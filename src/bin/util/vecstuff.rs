pub fn vec_without<T: Clone>(vec: &Vec<T>, index: usize) -> Vec<T> {
    vec
        .into_iter()
        .enumerate()
        .filter_map(|(idx, value)| {
            match idx == index {
                true => None,
                false => Some(value.clone())
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use crate::util::vecstuff::vec_without;

    #[test]
    fn test_vec_without() {
        let vec = vec![11, 22, 33, 44];

        let vec_without_0 = vec_without(&vec, 0);
        assert_eq!(vec_without_0, vec![22, 33, 44]);

        let vec_without_1 = vec_without(&vec, 1);
        assert_eq!(vec_without_1, vec![11, 33, 44]);

        let vec_without_2 = vec_without(&vec, 2);
        assert_eq!(vec_without_2, vec![11, 22, 44]);

        let vec_without_3 = vec_without(&vec, 3);
        assert_eq!(vec_without_3, vec![11, 22, 33]);
    }
}