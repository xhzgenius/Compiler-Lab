fun @main(): i32 {
%entry:
  @x = alloc i32
  store 10, @x
  %0 = load @x
  %1 = add %0, 1
  store %1, @x
  %2 = load @x
  ret %2
}
