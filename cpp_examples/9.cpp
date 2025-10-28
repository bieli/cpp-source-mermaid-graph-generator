#include <initializer_list>
using namespace std;

int sum(initializer_list<int> nums) {
    int total = 0;
    for (int n : nums) total += n;
    return total;
}

main() {
    int s = sum({1, 2, 3});
    if (s == 6) {
        exit(0);
    }
    return s;
}

