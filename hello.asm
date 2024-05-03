  .text
  .global main
main:
  addi sp, sp, -16
  li	t0, 10
  sw t0, 0(sp)
  lw t1, 0(sp)
  li	t2, 466
  add	t1, t1, t2
  sw t1, 0(sp)
  lw t2, 0(sp)
  mv a0, t2
  ret
