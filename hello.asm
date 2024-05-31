  .text
  .global main
main:
  addi	sp, sp, -16
  li	t0, 2
  sw	t0, 0(sp)
  lw	t0, 0(sp)
  xor	t0, x0, t0
  snez	t0, t0
  bnez	t0, bb1_if_block_1
  j	bb0_if_block_end

bb1_if_block_1:
  li	a0, 0
  j	main_ret

bb0_if_block_end:
  lw	t1, 0(sp)
  mv	a0, t1
  j	main_ret

main_ret:
  addi	sp, sp, 16
  ret
