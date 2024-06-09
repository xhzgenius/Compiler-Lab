  .data
a:
  .zero 8

  .text
  .global func
func:
  addi	sp, sp, -48
  sw	ra, 44(sp)

.func_body:
# Alloc(Alloc)
# Store(Store { value: Value(1073741824), dest: Value(1073741825) })
  mv	t0, a0
# GetElemPtr(GetElemPtr { src: Value(2), index: Value(1073741827) })
  li	t1, 0
  la	t2, a
  li	x31, 4
  mul	x31, t1, x31
  add	t2, t2, x31
# Store(Store { value: Value(1073741829), dest: Value(1073741828) })
  li	t1, 1
  sw	t1, 0(t2)
# GetElemPtr(GetElemPtr { src: Value(2), index: Value(1073741831) })
  li	t3, 0
  la	t4, a
  li	x31, 4
  mul	x31, t3, x31
  add	t4, t4, x31
# Load(Load { src: Value(1073741832) })
  lw	t3, 12(sp)
  lw	t3, 0(t4)
# Binary(Binary { op: Sub, lhs: Value(1073741834), rhs: Value(1073741833) })
  li	t5, 3
  sub	t3, t5, t3
# Load(Load { src: Value(1073741825) })
  mv	t5, t0
# GetPtr(GetPtr { src: Value(1073741836), index: Value(1073741835) })
  li	x31, 4
  mul	x31, t3, x31
  add	t5, t5, x31
# Load(Load { src: Value(1073741837) })
  lw	t3, 28(sp)
  lw	t3, 0(t5)
# Return(Return { value: Some(Value(1073741838)) })
  mv	a0, t3
# Save global variables.
# Save local variables.
  sw	t0, 0(sp)
  j	.func_ret

.func_ret:
  lw	ra, 44(sp)
  addi	sp, sp, 48
  ret

  .global main
main:
  addi	sp, sp, -48
  sw	ra, 44(sp)

.main_body:
# Alloc(Alloc)
# Alloc(Alloc)
# GetElemPtr(GetElemPtr { src: Value(1073741841), index: Value(1073741842) })
  li	t0, 0
  addi	t1, sp, 0
  li	x31, 4
  mul	x31, t0, x31
  add	t1, t1, x31
# Store(Store { value: Value(1073741844), dest: Value(1073741843) })
  li	t0, -1
  sw	t0, 0(t1)
# GetElemPtr(GetElemPtr { src: Value(1073741841), index: Value(1073741846) })
  li	t2, 1
  addi	t3, sp, 0
  li	x31, 4
  mul	x31, t2, x31
  add	t3, t3, x31
# Store(Store { value: Value(1073741848), dest: Value(1073741847) })
  li	t2, 4
  sw	t2, 0(t3)
# GetElemPtr(GetElemPtr { src: Value(1073741841), index: Value(1073741850) })
  li	t4, 2
  addi	t5, sp, 0
  li	x31, 4
  mul	x31, t4, x31
  add	t5, t5, x31
# Store(Store { value: Value(1073741852), dest: Value(1073741851) })
  li	t4, 8
  sw	t4, 0(t5)
# GetElemPtr(GetElemPtr { src: Value(1073741841), index: Value(1073741854) })
  sw	t1, 8(sp)
  li	t1, 0
  addi	t0, sp, 0
  li	x31, 4
  mul	x31, t1, x31
  add	t0, t0, x31
# Call(Call { callee: Function(9), args: [Value(1073741855)] })
  mv	a0, t0
  sw	t3, 12(sp)
  sw	t5, 16(sp)
# Save global variables.
  call	func
  mv	t0, a0
# Store(Store { value: Value(1073741856), dest: Value(1073741840) })
  mv	t1, t0
# Load(Load { src: Value(1073741840) })
  mv	t0, t1
# GetElemPtr(GetElemPtr { src: Value(1073741841), index: Value(1073741859) })
  li	t2, 1
  addi	t3, sp, 0
  li	x31, 4
  mul	x31, t2, x31
  add	t3, t3, x31
# Load(Load { src: Value(1073741860) })
  lw	t2, 36(sp)
  lw	t2, 0(t3)
# Binary(Binary { op: Add, lhs: Value(1073741858), rhs: Value(1073741861) })
  add	t0, t0, t2
# Return(Return { value: Some(Value(1073741862)) })
  mv	a0, t0
# Save global variables.
# Save local variables.
  sw	t1, 4(sp)
  j	.main_ret

.main_ret:
  lw	ra, 44(sp)
  addi	sp, sp, 48
  ret

