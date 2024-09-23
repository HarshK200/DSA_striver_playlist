fn sort_partition(arr: &mut Vec<u32>, low: usize, high: usize) -> Result<usize, String> {
    let pivot_value = arr[high];
    let mut pivot_index = low as i32 - 1;

    // NOTE: moving everything smaller than pivot to the left
    for j in low..high {
        if arr[j] <= pivot_value {
            pivot_index += 1;
            arr.swap(pivot_index as usize, j);
        }
    }
    // NOTE: putting pivot at it's rightful place
    pivot_index += 1;
    arr.swap(pivot_index as usize, high);

    return Ok(pivot_index as usize);
}

fn qs(arr: &mut Vec<u32>, low: usize, high: usize) -> Result<(), String> {
    // NOTE: base case
    if low >= high {
        return Ok(());
    }

    // NOTE: weak sorting the array and finding the pivot
    let pivot = sort_partition(arr, low, high)?;

    // NOTE: recursive left side
    let _ = qs(arr, 0, pivot - 1);

    // NOTE: recursive rigth side
    let _ = qs(arr, pivot + 1, high);

    Ok(())
}

pub fn quick_sort(arr: &mut Vec<u32>) -> Result<(), String> {
    qs(arr, 0, arr.len() - 1)?;
    return Ok(());
}
