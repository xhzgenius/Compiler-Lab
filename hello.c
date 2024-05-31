int var;

int var = 1;

const int con = 114;

int func(int x) {
  x = 3;
  var = var + x;
  return var;
}

int main() {
  // putint 和 putch 都是 SysY 库函数
  // SysY 要求库函数不声明就可以使用
  putint(func(1));
  var = func(var);
  putint(func(2));
  putch(10);
  return var;
}