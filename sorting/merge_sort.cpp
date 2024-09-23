#include <iostream>

// NOTE: the merge algorithm
void merge(int *arr, int high, int mid, int low) {
    // std::cout << "low: " << low << " middle: " << mid << " high: " << high <<
    // std::endl;
    int *temp = new int[high + 1];
    int left = low;
    int right = mid + 1;
    int i = 0;

    while (left <= mid && right <= high) {
        if (arr[left] <= arr[right]) {
            temp[i] = arr[left];
            left++;
        } else {
            temp[i] = arr[right];
            right++;
        }
        i++;
    }

    while (left <= mid) {
        temp[i] = arr[left];
        left++;
        i++;
    }

    while (right <= mid) {
        temp[i] = arr[right];
        right++;
        i++;
    }

    for (int i = low; i <= high; i++) {
        arr[i] = temp[i - low];
    }
    delete[] temp;
}

// NOTE: better time complexity compared to bubble and insertion sort
void merge_sort(int arr[], int low, int high) {
    // base case
    if (low >= high)
        return;

    //  NOTE: calcualting middle
    int middle = ((high - low) / 2) + low;

    // recursive case  NOTE: recurse left half
    merge_sort(arr, low, middle);
    // NOTE: recurse right half
    merge_sort(arr, middle + 1, high);

    // NOTE: post recursive step
    merge(arr, high, middle, low);
}

int main() {
    int n;
    std::cout << "enter the size of the array: ";
    std::cin >> n;
    int arr[n];

    std::cout << "enter the elements of the array:\n";
    for (int i = 0; i < n; i++) {
        std::cin >> arr[i];
    }

    merge_sort(arr, 0, n - 1);

    for (int i = 0; i < n; i++) {
        std::cout << arr[i] << " ";
    }

    return 0;
}
