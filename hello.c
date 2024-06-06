int x = 0;

void add(int y) {
    x = x+y;
    add(y-1);
}

int main() {
    add(10);
    return x;
}