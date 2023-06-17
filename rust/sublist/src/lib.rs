#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(sub_list: &[T], super_list: &[T]) -> Comparison {
    if sub_list == super_list {
        return Comparison::Equal;
    }

    let (super_list, sub_list, list_type) = if super_list.len() > sub_list.len() {
        (super_list, sub_list, Comparison::Sublist)
    } else {
        (sub_list, super_list, Comparison::Superlist)
    };

    let super_len = super_list.len();
    let sub_len = sub_list.len();

    let end = super_len - sub_len + 1;

    for i in 0..end {
        if sub_list == &super_list[i..i + sub_len] {
            return list_type;
        }
    }

    Comparison::Unequal
}
