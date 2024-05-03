  .text
  .global main
main:
  addi	sp, sp, -32
  li	t0, 1
  sw	t0, 0(sp)
  li	t0, -123
  sw	t0, 16(sp)
  li	t0, 2
  sw	t0, 0(sp)
  li	t0, 3
  sw	t0, 16(sp)
  lw	t0, 16(sp)
  li	t1, 114514
  add	t0, t0, t1
  lw	t1, 16(sp)
  mv	a0, t1
  addi	sp, sp, 32
  ret
