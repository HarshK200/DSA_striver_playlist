pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut prev_count = 0;
    let mut current_count = 0;

    for i in 0..nums.len() {
        if nums[i] == 1 {
            current_count += 1;
        } else {
            if current_count > prev_count {
                prev_count = current_count;
            }
            current_count = 0;
        }
    }

    return std::cmp::max(prev_count, current_count);
}
