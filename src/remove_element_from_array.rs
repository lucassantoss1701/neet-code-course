pub fn execute(nums: &mut Vec<i64>, number_to_remove: i64) -> &mut Vec<i64> {
    let mut left = 0;

    for right in 0..nums.len() {
        if nums[right] != number_to_remove {
            nums[left] = nums[right];
            left += 1;

            
        }
    }

    for i in left..nums.len() {
        nums[i] = 0;
    }

    nums
}
