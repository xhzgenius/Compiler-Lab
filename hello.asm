  .data
x:
  .zero 4

  .text
  .global half
half:
  addi	sp, sp, -112
  sw	ra, 108(sp)

.half_body:
  sw	a0, 0(sp)
  sw	a1, 8(sp)
  sw	a2, 16(sp)
  sw	a3, 24(sp)
  sw	a4, 32(sp)
  sw	a5, 40(sp)
  sw	a6, 48(sp)
  sw	a7, 56(sp)
  lw	t0, 112(sp)
  sw	t0, 64(sp)
  lw	t1, 116(sp)
  sw	t1, 72(sp)
  lw	t2, 120(sp)
  sw	t2, 80(sp)
  lw	t3, 80(sp)
  li	t4, 2
  div	t4, t3, t4
  mv	a0, t4
  j	.half_ret

.half_ret:
  lw	ra, 108(sp)
  addi	sp, sp, 112
  ret

  .global f
f:
  addi	sp, sp, -16
  sw	ra, 12(sp)

.f_body:
  la	t0, x
  li	t1, 2
  div	t1, t0, t1
  mv	a0, t1
  j	.f_ret

.f_ret:
  lw	ra, 12(sp)
  addi	sp, sp, 16
  ret

  .global main
main:
  addi	sp, sp, -32
  sw	ra, 28(sp)

.main_body:
  li	t0, 514
  sw	t0, 12(sp)
  li	a0, 0
  li	a1, 1
  li	a2, 2
  li	a3, 3
  li	a4, 4
  li	a5, 5
  li	a6, 6
  li	a7, 7
  li	t0, 8
  sw	t0, 0(sp)
  li	t0, 9
  sw	t0, 4(sp)
  li	t0, 10
  sw	t0, 8(sp)
  call	half
  j	.main_ret

.main_ret:
  lw	ra, 28(sp)
  addi	sp, sp, 32
  ret

