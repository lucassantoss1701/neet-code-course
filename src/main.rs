mod remove_duplicates_from_sorted_array;
mod remove_element_from_array;

fn main() {
    let arr: &mut Vec<i64> = &mut vec![
        3, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 4, 5, 4, 2, 3, 2, 1, 2, 52, 2, 2, 2, 2, 2,
    ];

    let result = remove_element_from_array::execute(arr, 2);
    println!("{:?}", result)
}
