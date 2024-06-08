  .data
  .text
  .global fib
fib:
  addi	sp, sp, -64
  sw	ra, 60(sp)

.fib_body:
# Alloc(Alloc)
# Store(Store { value: Value(1073741824), dest: Value(1073741825) })
  mv	t0, a0
# Load(Load { src: Value(1073741825) })
  mv	t1, t0
# Binary(Binary { op: Eq, lhs: Value(1073741827), rhs: Value(1073741828) })
  li	t2, 0
  xor	t1, t1, t2
  seqz	t1, t1
# Branch(Branch { cond: Value(1073741829), true_bb: BasicBlock(3), false_bb: BasicBlock(2), true_args: [], false_args: [] })
# Save global variables.
# Save local variables.
  sw	t0, 0(sp)
  bnez	t1, .bb1_if_block_1
  j	.bb0_if_block_end

.bb1_if_block_1:
# Return(Return { value: Some(Value(1073741830)) })
  li	a0, 0
# Save global variables.
# Save local variables.
  j	.fib_ret

.bb0_if_block_end:
# Load(Load { src: Value(1073741825) })
  lw	t0, 0(sp)
  mv	t1, t0
# Binary(Binary { op: Eq, lhs: Value(1073741833), rhs: Value(1073741834) })
  li	t2, 1
  xor	t1, t1, t2
  seqz	t1, t1
# Branch(Branch { cond: Value(1073741835), true_bb: BasicBlock(5), false_bb: BasicBlock(4), true_args: [], false_args: [] })
# Save global variables.
# Save local variables.
  sw	t0, 0(sp)
  bnez	t1, .bb3_if_block_1
  j	.bb2_if_block_end

.bb3_if_block_1:
# Return(Return { value: Some(Value(1073741836)) })
  li	a0, 1
# Save global variables.
# Save local variables.
  j	.fib_ret

.bb2_if_block_end:
# Alloc(Alloc)
# Load(Load { src: Value(1073741825) })
  lw	t0, 0(sp)
  mv	t1, t0
# Binary(Binary { op: Sub, lhs: Value(1073741840), rhs: Value(1073741841) })
  li	t2, 1
  sub	t1, t1, t2
# Store(Store { value: Value(1073741842), dest: Value(1073741839) })
  mv	t2, t1
# Alloc(Alloc)
# Load(Load { src: Value(1073741825) })
  mv	t1, t0
# Binary(Binary { op: Sub, lhs: Value(1073741845), rhs: Value(1073741846) })
  li	t3, 2
  sub	t1, t1, t3
# Store(Store { value: Value(1073741847), dest: Value(1073741844) })
  mv	t3, t1
# Load(Load { src: Value(1073741839) })
  mv	t1, t2
# Call(Call { callee: Function(9), args: [Value(1073741849)] })
  mv	a0, t1
  sw	t0, 0(sp)
  sw	t2, 16(sp)
  sw	t3, 8(sp)
# Save global variables.
  call	fib
  mv	t0, a0
# Load(Load { src: Value(1073741844) })
  lw	t1, 8(sp)
  mv	t2, t1
# Call(Call { callee: Function(9), args: [Value(1073741851)] })
  mv	a0, t2
  sw	t0, 44(sp)
  sw	t1, 8(sp)
# Save global variables.
  call	fib
  mv	t0, a0
# Binary(Binary { op: Add, lhs: Value(1073741850), rhs: Value(1073741852) })
  lw	t1, 44(sp)
  add	t0, t1, t0
# Return(Return { value: Some(Value(1073741853)) })
  mv	a0, t0
# Save global variables.
# Save local variables.
  j	.fib_ret

.fib_ret:
  lw	ra, 60(sp)
  addi	sp, sp, 64
  ret

  .global main
main:
  addi	sp, sp, -32
  sw	ra, 28(sp)

.main_body:
# Alloc(Alloc)
# Store(Store { value: Value(1073741856), dest: Value(1073741855) })
  li	t0, 10
  mv	t1, t0
# Load(Load { src: Value(1073741855) })
  mv	t0, t1
# Call(Call { callee: Function(9), args: [Value(1073741858)] })
  mv	a0, t0
  sw	t1, 0(sp)
# Save global variables.
  call	fib
  mv	t0, a0
# Return(Return { value: Some(Value(1073741859)) })
  mv	a0, t0
# Save global variables.
# Save local variables.
  j	.main_ret

.main_ret:
  lw	ra, 28(sp)
  addi	sp, sp, 32
  ret

