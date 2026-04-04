use std::cmp::Ordering;

pub fn find<T: Ord>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    let half = array.as_ref().len() / 2;
    match key.cmp(array.as_ref().get(half)?) {
        Ordering::Less => find(array.as_ref().get(..half).unwrap(), key),
        Ordering::Equal => Some(half),
        Ordering::Greater => {
            find(array.as_ref().get(half + 1..).unwrap(), key).map(|i| half + i + 1)
        }
    }
}
