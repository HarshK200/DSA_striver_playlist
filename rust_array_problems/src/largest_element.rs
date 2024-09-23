pub fn largest_element() {
    let arr: [i32; 6] = [3, 12, 1, 4, 69, 8];

    let mut largest = arr[0];
    
    for i in 1..arr.len() {
        if largest < arr[i] {
            largest = arr[i];
        }
    }


    println!("Largest element in arr is: {}", largest);
}
