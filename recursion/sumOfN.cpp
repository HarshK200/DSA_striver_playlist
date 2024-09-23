#include <iostream>

int sumOfN(int n) {
    // base case
    if (n == 1)
        return 1;

    return n + sumOfN(n - 1);
}

int main() {
    int n = 4;

    std::cout << sumOfN(n) << std::endl;

    return 0;
}
