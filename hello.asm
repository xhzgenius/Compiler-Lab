  .text
  .global main
main:
  addi	sp, sp, -80

main_body:
  li	t0, 12345
  sw	t0, 0(sp)
  lw	t0, 0(sp)
  sw	zero, 12(sp)
  xor	t1, t0, zero
  snez	t1, t1
  bnez	t1, bb0_LAnd_if_block
  j	bb1_LAnd_if_block_end

bb0_LAnd_if_block:
  sw	zero, 12(sp)
  j	bb1_LAnd_if_block_end

bb1_LAnd_if_block_end:
  lw	t2, 12(sp)
  li	t3, 1
  sw	t3, 28(sp)
  xor	t3, t2, zero
  seqz	t3, t3
  bnez	t3, bb2_LOr_if_block
  j	bb3_LOr_if_block_end

bb2_LOr_if_block:
  lw	t4, 0(sp)
  xor	t5, zero, t4
  snez	t5, t5
  sw	t5, 28(sp)
  j	bb3_LOr_if_block_end

bb3_LOr_if_block_end:
  lw	x31, 28(sp)
  li	a0, 1
  sw	a0, 52(sp)
  xor	a0, x31, zero
  seqz	a0, a0
  bnez	a0, bb4_LOr_if_block
  j	bb5_LOr_if_block_end

bb4_LOr_if_block:
  lw	a1, 0(sp)
  xor	a2, zero, a1
  snez	a2, a2
  xor	a3, zero, a2
  snez	a3, a3
  sw	a3, 52(sp)
  j	bb5_LOr_if_block_end

bb5_LOr_if_block_end:
  lw	a4, 52(sp)
  mv	a0, a4
  j	main_ret

main_ret:
  addi	sp, sp, 80
  ret
