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
  sw	a0, 0(sp)
# Load(Load { src: Value(2) })
  la	t1, x
  lw	t0, 0(t1)
# Load(Load { src: Value(1073741825) })
  lw	t1, 0(sp)
# Binary(Binary { op: Add, lhs: Value(1073741827), rhs: Value(1073741828) })
  add	t2, t0, t1
# Store(Store { value: Value(1073741829), dest: Value(2) })
  la	t3, x
  sw	t2, 0(t3)
# Load(Load { src: Value(1073741825) })
  lw	t3, 0(sp)
# Binary(Binary { op: Sub, lhs: Value(1073741831), rhs: Value(1073741832) })
  li	t4, 1
  sub	t4, t3, t4
# Call(Call { callee: Function(9), args: [Value(1073741833)] })
  sw	t0, 8(sp)
  sw	t1, 12(sp)
  sw	t2, 16(sp)
  sw	t3, 20(sp)
  sw	t4, 24(sp)
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
# Store(Store { value: Value(1073741838), dest: Value(1073741837) })
  sw	t0, 8(sp)
# Load(Load { src: Value(1073741837) })
  lw	t1, 8(sp)
# Binary(Binary { op: Add, lhs: Value(1073741840), rhs: Value(1073741841) })
  li	t2, 1
  add	t2, t1, t2
# Return(Return { value: Some(Value(1073741842)) })
  mv	a0, t2
  j	.main_ret

.main_ret:
  lw	ra, 28(sp)
  addi	sp, sp, 32
  ret

