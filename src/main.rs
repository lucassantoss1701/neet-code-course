mod remove_duplicates_from_sorted_array;

fn main() {
    let arr: &mut Vec<i64> = &mut vec![1, 1, 2];

    let result = remove_duplicates_from_sorted_array::execute(arr);
    println!("{:?}", result)
}
