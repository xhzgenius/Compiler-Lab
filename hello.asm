  .text
  .global main
main:
  addi	sp, sp, -16
  li	t0, 1
  sw	t0, 0(sp)
  lw	t0, 0(sp)
  li	t1, 1
  add	t0, t1, t0
  li	a0, 114
  addi	sp, sp, 16
  ret
