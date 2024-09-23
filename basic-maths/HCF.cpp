#include <iostream>
#include <vector>

std::vector<int> lcmAndGcd(long long A, long long B) {
    // code here
    int smallerNum;
    int largerNum;
    if (A < B) {
        smallerNum = A;
        largerNum = B;
    } else {
        smallerNum = B;
        largerNum = A;
    }
    int tempA = A;
    int tempB = B;

    int GCD;
    while (tempA != 0 && tempB != 0) {
        if (tempA > tempB)
            tempA %= tempB;
        else
            tempB %= tempA;
    }
    if (tempA == 0)
        GCD = tempB;
    else
        GCD = tempA;

    int LCM = largerNum;
    while (true) {
        if (LCM % A == 0 && LCM % B == 0) {
            break;
        }
        LCM++;
    }

    return {LCM, GCD};
}

int main() {
    std::cout << "LCM: " << lcmAndGcd(6, 12).front() << std::endl;
    std::cout << "HCF/GCD: " << lcmAndGcd(6, 12).back() << std::endl;

    return 0;
}
