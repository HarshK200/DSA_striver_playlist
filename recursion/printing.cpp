#include <iostream>

void print(int n) {
    // base case
    if (n <= 0)
        return;

    // recursive case
    print(n - 1);
    std::cout << n << " ";
}

int main() {
    int target = 12;

    print(target);

    return 0;
}
