  .data
a:
  .zero 4

b:
  .zero 4

  .text
  .global main
main:
  addi	sp, sp, 48
  sw	ra, 44(sp)

.main_body:
# Store(Store { value: Value(1073741824), dest: Value(2) })
  li	t0, 10
  mv	t0, t0
# Store(Store { value: Value(1073741826), dest: Value(4) })
  li	t1, 5
  mv	t1, t1
# Alloc(Alloc)
# Load(Load { src: Value(2) })
  mv	t2, t0
# Binary(Binary { op: Mul, lhs: Value(1073741829), rhs: Value(1073741830) })
  li	t3, 2
  mul	t3, t2, t3
# Load(Load { src: Value(4) })
  mv	t4, t1
# Binary(Binary { op: Add, lhs: Value(1073741831), rhs: Value(1073741832) })
  add	t5, t3, t4
# Binary(Binary { op: Add, lhs: Value(1073741833), rhs: Value(1073741834) })
  la	x31, a
  sw	t0, 0(x31)
  li	t0, 3
  add	t0, t5, t0
# Store(Store { value: Value(1073741835), dest: Value(1073741828) })
  la	x31, b
  sw	t1, 0(x31)
  mv	t1, t0
# Load(Load { src: Value(1073741828) })
  sw	t2, 8(sp)
  mv	t2, t1
# Return(Return { value: Some(Value(1073741837)) })
  mv	a0, t2
  j	.main_ret

.main_ret:
# Save global variables.
  lw	ra, 44(sp)
  addi	sp, sp, -48
  ret

