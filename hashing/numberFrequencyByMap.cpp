#include <iostream>
#include <map>

int main() {
    int n;
    std::cout << "Enter the size of the array: ";
    std::cin >> n; // size of the array

    // taking array values as input
    std::cout << "Enter the values for the array: ";
    int arr[n];
    for (int i = 0; i < n; i++) {
        std::cin >> arr[i];
    }

    // pre-compute
    std::map<int, int> mpp;
    for (int i = 0; i < n; i++) {
        mpp[arr[i]] += 1;
    }

    // fetch
    int q;
    std::cout << "how many values do you want to fetch: ";
    std::cin >> q;
    while (q--) {
        int number;
        std::cout << "value for number: ";
        std::cin >> number;
        std::cout << "value:  " << mpp[number] << std::endl;
    }

    return 0;
}
