#include <climits>
#include <iostream>

int main() {
    // reading user input
    int n;
    std::cout << "Enter a number: ";
    std::cin >> n;

    int cpN = n;

    // reverseing the no.
    int reverse = 0;
    while (cpN != 0) {
        if (reverse > INT_MAX || reverse < INT_MIN)
            return false;
        reverse = (reverse * 10) + (cpN % 10);
        cpN /= 10;
    }

    std::cout << "The reverse of the number " << n << " is " << reverse;
    std::cout << std::endl;

    return 0;
}
