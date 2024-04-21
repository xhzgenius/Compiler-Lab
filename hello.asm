  .text
  .global main
main:
  li	t0, 4
  li	t1, 5
  mul	t0, t0, t1
  li	t1, 6
  div	t0, t0, t1
  li	t1, 3
  add	t0, t1, t0
  li	t1, 2
  mul	t0, t1, t0
  li	t1, 1
  add	t0, t1, t0
  li	t1, 7
  li	t2, 8
  mul	t1, t1, t2
  add	t0, t0, t1
  li	t1, 9
  sub	t0, t0, t1
  mv a0, t0
  ret
