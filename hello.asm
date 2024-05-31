  .text
  .global getint
getint:
  addi	sp, sp, -0

getint_body:

getint_ret:
  addi	sp, sp, 0
  ret

  .text
  .global getch
getch:
  addi	sp, sp, -0

getch_body:

getch_ret:
  addi	sp, sp, 0
  ret

  .text
  .global getarray
getarray:
  addi	sp, sp, -0

getarray_body:

getarray_ret:
  addi	sp, sp, 0
  ret

  .text
  .global putint
putint:
  addi	sp, sp, -0

putint_body:

putint_ret:
  addi	sp, sp, 0
  ret

  .text
  .global putch
putch:
  addi	sp, sp, -0

putch_body:

putch_ret:
  addi	sp, sp, 0
  ret

  .text
  .global putarray
putarray:
  addi	sp, sp, -0

putarray_body:

putarray_ret:
  addi	sp, sp, 0
  ret

  .text
  .global starttime
starttime:
  addi	sp, sp, -0

starttime_body:

starttime_ret:
  addi	sp, sp, 0
  ret

  .text
  .global stoptime
stoptime:
  addi	sp, sp, -0

stoptime_body:

stoptime_ret:
  addi	sp, sp, 0
  ret

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

