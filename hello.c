int main() {
  int a = 2;
  if (a) if (a+1)
  {
    // return;
    a = 114514;
  }  // 在实际写 C/C++ 程序的时候别这样, 建议 if 的分支全部带大括号
  else a = 114;
  return a;
}