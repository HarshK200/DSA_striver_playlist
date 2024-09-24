pub fn rearrange_ele_by_sign_bruteforce(nums: Vec<i32>) -> Vec<i32> {
    let mut negative_temp = Vec::new();
    let mut positive_temp = Vec::new();

    for i in nums {
        if i < 0 {
            negative_temp.push(i);
        } else {
            positive_temp.push(i);
        }
    }

    let mut result = Vec::new();
    let mut k = 0;
    let mut j = 0;
    for i in 0..(negative_temp.len() + positive_temp.len()) {
        if i % 2 == 0 {
            result.push(positive_temp[j]);
            j += 1;
        } else {
            result.push(negative_temp[k]);
            k += 1;
        }
    }

    return result;
}

pub fn rearrange_ele_by_sign_optimal(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; nums.len()];
    let mut pos = 0;
    let mut neg = 1;

    for i in nums.iter() {
        if *i < 0 {
            ans[neg] = *i;
            neg += 2;
        } else {
            ans[pos] = *i;
            pos += 2;
        }
    }

    return ans;
}
