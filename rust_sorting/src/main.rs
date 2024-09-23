mod quick_sort;
mod take_input_num;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1"); // WARN: for debuging purpose

    println!("Enter the size of the arr: ");
    let length = take_input_num::take_num_input();

    let mut arr = Vec::new();
    for _ in 0..length {
        arr.push(take_input_num::take_num_input());
    }

    println!("unsorted array: {:?} ", arr);
    let result = quick_sort::quick_sort(&mut arr);

    match result {
        Ok(..) => {
            println!("sorted array: {:?} ", arr);
        }
        Err(e) => {
            println!("Some err occured: {}", e);
        }
    }
}
