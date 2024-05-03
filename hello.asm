  .text
  .global main
main:
  addi	sp, sp, -16
  li	t0, 1
  sw	t0, 0(sp)
  li	t0, 2
  sw	t0, 0(sp)
  li	t0, 3
  sw	t0, 8(sp)
  lw	t0, 8(sp)
  li	t1, 114514
  add	t0, t0, t1
  lw	t1, 0(sp)
  mv	a0, t1
  addi	sp, sp, 16
  ret
