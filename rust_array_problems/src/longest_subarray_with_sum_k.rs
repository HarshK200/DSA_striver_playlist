// NOTE: time-complexity approx O(n^2)
// and space-complexity of O(1)
pub fn longest_subarray_bruteforce(nums: &Vec<i32>, target: i32) -> i32 {
    let mut length = 0;

    // NOTE: generating all possible sub arrays
    for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            if sum == target {
                length = std::cmp::max(length, j - i + 1);
            }
        }
    }

    return length as i32;
}

// NOTE: This solution is the optimal solution when the array contains 0's and -ve numbers
// The time complexity is O(n * 1) since we iterate through once the time complexity n and we use
// hashmap.contains_key which is constant time.
// WARN: there is a minor chance that all the items map to the same bucket in the haspmap i.e.
// collison then timecomplexity is O(n) worst case. (VERY RARE)
// NOTE: space-complexity O(n)
pub fn longest_subarray_better(nums: &Vec<i32>, target: i32) -> i32 {
    let mut prev_sum_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    let mut sum = 0;
    let mut max_len = 0;

    // NOTE: Loop through the entire array
    for i in 0..nums.len() {
        sum += nums[i];
        // NOTE: for the first time when sum == target
        if sum == target {
            max_len = i + 1;
        }

        let remaining = sum - target;
        if prev_sum_map.contains_key(&remaining) {
            let len = i - prev_sum_map[&(remaining as i32)];
            max_len = std::cmp::max(max_len, len);
        }
        // NOTE: do not replace if the map already has the similar sum because similar sum will
        // only occur on 0 / -ve numbers
        if !prev_sum_map.contains_key(&sum) {
            prev_sum_map.insert(sum, i);
        }
    }

    return max_len as i32;
}

// NOTE: if the array contains only positive and 0's then this works else it doesn't
// time complexity is O(n) and space-complexity is O(1);
pub fn longest_subarray_optimal(nums: &Vec<i32>, target: i32) -> i32 {
    let mut max_len = 0;

    let mut sum = nums[0];
    let mut j = 0;
    let mut i = 0;
    while j < nums.len() {
        while i <= j && sum > target {
            sum -= nums[i];
            i += 1;
        }
        if sum == target {
            let len = j - i + 1;
            max_len = std::cmp::max(len, max_len);
        }
        j += 1;
        if j < nums.len() {
            sum += nums[j]
        }
    }

    return max_len as i32;
}
