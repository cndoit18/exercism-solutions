#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if second_list.is_empty()
        || (first_list.len() > second_list.len()
            && first_list
                .windows(second_list.len())
                .any(|x| x == second_list))
    {
        return Comparison::Superlist;
    }
    if first_list.is_empty()
        || (first_list.len() < second_list.len()
            && second_list
                .windows(first_list.len())
                .any(|x| x == first_list))
    {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}
