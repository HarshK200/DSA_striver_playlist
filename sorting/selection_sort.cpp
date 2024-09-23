#include <iostream>
#include <utility>

// NOTE: Arrays are passed in my refence by default
void selection_sort(int arr[], int n) {
    // NOTE: going till n-1 since last element will always be sorted
    for (int i = 0; i < n - 1; i++) {
        int minimum = i;

        for (int j = i; j < n; j++) {
            // std::cout << arr[i] << "\t" << arr[j] << std::endl;
            if (arr[j] < arr[minimum]) {
                minimum = j;
            }
        }

        std::swap(arr[minimum], arr[i]);
    }
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

    selection_sort(arr, n);

    for (int i = 0; i < n; i++) {
        std::cout << arr[i] << " ";
    }

    return 0;
}
