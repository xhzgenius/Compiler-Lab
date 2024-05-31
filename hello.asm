  .text
  .global main
main:
  addi	sp, sp, -16
  li	t0, 2
  sw	t0, 0(sp)
  lw	t0, 0(sp)
  bnez	t0, bb1_if_block_1
  j	bb0_if_block_end

bb1_if_block_1:
  li	t1, 5
  sw	t1, 0(sp)
  j	bb0_if_block_end

bb0_if_block_end:
  lw	t1, 0(sp)
  mv	a0, t1
  addi	sp, sp, 16
  ret
