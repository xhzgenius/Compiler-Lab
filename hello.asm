  .text
  .global main
main:
  addi	sp, sp, -16
  li	t0, 10
  sw	t0, 0(sp)
  lw	t0, 0(sp)
  li	t1, 1
  add	t0, t0, t1
  sw	t0, 0(sp)
  lw	t0, 0(sp)
  mv	a0, t0
  addi	sp, sp, 16
  ret
