// mod largest_element;
// mod second_largest;
// mod is_arr_sorted;
// mod rotate_array;
// mod rotated_sorted;
// mod remove_duplicates;
// mod move_zeros;
// mod missing_numbers;
// mod union_of_sorted_arr;
// mod maximum_consicutive_ones;
// mod single_number;
// mod longest_subarray_with_sum_k;
// mod two_sum;
// mod binary_serarch;
// mod sort_arr_0_1_2;
// mod majority_element;
// mod maximum_subarray_sum;
// mod buy_sell_stock;
mod rearrange_by_sign;

fn main() {
    let nums = vec![3,1,-2,-5,2,-4];
    println!("{:?}", nums);

    let ans = rearrange_by_sign::rearrange_ele_by_sign_optimal(nums);
    println!("rearranged: {:?}", ans);
}
