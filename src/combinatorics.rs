use crate::math::factorial;

// pub fn permutations<T>(values: &[T], n: usize) -> impl Iterator<Item = Vec<&T>> {}

pub fn nth_perm<T>(values: &[T], n: usize) -> Option<Vec<&T>> {
    if factorial(values.len() as u64) <= n as u64 {
        return None;
    }
    Some(values.iter().collect())
}
