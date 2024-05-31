int var;

int func(int x) {
  var = var + x;
  return var;
}

int main() {
  // putint 和 putch 都是 SysY 库函数
  // SysY 要求库函数不声明就可以使用
  putint(func(1));
  var = var * 10;
  putint(func(2));
  putch(10);
  return var;
}