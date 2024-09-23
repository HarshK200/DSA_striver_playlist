// NOTE: time-complexity O(n + k) where n is the size of the array and k is the total no. of
// unique elements in the array (since they will be pushed to map and then iterated over at the end)
pub fn majority_ele_bruteforce(nums: Vec<i32>) -> i32 {
    let mut value_to_count: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let n = nums.len();

    for i in nums.iter() {
        let count = value_to_count.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut majority_ele = nums[0];
    for (value, count) in value_to_count {
        if count > (n as i32) / 2 {
            majority_ele = value;
        }
    }

    return majority_ele;
}

// NOTE: moore's voting algorithm
// time-complexity O(n) and space complexity O(1)
pub fn majority_ele_optimal(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut ele = 0;

    for i in 0..nums.len() {
        if count == 0 {
            ele = nums[i];
        }
        if ele == nums[i] {
            count += 1;
        } else {
            count -= 1;
        }
    }

    return ele;
}
