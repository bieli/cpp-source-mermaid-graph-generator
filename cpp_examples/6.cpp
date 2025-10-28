#include <iostream>
#include <vector>
using namespace std;

main() {
    vector<int> data = {1, 2, 3, 4};
    auto sum = [](const vector<int>& v) {
        int total = 0;
        for (auto x : v) total += x;
        return total;
    };
    int result = sum(data);
    if (result > 5) {
        exit(42);
    }
    cout << result;
}

