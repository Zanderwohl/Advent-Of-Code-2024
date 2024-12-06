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

pub fn center<T>(vec: &Vec<T>) -> &T {
    let len = vec.len();
    let offset = (len + 1) % 2;
    let center = (vec.len() / 2) + offset;
    &vec[center]
}

pub fn deep_copy_matrix<T: Clone>(original: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    original.iter().map(|inner| inner.clone()).collect()
}

#[cfg(test)]
mod tests {
    use crate::util::vecstuff::{center, vec_without};

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

    #[test]
    fn find_center() {
        let a = vec![75, 47, 61, 53, 29];
        let a_expected = 61;
        let b = vec![97, 61, 53, 29, 13];
        let b_expected = 53;
        let c = vec![75, 29, 13];
        let c_expected = 29;

        assert_eq!(*center(&a), a_expected);
        assert_eq!(*center(&b), b_expected);
        assert_eq!(*center(&c), c_expected);
    }
}