pub fn is_sorted() {
    let arr: [i32; 6] = [1, 2, 3, 4, 9, 8];

    let mut is_sorted = true;

    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            is_sorted = false;
            break;
        }
    }

    println!("is array sorted: {}", is_sorted);
}
