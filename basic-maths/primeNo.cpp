#include <iostream>

bool isPrime(int n) {
    int count = 0;

    for (int i = 1; i * i <= n; i++) {

        if (n % i == 0) {
            count++;
            if (n / i != i) {
                count++;
            }
        }
    }

    return count == 2;
}

int main() {

    std::cout << isPrime(9) << std::endl;

    return 0;
}
