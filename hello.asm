  .data
x:
  .word 0

  .text
  .global add
add:
  addi	sp, sp, 32
  sw	ra, 28(sp)

.add_body:
# Alloc(Alloc)
# Store(Store { value: Value(1073741824), dest: Value(1073741825) })
  mv	t0, a0
# Load(Load { src: Value(1073741825) })
  mv	t1, t0
# Binary(Binary { op: Sub, lhs: Value(1073741827), rhs: Value(1073741828) })
  li	t2, 1
  sub	t2, t1, t2
# Call(Call { callee: Function(9), args: [Value(1073741829)] })
  sw	t0, 0(sp)
  sw	t1, 8(sp)
  sw	t2, 12(sp)
  lw	a0, 12(sp)
  call	add
# Load(Load { src: Value(2) })
  la	t0, x
  lw	t0, 0(t0)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741830), rhs: Value(1073741831) })
  add	t2, a0, t1
# Store(Store { value: Value(1073741832), dest: Value(2) })
  mv	t0, t2
# Return(Return { value: Some(Value(1073741834)) })
  li	a0, 1
  j	.add_ret

.add_ret:
# Save global variables.
  la	t3, x
  sw	t0, 0(t3)
  lw	ra, 28(sp)
  addi	sp, sp, -32
  ret

  .global main
main:
  addi	sp, sp, 16
  sw	ra, 12(sp)

.main_body:
# Call(Call { callee: Function(9), args: [Value(1073741836)] })
  li	a0, 10
  call	add
# Load(Load { src: Value(2) })
  la	t0, x
  lw	t0, 0(t0)
  mv	t1, t0
# Return(Return { value: Some(Value(1073741838)) })
  mv	a0, t1
  j	.main_ret

.main_ret:
# Save global variables.
  la	t1, x
  sw	t0, 0(t1)
  lw	ra, 12(sp)
  addi	sp, sp, -16
  ret

