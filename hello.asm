  .data
x:
  .word 0

  .text
  .global add
add:
  addi	sp, sp, -32
  sw	ra, 28(sp)

.add_body:
# Alloc(Alloc)
# Store(Store { value: Value(1073741824), dest: Value(1073741825) })
  mv	a0, t0
# Load(Load { src: Value(2) })
  la	t1, x
  lw	t1, 0(t1)
  mv	t1, t2
# Load(Load { src: Value(1073741825) })
  mv	t0, t3
# Binary(Binary { op: Add, lhs: Value(1073741827), rhs: Value(1073741828) })
  add	t4, t2, t3
# Store(Store { value: Value(1073741829), dest: Value(2) })
  mv	t4, t1
# Load(Load { src: Value(1073741825) })
  mv	t0, t5
# Binary(Binary { op: Sub, lhs: Value(1073741831), rhs: Value(1073741832) })
  li	x31, 1
  sub	x31, t5, x31
# Call(Call { callee: Function(9), args: [Value(1073741833)] })
  sw	t0, 0(sp)
  la	t0, x
  sw	t1, 0(t0)
  sw	t2, 8(sp)
  sw	t3, 12(sp)
  sw	t4, 16(sp)
  sw	t5, 20(sp)
  sw	x31, 24(sp)
  lw	a0, 24(sp)
  call	add
# Return(Return { value: None })
  j	.add_ret

.add_ret:
  lw	ra, 28(sp)
  addi	sp, sp, 32
  ret

  .global main
main:
  addi	sp, sp, -32
  sw	ra, 28(sp)

.main_body:
# Alloc(Alloc)
# Alloc(Alloc)
# Load(Load { src: Value(1073741836) })
  lw	t0, 0(sp)
  mv	t0, t1
# Store(Store { value: Value(1073741838), dest: Value(1073741837) })
  mv	t1, t2
# Load(Load { src: Value(1073741837) })
  mv	t2, t3
# Binary(Binary { op: Add, lhs: Value(1073741840), rhs: Value(1073741841) })
  li	t4, 1
  add	t4, t3, t4
# Return(Return { value: Some(Value(1073741842)) })
  mv	a0, t4
  j	.main_ret

.main_ret:
  lw	ra, 28(sp)
  addi	sp, sp, 32
  ret

