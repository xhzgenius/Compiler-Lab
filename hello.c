int x = 0;

void add(int y) {
    x = x+y;
    add(y-1);
}

int main() {
    int a;
    int b;
    b = a;
    return b+1;
    add(10);
    return x;
}