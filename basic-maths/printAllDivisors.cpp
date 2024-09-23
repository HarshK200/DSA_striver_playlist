#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

int main() {
    int n = 36;
    std::vector<int> factors;

    int i = 1;
    // as long as the square of i is smaller than n
    // runs for O(sqrt(n))
    for (int i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            factors.push_back(i);
            if (n / i != i)
                factors.push_back(n / i);
        }
    }

    std::sort(factors.begin(), factors.end());

    for (auto it : factors)
        std::cout << it << " ";

    return 0;
}
