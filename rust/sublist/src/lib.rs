#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // let (super_set, sub_set) = if second_list.len() > first_list.len() {
    //     (first_list, second_list)
    // } else {
    //     (second_list,first_list)
    // } ;

    let a_len = first_list.len();
    let b_len = second_list.len();

    if a_len == b_len && first_list == second_list {
        return Comparison::Equal;
    };

    if b_len > a_len {
        let end = b_len - a_len + 1;

        for i in 0..end {
            let a = first_list;
            let b = &second_list[i..i + a_len];

            if a == b {
                return Comparison::Sublist;
            }
        }
    } else if a_len > b_len {
        let end = a_len - b_len + 1;
        for i in 0..end {
            let a = second_list;
            let b = &first_list[i..i + b_len];

            if a == b {
                return Comparison::Superlist;
            }
        }
    }

    Comparison::Unequal
}
