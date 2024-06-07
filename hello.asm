  .data
var:
  .word 0

  .text
  .global main
main:
  addi	sp, sp, -32
  sw	ra, 28(sp)

.main_body:
  j	.bb0_while_start

.bb0_while_start:
  la	t0, var
  lw	t0, 0(t0)
  mv	t1, t0
  mv	t2, zero
  xor	t3, t1, zero
  snez	t3, t3
  bnez	t3, .bb3_LAnd_if_block
  j	.bb4_LAnd_if_block_end

.bb1_while_body:
  j	.bb0_while_start

.bb2_while_end:
  mv	t4, t0
  mv	a0, t4
  j	.main_ret

.bb3_LAnd_if_block:
  li	t4, 1
  mv	t2, t4
  j	.bb4_LAnd_if_block_end

.bb4_LAnd_if_block_end:
  mv	t4, t2
  bnez	t4, .bb1_while_body
  j	.bb2_while_end

.main_ret:
  la	t5, var
  sw	t0, 0(t5)
  lw	ra, 28(sp)
  addi	sp, sp, 32
  ret

