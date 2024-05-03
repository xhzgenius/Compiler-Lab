int main() {
  int a = 1;
  {
    a = 2;
    int a = 3;
    a+114514;
  }
  return a;
}