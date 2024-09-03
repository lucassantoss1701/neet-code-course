pub fn execute(nums: &mut Vec<i64>) -> i64 {
    let mut dup_count = 0;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            dup_count += 1
        }

        nums[i - dup_count] = nums[i];
    }

    let nums_len = nums.len();

    (nums_len - dup_count) as i64
}
