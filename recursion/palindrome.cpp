#include <iostream>

bool isPalindrome(int i, std::string& s, int n) {

    // base case
    if (i >= n/2) {
        return true;
    }

    // if the string is not palindrome
    if (s[i] != s[n - i - 1]) {
        return false;
    }

    return isPalindrome(i + 1, s, n);
}

int main() {

    // std::string s = "dapp";
    std::string s = "pop";

    std::cout << "is Palindrome: " << isPalindrome(0, s, s.size()) << std::endl;

    return 0;
}
