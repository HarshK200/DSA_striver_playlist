pub fn rotate_array_suboptimal() {
    let mut nums: Vec<i32> = (1..10).collect();
    let k = 3;

    println!("un-rotated array: {:?}", nums);

    for _j in 0..k {
        let temp = nums[nums.len() - 1];

        for i in (0..(nums.len() - 1)).rev() {
            nums[i + 1] = nums[i];
        }

        nums[0] = temp;
    }

    println!("Rotated array: {:?}", nums);
}

pub fn rotate_array_optimal() {
    let mut nums: Vec<i32> = (1..10).collect();
    let k = 3;

    println!("un-rotated array: {:?}", nums);

    let mut temp = vec![0; nums.len()];

    for i in 0..nums.len() {
        temp[(i + k) % nums.len()] = nums[i];
    }

    nums = temp;
    println!("rotated array: {:?}", nums);
}
