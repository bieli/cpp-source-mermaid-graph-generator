#include <memory>
using namespace std;

int compute(int x) {
    return x * 2;
}

main() {
    unique_ptr<int> ptr = make_unique<int>(5);
    int result = compute(*ptr);
    if (result > 10) {
        return -1;
    }
    cout << result;
}

