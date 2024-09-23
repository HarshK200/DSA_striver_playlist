#include <iostream>

int main() {
    // reading user input
    int numToCount;
    std::cout << "Enter a number: ";
    std::cin >> numToCount;

    int cpNumToCount = numToCount;
    // counting the no. of digits

    int count = 0;
    for (count = 0; cpNumToCount != 0; count++) {
        cpNumToCount /= 10;
    }

    std::cout << "The digits in no. " << numToCount << " is " << count;
    std::cout << std::endl;

    return 0;
}
