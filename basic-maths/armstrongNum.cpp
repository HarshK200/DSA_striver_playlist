#include <cmath>
#include <iostream>

// since n is passed by value i.e. a copy of it we can modify n directly (no
// need to make a copy)
bool armstrong(int n) {
    int nCopy = n;
    int sum = 0;
    int count = 0;

    while (nCopy != 0) {
        count++;
        nCopy /= 10;
    }
    nCopy = n;

    while (nCopy != 0) {
        sum += pow((nCopy % 10), count);
        nCopy /= 10;
    }

    if (sum == n) {
        return true;
    }

    return false;
}

int main() {

    std::cout << armstrong(371) << std::endl;

    return 0;
}
