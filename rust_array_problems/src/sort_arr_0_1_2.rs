use core::panic;
use std::usize;

// NOTE: Just sort the array that you are good to go
// time complexity == time complexity of sort algorithm used
pub fn sort_arr_012s_bruteforce(nums: &mut Vec<i32>) {
    nums.sort();
}

// NOTE: time complexity O(2n)
pub fn sort_arr_012s_better(nums: &mut Vec<i32>) {
    let mut count0 = 0;
    let mut count1 = 0;
    let mut count2 = 0;

    for i in nums.iter() {
        match *i {
            0 => count0 += 1,
            1 => count1 += 1,
            2 => count2 += 1,
            x => {
                panic!("unexpected value {}", x)
            }
        }
    }

    // println!("0: {}, 1: {}, 2: {}", count0, count1, count2);

    for i in 0..count0 {
        nums[i] = 0;
    }
    for i in 0..count1 {
        nums[count0 + i] = 1;
    }
    for i in 0..count2 {
        nums[count0 + count1 + i] = 2;
    }
}

// HACK:  this uses dutch national flag algorithm
pub fn sort_arr_012s_optimal(nums: &mut Vec<i32>) {
    let mut low = 0;
    let mut mid = 0 as i32;
    let mut high = (nums.len() - 1) as i32;

    while mid <= high {
        println!("mid: {mid}, high: {high}");
        match nums[mid as usize] {
            0 => {
                nums.swap(low, mid as usize);
                low += 1;
                mid += 1;
            }
            1 => {
                mid += 1;
            }
            2 => {
                nums.swap(mid as usize, high as usize);
                high -= 1;
            }
            _ => {}
        }
    }
}
