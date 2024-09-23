pub fn check(nums: Vec<i32>) -> bool {
    let mut is_sorted = true;
    let mut arr_rotated_start = 0;

    for i in 0..nums.len() - 1 {
        if nums[i + 1] < nums[i] {
            arr_rotated_start = i + 1;
            break;
        }
    }

    println!("the rotated sorted start index: {}", arr_rotated_start);

    let mut current_index;
    let mut next_index;

    for i in 0..nums.len() - 2 {
        current_index = (arr_rotated_start + i) % nums.len();
        next_index = (current_index + 1) % nums.len();

        println!("compairing: {} : {}", nums[current_index], nums[next_index]);

        if nums[current_index] > nums[next_index] {
            is_sorted = false;
            break;
        }
    }

    return is_sorted;
}

// NOTE: TIME COMPLEXITY: O(n) since we loop through the whole array once and then do it again that
// would be O(2 * n) we drop the constant 2 and we get O(n) and the worst case;
