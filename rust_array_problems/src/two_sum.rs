// NOTE: timecomplexity O(n^2) and space complexity O(1)
pub fn two_sum_bruteforce(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == target && i != j {
                result.push(i as i32);
                result.push(j as i32);
            }
        }
    }

    return result;
}

// NOTE: time complexity O(n) and space complexity O(n + 2)
pub fn two_sum_optimal(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut prev_nums_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

    for i in 0..nums.len() {
        let prev_num_needed = target - nums[i];
        if prev_nums_map.contains_key(&prev_num_needed) {
            let prev_num_idx = *prev_nums_map.get(&prev_num_needed).unwrap() as i32;
            result.push(prev_num_idx);
            result.push(i as i32);
        }
        prev_nums_map.insert(nums[i], i);
    }

    return result;
}
