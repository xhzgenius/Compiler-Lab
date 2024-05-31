  .text
  .global main
main:
  addi	sp, sp, -0

main_body:
  j	bb0_while_start

bb0_while_start:
  li	t0, 1
  bnez	t0, bb1_while_body
  j	bb2_while_end

bb1_while_body:
  j	bb2_while_end

bb2_while_end:
  mv	a0, zero
  j	main_ret

main_ret:
  addi	sp, sp, 0
  ret

