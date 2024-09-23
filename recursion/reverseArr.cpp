#include <iostream>
#include <utility>

void reverse(int arr[], int i, int n) {
    // base case
    if (i >= n)
        return;

    // pre recursion
    std::swap(arr[i], arr[n - i - 1]);

    // recursive case
    reverse(arr, i + 1, n);
}

int main() {
    int arr[4] = {12, 1, 32, 69};
    int arrSize = 4;

    for (auto n : arr) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    reverse(arr, 0, arrSize);

    for (auto n : arr) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    return 0;
}
