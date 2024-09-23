#include <iostream>

int factorial(int n) {
    // base case
    if (n == 1 || n == 0)
        return 1;

    return n * factorial(n - 1);
}

int main() {
    int n = 4;

    std::cout << factorial(n) << std::endl;

    return 0;
}
