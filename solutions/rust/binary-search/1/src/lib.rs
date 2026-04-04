use std::cmp::Ordering;

pub fn find<T: PartialOrd>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    match array.as_ref().len() {
        0 => None,
        1 => {
            return match array.as_ref().get(0).is_some_and(|item| *item == key) {
                true => Some(0),
                false => None,
            };
        }
        _ => {
            let half = array.as_ref().len() / 2;
            match array.as_ref().get(half) {
                Some(item) => match item.partial_cmp(&key) {
                    Some(Ordering::Equal) => return Some(half),
                    Some(Ordering::Greater) => {
                        return find(array.as_ref().get(..half).unwrap(), key);
                    }
                    Some(Ordering::Less) => match find(array.as_ref().get(half..).unwrap(), key) {
                        Some(next_idx) => return Some(half + next_idx),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            }
        }
    }
}
