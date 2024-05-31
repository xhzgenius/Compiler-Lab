  .text
  .global main
main:
  addi	sp, sp, -16
  li	t0, 2
  sw	t0, 0(sp)
  lw	t0, 0(sp)
  bnez	t0, bb1_if_block_1
  j	bb2_if_block_2

bb1_if_block_1:
  lw	t1, 0(sp)
  li	t2, 1
  add	t1, t1, t2
  sw	t1, 0(sp)
  j	bb0_if_block_end

bb2_if_block_2:
  sw	x0, 0(sp)
  j	bb0_if_block_end

bb0_if_block_end:
  lw	t1, 0(sp)
  mv	a0, t1
  j	main_ret

main_ret:
  addi	sp, sp, 16
  ret
