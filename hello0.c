

const int x = 1;

int half(int x0, int x1, int x2, int x3, int x4, int x5, int x6, int x7, int x8, int x9, int x10)
{
  return x10 / 2;
}

int x;

int f() { return x / 2; }

int main()
{
  int tmp1 = -1, tmp2 = -2, tmp3 = -3, tmp4 = -4, tmp5 = -5, tmp6 = -6, 
      tmp7 = -7, tmp8 = -8, tmp9 = -9, tmp10 = -10;
  int a0 = half(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, tmp10);
  int tmp11 = tmp10+x;
  return a0;
  // return main - x;
}