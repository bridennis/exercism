use std::cmp::Ordering;

pub fn find<T: PartialOrd>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    match array.as_ref().len() {
        0 => None,
        1 => match array.as_ref().first().is_some_and(|item| *item == key) {
            true => Some(0),
            false => None,
        },
        _ => {
            let half = array.as_ref().len() / 2;
            match array.as_ref().get(half) {
                Some(item) => match item.partial_cmp(&key) {
                    Some(Ordering::Equal) => Some(half),
                    Some(Ordering::Greater) => find(array.as_ref().get(..half).unwrap(), key),
                    Some(Ordering::Less) => find(array.as_ref().get(half..).unwrap(), key)
                        .map(|next_idx| half + next_idx),
                    _ => None,
                },
                _ => None,
            }
        }
    }
}
