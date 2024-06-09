  .data
a:
  .word 0
  .word 1
  .word 2
  .word 3
  .word 4
  .text
  .global main
main:
  addi	sp, sp, -16
  sw	ra, 12(sp)

.main_body:
# GetElemPtr(GetElemPtr { src: Value(7), index: Value(1073741824) })
  li	t0, 4
  la	t1, a
  li	x31, 4
  mul	x31, t0, x31
  add	t1, t1, x31
# Load(Load { src: Value(1073741825) })
  lw	t0, 4(sp)
  lw	t0, 0(t1)
# Return(Return { value: Some(Value(1073741826)) })
  mv	a0, t0
# Save global variables.
# Save local variables.
  j	.main_ret

.main_ret:
  lw	ra, 12(sp)
  addi	sp, sp, 16
  ret

