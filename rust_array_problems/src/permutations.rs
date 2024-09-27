// NOTE: the timecomplexity is O(n!) * O(n^2) since permutation is n factorial where n is the no of values
// available for combination
// NOTE: space complexity is O(n! * n)
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
