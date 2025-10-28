#include <vector>
using namespace std;

main() {
    vector<int> nums = {1, 2, 3, 4};
    for (int n : nums) {
        if (n == 3) {
            exit(3);
        }
    }
    return 0;
}

