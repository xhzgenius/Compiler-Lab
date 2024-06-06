  .data
x:
  .zero 4

x:
  .zero 4

  .text
  .global half
half:
  addi	sp, sp, -16

.half_body:
  la	t0, x
  li	t1, 2
  div	t1, t0, t1
  mv	a0, t1
  j	.half_ret

.half_ret:
  addi	sp, sp, 16
  ret

  .global half
half:
  addi	sp, sp, -16

.half_body:
  la	t0, x
  li	t1, 2
  div	t1, t0, t1
  mv	a0, t1
  j	.half_ret

.half_ret:
  addi	sp, sp, 16
  ret

  .global f
f:
  addi	sp, sp, -0

.f_body:
  j	.f_ret

.f_ret:
  addi	sp, sp, 0
  ret

  .global main
main:
  addi	sp, sp, -48

.main_body:
  li	t0, 514
  sw	t0, 0(sp)
  j	.bb0_while_start

.bb0_while_start:
  lw	t0, 0(sp)
  bnez	t0, .bb1_while_body
  j	.bb2_while_end

.bb1_while_body:
  la	t1, x
  lw	t2, 0(sp)
  add	t3, t1, t2
  sw	t3, 0(sp)
  j	.bb0_while_start

.bb2_while_end:
  lw	t4, 0(sp)
  la	t5, x
  sub	x31, t4, t5
  mv	a0, x31
  j	.main_ret

.main_ret:
  addi	sp, sp, 48
  ret

