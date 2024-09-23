
// WARN: Only works on a sorted array
pub fn binary_search(nums: &mut Vec<i32>, target: i32) -> i32 {
    nums.sort();
    println!("sorted: {:?}", nums);
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut index = -1;

    while low <= high {
        let mid = ((high - low) / 2) + low;

        if target == nums[mid] {
            index = mid as i32;
            break;
        }

        if target > nums[mid] {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return index;
}
