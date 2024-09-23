#include <iostream>

int main() {
    int n;
    std::cout << "Enter the size of the array: ";
    std::cin >> n; // size of the array

    std::cout << "Enter the values for the array: ";
    int arr[n];
    for (int i = 0; i < n; i++) {
        std::cin >> arr[i];
    }

    // pre-compute
    int hash[13] = {0}; // array filled with 0's of size 13
    for (int i = 0; i < n; i++) {
        hash[arr[i]] += 1;
    }

    int q;
    std::cin >> q;
    // fetching
    while (q--) {
        int number;
        std::cin >> number;
        std::cout << hash[number] << "\n";
    }

    return 0;
}
