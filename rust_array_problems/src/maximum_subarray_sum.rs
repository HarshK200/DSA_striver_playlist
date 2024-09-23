// NOTE:  time-complexity is terrible i.e. O(n^3) and space complexity is O(1)
pub fn maximum_subarray_bruteforce(nums: &Vec<i32>) -> i32 {
    let mut max_subarr_sum = nums[0];

    for i in 0..nums.len() {
        for j in i..nums.len() {
            let mut curr_subarr_sum = 0;
            for k in i..j + 1 {
                curr_subarr_sum += nums[k];
            }
            if curr_subarr_sum > max_subarr_sum {
                max_subarr_sum = curr_subarr_sum;
            }
        }
    }

    return max_subarr_sum;
}

// NOTE: time-complexity is O(n^2)
pub fn maximum_subarray_better(nums: &Vec<i32>) -> i32 {
    let mut max_subarr_sum = nums[0];

    for i in 0..nums.len() {
        let mut curr_subarr_sum = 0;
        for j in i..nums.len() {
            curr_subarr_sum += nums[j];
            if curr_subarr_sum > max_subarr_sum {
                max_subarr_sum = curr_subarr_sum;
            }
        }
    }

    return max_subarr_sum;
}

// NOTE: optimal solution kadane's algorithm
pub fn maximum_subarray_optimal(nums: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut max = nums[0];

    for i in nums.iter() {
        sum += *i;
        if sum > max {
            max = sum;
        }
        if sum < 0 {
            sum = 0;
        }
    }

    return max;
}
