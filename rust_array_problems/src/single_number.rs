// NOTE: time complexity is O(n^2) since we do n iterations for n numbers

pub fn single_number_bruteforce(nums: Vec<i32>) -> i32 {
    let mut missing = nums[0];

    for i in 0..nums.len() {
        let num = nums[i];
        let mut has_dup = false;

        for j in 0..nums.len() {
            if num == nums[j] && j != i {
                println!("checking {} == {}", num, nums[j]);
                has_dup = true;
                break;
            }
        }

        if !has_dup {
            missing = num;
            break;
        }
    }

    return missing;
}

// NOTE: timecomplexity is O(3n) since there are 3 loops
// Space complexity is O(n/2 + 1)

pub fn single_number_better(nums: Vec<i32>) -> i32 {
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for i in nums.iter() {
        map.insert(*i, 0);
    }

    for i in nums.iter() {
        let curr = map.get_mut(&i);
        match curr {
            Some(x) => {
                *x += 1;
            }
            None => {}
        }
    }

    let mut sol = 0;
    for (key, val) in map {
        if val != 2 {
            sol = key;
        }
    }

    return sol;
}

pub fn single_number_optimal(nums: Vec<i32>) -> i32 {
    let mut xor = 0;

    for i in nums {
        xor = xor ^ i;
    }

    return xor;
}
