#include <climits>
#include <iostream>

bool palindrome() {
    int x = -121;

    int cpX = x;
    int reverse = 0;

    while (cpX != 0) {
        if (reverse > INT_MAX || reverse < INT_MIN)
            return false;
        reverse = (reverse * 10) + cpX % 10;
        cpX /= 10;
    }

    if (x == reverse)
        return true;

    return false;
}

int main() {

    std::cout << palindrome();

    return 0;
}
