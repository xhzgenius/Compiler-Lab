int x = 0;

int add(int y) {
    x = add(y-1)+x;
    return 1;
}

int main() {
    add(10);
    return x;
}