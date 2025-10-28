main() {
    init();
    int status = checkStatus();
    if (status != 0) {
        exit(99);
    }
    int result = runTask();
    log(result);
}

