main() {
    int x = getValue();
    if (x < 0) {
        exit(1);
    }
    int y = process(x);
    if (y == 0) {
        return -1;
    }
    finalize(y);
}

