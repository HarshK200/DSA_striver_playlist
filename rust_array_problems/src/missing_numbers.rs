pub fn missing_number_best(nums: Vec<i32>) -> i32 {
    let mut missing = nums.len() as i32;
    for i in 0..nums.len() {
        println!("missing: {} | adding {} - {} to it", missing, i, nums[i]);

        missing += i as i32 - nums[i]; // NOTE: the substarction will be 0 at the end if all the
                                       // elements in the range were present in the array else it
                                       // would be the missing element
    }
    return missing;
}

pub fn missing_number_optimal(nums: Vec<i32>) -> i32 {
    let mut temp = nums.clone();
    temp.sort();
    let mut missing: i32 = temp[temp.len() - 1] + 1;

    println!("sorted arr: {:?}", temp);

    for i in 0..temp.len() {
        if i as i32 != temp[i] {
            missing = i as i32;
            break;
        }
    }

    return missing;
}

pub fn missing_number_suboptimal(nums: Vec<i32>) -> i32 {
    let mut largest: i32 = nums[0];

    for i in 0..nums.len() {
        if nums[i] > largest {
            largest = nums[i];
        }
    }

    let mut missing = largest + 1;

    // WARN: sub-optimal cause it calls nums.contains every loop iteration
    for i in 0..largest {
        if !nums.contains(&i) {
            missing = i;
            break;
        }
    }

    return missing;
}
