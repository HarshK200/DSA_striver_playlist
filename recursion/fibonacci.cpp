#include <iostream>

int fibonacci(int n) {
    // base cases
    if (n == 0 || n == 1) {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
    // how many first fibonacci numbers to print
    int n = 12;

    std::cout << fibonacci(n);

    return 0;
}
