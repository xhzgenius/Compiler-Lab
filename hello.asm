  .text
  .global main
main:
  addi	sp, sp, -32
  li	t0, 1
  sw	t0, 0(sp)
  li	t0, -123
  sw	t0, 16(sp)
  lw	t0, 16(sp)
  li	t1, 1
  sub	t0, t0, t1
  lw	t1, 16(sp)
  li	t2, 2
  sw	t2, 0(sp)
  li	t2, 3
  sw	t2, 16(sp)
  lw	t2, 16(sp)
  li	t3, 114514
  add	t2, t2, t3
  lw	t3, 16(sp)
  mv	a0, t3
  addi	sp, sp, 32
  ret
