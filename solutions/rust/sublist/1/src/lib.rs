#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if superlist(first_list, second_list) {
        return Comparison::Superlist;
    }

    if superlist(second_list, first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

fn superlist(first: &[i32], second: &[i32]) -> bool {
    second.is_empty() || first.windows(second.len()).any(|window| window == second)
}
