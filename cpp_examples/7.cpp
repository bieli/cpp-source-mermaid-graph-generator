#include <iostream>
using namespace std;

void process(int* ptr) {
    if (ptr == nullptr) {
        exit(1);
    }
    cout << *ptr;
}

main() {
    int value = 10;
    int* p = &value;
    process(p);
}

