pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut temp = Vec::new();
    let mut count = 0;

    for i in 0..nums.len() {
        if !temp.contains(&nums[i]) {
            temp.push(nums[i]);
            count += 1;
        }
    }

    for i in 0..count {
        nums[i] = temp[i];
    }

    return count as i32;
}
