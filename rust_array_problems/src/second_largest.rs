pub fn second_largest_element() {
    let arr: [i32; 6] = [3, 12, 1, 4, 69, 8];

    let mut largest = arr[0];
    let mut second_largest: Option<i32> = None;

    for i in 1..arr.len() {
        if arr[i] > largest {
            second_largest = Some(largest);
            largest = arr[i];
        } else if arr[i] < largest && arr[i] > second_largest.unwrap_or(std::i32::MIN) {
            second_largest = Some(arr[i]);
        }
    }

    match second_largest {
        Some(ele) => {
            println!("Second largest element is: {}", ele);
        }
        None => {
            println!("There was no second largest element");
        }
    }
}
