main() {
    int a = loadData();
    if (a == 0) {
        return 0;
    }
    int b = compute(a);
    if (b > 100) {
        return 1;
    }
    saveResult(b);
}

