use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;

/// Binary search in rust
///
///```
///use bn_search::searh;
///let data: Vec<i32> = (0..999999).map(i32::from).collect();
///let t = searh(&data, 44);
///```
pub fn searh<I: PartialEq + PartialOrd + Debug>(data: &[I], search: I) -> Option<&I> {
    let middle: usize = data.len() / 2;
    let (left, rigiht): (&[I], &[I]) = data.split_at(middle);

    if rigiht.first() == Some(&search) {
        rigiht.first()
    } else if left.last() == Some(&search) {
        left.last()
    }else if left.last() < Some(&search) {
        searh(rigiht, search)
    } else if rigiht.last() > Some(&search) {
        searh(left, search)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_numeric_bn_search() {
        let data: Vec<i32> = (0..999999).map(i32::from).collect();
        let t = searh(&data, 44);

        assert_eq!(t, Some(&44));
    }
}
