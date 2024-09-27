fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums_clone = nums.clone();
    let mut result: Vec<Vec<i32>> = Vec::new();

    // NOTE: base case
    if nums_clone.len() == 1 {
        return vec![nums_clone];
    }

    for _i in 0..nums_clone.len() {
        let n = nums_clone.remove(0);

        // NOTE: recursive call
        let perms = permute(nums_clone.clone());

        for mut perm in perms {
            perm.push(n);
            result.push(perm);
        }

        nums_clone.push(n);
    }

    return result;
}

// NOTE: generate all the permutation and then sort them
pub fn next_permutation_bruteforce(nums: &mut Vec<i32>) {
    let mut permutations = permute(nums.clone());
    permutations.sort();
    permutations.dedup();

    for i in 0..permutations.len() {
        let permutation = permutations[i].clone();

        if permutation == *nums {
            if i + 1 == permutations.len() {
                *nums = permutations[0].clone();
                break;
            }
            *nums = permutations[i + 1].clone();
            break;
        }
    }

    println!("next permutation is {:?}", nums);
}

// NOTE: time complexity is O(3 * n) since we loop twice and a reverse
// space complexity is O(1) since we don't use extra space
pub fn next_permutation_optimal(nums: &mut Vec<i32>) {
    let mut dip_idx = -1;
    let n = nums.len();

    if nums.len() == 1 {
        return;
    }

    // NOTE: finding the dip
    for i in (0..=n - 2).rev() {
        if nums[i] < nums[i + 1] {
            dip_idx = i as i32;
            break;
        }
    }

    println!("dipidx = {dip_idx}");

    // NOTE: no dip is found that means the the given permutation is lexio-graphically the largest one
    if dip_idx == -1 {
        nums.reverse();
        return;
    }

    for i in ((dip_idx + 1) as usize..=n - 1).rev() {
        if nums[i] > nums[dip_idx as usize] {
            nums.swap(i, dip_idx as usize);
            break;
        }
    }

    nums[(dip_idx + 1) as usize..n].reverse();

    println!("next permutation is {:?}", nums);
}
