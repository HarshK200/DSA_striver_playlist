#include <iostream>
#include <vector>

int main() {

    // std::vector<int> v = {1, 3, 5, 8};
    // std::cout << "The vector" << std::endl;
    //
    // // verbose version
    // for (std::vector<int>::iterator it = v.begin(); it != v.end(); it++) {
    //     std::cout << *(it) << std::endl;
    // }
    //
    // // small version
    // auto toDelete = v.begin();
    //
    // v.erase(toDelete + 1); // this should delete 3 i.e. at the address
    // // of 2
    //
    // std::cout << "deleting multiple from the vector: " << std::endl;
    // auto beginDelit = v.begin() + 1;
    // auto endDelit = v.end() - 1;
    // //
    // v.erase(beginDelit, endDelit); // this should delete 3 and 5
    //
    // // The below line inserts 300 at the very starting of the vector
    // v.insert(v.begin(), 300);
    //
    // // inserting multiple elements in the vector
    // v.insert(v.begin() + 1, 2, 69); // this results in { 100, 69, 69, 100 }

    std::vector<int> v(2, 100);

    std::vector<int> insertInto = {23, 1, 3, 21};

    // assuming i wanna insert the v vector in to the above vector
    // i would have to write something like this
    insertInto.insert(insertInto.begin() + 1, v.begin(), v.end());

    for (auto it = insertInto.begin(); it != insertInto.end(); it++) {
        std::cout << *(it) << std::endl;
    }

    return 0;
}
