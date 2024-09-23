pub fn move_zeros(nums: &mut Vec<i32>) {
    let mut temp = Vec::new();

    for i in 0..nums.len() {
        if nums[i] != 0 {
            temp.push(nums[i]);
        }
    }

    for i in 0..nums.len() {
        if i < temp.len() {
            nums[i] = temp[i];
        } else {
            nums[i] = 0;
        }
    }
}

pub fn optimal_move_zeros(nums: &mut Vec<i32>) {
    let mut i = -1 as i32;

    for j in 0..nums.len() {
        if nums[j] != 0 {
            i += 1;
            let temp = nums[i as usize];
            nums[i as usize] = nums[j];
            nums[j] = temp;
        }
    }
}
