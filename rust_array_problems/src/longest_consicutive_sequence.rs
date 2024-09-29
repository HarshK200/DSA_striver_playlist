// NOTE: timecomplexity is near about O(n^2)
pub fn longest_consecutive_sequence_bruteforce(nums: Vec<i32>) -> i32 {
    let mut longest_consec_seq_len = 1;

    for i in nums.iter() {
        let mut curr_ele = *i;
        let mut current_seq_len = 1;

        while linear_search(&nums, curr_ele + 1) {
            current_seq_len += 1;
            curr_ele += 1;
        }

        if current_seq_len > longest_consec_seq_len {
            longest_consec_seq_len = current_seq_len;
        }
    }

    return longest_consec_seq_len;
}

pub fn linear_search(arr: &Vec<i32>, target: i32) -> bool {
    for i in arr.iter() {
        if *i == target {
            return true;
        }
    }

    return false;
}

// NOTE: time complexity O(n * log(n)) since we sort the array
// NOTE: space complexity is O(n) since we cp the array
pub fn longest_consecutive_sequence_better(nums: Vec<i32>) -> i32 {
    let mut longest_consec_seq_len = 0;
    let mut nums_cp = nums.clone();
    nums_cp.sort();
    nums_cp.dedup();

    let mut current_seq_len = 0;
    let mut curr_ele = i32::MIN;
    for i in nums_cp {
        if i == (curr_ele + 1) {
            current_seq_len += 1;
            curr_ele += 1;
        } else {
            curr_ele = i;
            current_seq_len = 1;
        }
        longest_consec_seq_len = std::cmp::max(current_seq_len, longest_consec_seq_len);
    }

    return longest_consec_seq_len;
}

pub fn longest_consecutive_sequence_optimal(nums: Vec<i32>) -> i32 {
    let mut longest_consec_seq_len = 0;
    let set: std::collections::HashSet<i32> = nums.into_iter().collect();
    println!("{:?}", set);

    for &num in &set {
        let is_starting_point = !set.contains(&(num - 1));
        if !is_starting_point {
            continue;
        }

        longest_consec_seq_len = longest_consec_seq_len.max(find_consecutive_len(&set, &num))
    }

    return longest_consec_seq_len;
}

// helper function
fn find_consecutive_len(set: &std::collections::HashSet<i32>, num: &i32) -> i32 {
    let mut curr_ele = *num;
    let mut curr_len = 1;

    // NOTE: constant lookup since the set is a hashset(unordered)
    while set.contains(&(curr_ele + 1)) {
        curr_len += 1;
        curr_ele += 1;
    }

    return curr_len;
}
