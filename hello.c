

const int x = 1;

int half()
{
  return x / 2;
}

int x;

int f() { return x / 2; }

int main()
{
  int main = 514;
  while (main)
    main = x + main;
  // return half(10);
  return main - x;
}