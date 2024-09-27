// NOTE: timecomplexity is O(n^2) since we loop (n + n-1 + n-2 + n-3....) times i.e. (n/2 * (n + 1)) i.e. (n^2 + n)/2
// space complexity is O(n) n being the no. of leaders
pub fn leaders_in_arr_bruteforce(nums: &Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut leaders = Vec::new();

    for i in 0..n {
        let mut is_leader = true;
        for j in (i + 1)..n {
            if nums[i] < nums[j] {
                is_leader = false;
                break;
            }
        }

        if is_leader {
            leaders.push(nums[i]);
        }
    }

    return leaders;
}

// NOTE: time complexity is O(n) when not reversing
// space complexity is O(n) for the leaders array
pub fn leaders_in_arr_optimal(nums: &Vec<i32>) -> Vec<i32> {
    let mut leaders: Vec<i32> = Vec::new();
    let mut max = i32::MIN;

    for i in (0..nums.len()).rev() {
        if nums[i] > max {
            leaders.push(nums[i]);
            max = nums[i];
        }
    }

    // NOTE: returning in the order of the array
    // NOTE: don't reverse if sorted order is needed
    leaders.reverse();

    return leaders;
}
