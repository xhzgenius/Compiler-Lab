  .text
  .global main
main:
  addi	sp, sp, -32
  li	t0, 1
  sw	t0, 0(sp)
  li	t0, 2
  sw	t0, 8(sp)
  li	t0, 3
  sw	t0, 16(sp)
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t0, 0(sp)
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  lw	t1, 0(sp)
  add	t0, t0, t1
  lw	t1, 8(sp)
  add	t0, t0, t1
  lw	t1, 16(sp)
  add	t0, t0, t1
  li	a0, 0
  j	main_ret

main_ret:
  addi	sp, sp, 32
  ret
