  .data
n:
  .zero 4

  .text
  .global bubblesort
bubblesort:
  addi	sp, sp, -96
  sw	ra, 92(sp)

.bubblesort_body:
# Alloc(Alloc)
# Store(Store { value: Value(1073741824), dest: Value(1073741825) })
  mv	t0, a0
# Alloc(Alloc)
# Alloc(Alloc)
# Store(Store { value: Value(1073741829), dest: Value(1073741827) })
  li	t1, 0
  mv	t2, t1
# Jump(Jump { target: BasicBlock(2), args: [] })
# Save global variables.
# Save local variables.
  sw	t0, 4(sp)
  sw	t2, 12(sp)
  j	.bb0_while_start

.bb0_while_start:
# Load(Load { src: Value(1073741827) })
  lw	t0, 12(sp)
  mv	t1, t0
# Load(Load { src: Value(2) })
  la	x31, n
  lw	t2, 0(x31)
  mv	t3, t2
# Binary(Binary { op: Sub, lhs: Value(1073741833), rhs: Value(1073741834) })
  li	t4, 1
  sub	t3, t3, t4
# Binary(Binary { op: Lt, lhs: Value(1073741832), rhs: Value(1073741835) })
  slt	t1, t1, t3
# Branch(Branch { cond: Value(1073741836), true_bb: BasicBlock(3), false_bb: BasicBlock(4), true_args: [], false_args: [] })
# Save global variables.
  la	x31, n
  sw	t2, 0(x31)
# Save local variables.
  sw	t0, 12(sp)
  bnez	t1, .bb1_while_body
  j	.bb2_while_end

.bb1_while_body:
# Store(Store { value: Value(1073741838), dest: Value(1073741828) })
  li	t0, 0
  mv	t1, t0
# Jump(Jump { target: BasicBlock(5), args: [] })
# Save global variables.
# Save local variables.
  sw	t1, 0(sp)
  j	.bb3_while_start

.bb2_while_end:
# Return(Return { value: Some(Value(1073741895)) })
  li	a0, 0
# Save global variables.
# Save local variables.
  j	.bubblesort_ret

.bb3_while_start:
# Load(Load { src: Value(1073741828) })
  lw	t0, 0(sp)
  mv	t1, t0
# Load(Load { src: Value(2) })
  la	x31, n
  lw	t2, 0(x31)
  mv	t3, t2
# Load(Load { src: Value(1073741827) })
  lw	t4, 12(sp)
  mv	t5, t4
# Binary(Binary { op: Sub, lhs: Value(1073741842), rhs: Value(1073741843) })
  sub	t3, t3, t5
# Binary(Binary { op: Sub, lhs: Value(1073741844), rhs: Value(1073741845) })
  li	t5, 1
  sub	t3, t3, t5
# Binary(Binary { op: Lt, lhs: Value(1073741841), rhs: Value(1073741846) })
  slt	t1, t1, t3
# Branch(Branch { cond: Value(1073741847), true_bb: BasicBlock(6), false_bb: BasicBlock(7), true_args: [], false_args: [] })
# Save global variables.
  la	x31, n
  sw	t2, 0(x31)
# Save local variables.
  sw	t0, 0(sp)
  sw	t4, 12(sp)
  bnez	t1, .bb4_while_body
  j	.bb5_while_end

.bb4_while_body:
# Load(Load { src: Value(1073741828) })
  lw	t0, 0(sp)
  mv	t1, t0
# Load(Load { src: Value(1073741825) })
  lw	t2, 4(sp)
  mv	t3, t2
# GetPtr(GetPtr { src: Value(1073741850), index: Value(1073741849) })
  li	x31, 4
  mul	x31, t1, x31
  add	t3, t3, x31
# Load(Load { src: Value(1073741851) })
  lw	t1, 28(sp)
  lw	t1, 0(t3)
# Load(Load { src: Value(1073741828) })
  mv	t4, t0
# Binary(Binary { op: Add, lhs: Value(1073741853), rhs: Value(1073741854) })
  li	t5, 1
  add	t4, t4, t5
# Load(Load { src: Value(1073741825) })
  mv	t5, t2
# GetPtr(GetPtr { src: Value(1073741856), index: Value(1073741855) })
  li	x31, 4
  mul	x31, t4, x31
  add	t5, t5, x31
# Load(Load { src: Value(1073741857) })
  lw	t4, 48(sp)
  lw	t4, 0(t5)
# Binary(Binary { op: Gt, lhs: Value(1073741852), rhs: Value(1073741858) })
  sgt	t1, t1, t4
# Branch(Branch { cond: Value(1073741859), true_bb: BasicBlock(9), false_bb: BasicBlock(8), true_args: [], false_args: [] })
# Save global variables.
# Save local variables.
  sw	t0, 0(sp)
  sw	t2, 4(sp)
  bnez	t1, .bb7_if_block_1
  j	.bb6_if_block_end

.bb5_while_end:
# Load(Load { src: Value(1073741827) })
  lw	t0, 12(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741890), rhs: Value(1073741891) })
  li	t2, 1
  add	t1, t1, t2
# Store(Store { value: Value(1073741892), dest: Value(1073741827) })
  mv	t0, t1
# Jump(Jump { target: BasicBlock(2), args: [] })
# Save global variables.
# Save local variables.
  sw	t0, 12(sp)
  j	.bb0_while_start

.bb7_if_block_1:
# Alloc(Alloc)
# Load(Load { src: Value(1073741828) })
  lw	t0, 0(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741861), rhs: Value(1073741862) })
  li	t2, 1
  add	t1, t1, t2
# Load(Load { src: Value(1073741825) })
  lw	t2, 4(sp)
  mv	t4, t2
# GetPtr(GetPtr { src: Value(1073741864), index: Value(1073741863) })
  li	x31, 4
  mul	x31, t1, x31
  add	t4, t4, x31
# Load(Load { src: Value(1073741865) })
  lw	t1, 32(sp)
  lw	t1, 0(t4)
# Store(Store { value: Value(1073741866), dest: Value(1073741860) })
  sw	t3, 24(sp)
  mv	t3, t1
# Load(Load { src: Value(1073741828) })
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741868), rhs: Value(1073741869) })
  sw	t5, 44(sp)
  li	t5, 1
  add	t1, t1, t5
# Load(Load { src: Value(1073741825) })
  mv	t5, t2
# GetPtr(GetPtr { src: Value(1073741871), index: Value(1073741870) })
  li	x31, 4
  mul	x31, t1, x31
  add	t5, t5, x31
# Load(Load { src: Value(1073741828) })
  mv	t1, t0
# Load(Load { src: Value(1073741825) })
  sw	t0, 0(sp)
  mv	t0, t2
# GetPtr(GetPtr { src: Value(1073741874), index: Value(1073741873) })
  li	x31, 4
  mul	x31, t1, x31
  add	t0, t0, x31
# Load(Load { src: Value(1073741875) })
  lw	t1, 64(sp)
  lw	t1, 0(t0)
# Store(Store { value: Value(1073741876), dest: Value(1073741872) })
  sw	t1, 0(t5)
# Load(Load { src: Value(1073741828) })
  sw	t2, 4(sp)
  lw	t2, 0(sp)
  sw	t4, 28(sp)
  mv	t4, t2
# Load(Load { src: Value(1073741825) })
  sw	t3, 8(sp)
  lw	t3, 4(sp)
  sw	t5, 48(sp)
  mv	t5, t3
# GetPtr(GetPtr { src: Value(1073741879), index: Value(1073741878) })
  li	x31, 4
  mul	x31, t4, x31
  add	t5, t5, x31
# Load(Load { src: Value(1073741860) })
  lw	t4, 8(sp)
  sw	t0, 60(sp)
  mv	t0, t4
# Store(Store { value: Value(1073741881), dest: Value(1073741880) })
  sw	t0, 0(t5)
# Jump(Jump { target: BasicBlock(8), args: [] })
# Save global variables.
# Save local variables.
  sw	t2, 0(sp)
  sw	t3, 4(sp)
  sw	t4, 8(sp)
  j	.bb6_if_block_end

.bb6_if_block_end:
# Load(Load { src: Value(1073741828) })
  lw	t2, 0(sp)
  mv	t3, t2
# Binary(Binary { op: Add, lhs: Value(1073741885), rhs: Value(1073741886) })
  li	t4, 1
  add	t3, t3, t4
# Store(Store { value: Value(1073741887), dest: Value(1073741828) })
  mv	t2, t3
# Jump(Jump { target: BasicBlock(5), args: [] })
# Save global variables.
# Save local variables.
  sw	t2, 0(sp)
  j	.bb3_while_start

.bubblesort_ret:
  lw	ra, 92(sp)
  addi	sp, sp, 96
  ret

  .global main
main:
  addi	sp, sp, -64
  sw	ra, 60(sp)

.main_body:
# Store(Store { value: Value(1073741897), dest: Value(2) })
  li	t0, 10
  mv	t1, t0
# Alloc(Alloc)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741900) })
  li	t0, 0
  addi	t2, sp, 0
  li	x31, 4
  mul	x31, t0, x31
  add	t2, t2, x31
# Store(Store { value: Value(1073741902), dest: Value(1073741901) })
  li	t0, 4
  sw	t0, 0(t2)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741904) })
  li	t3, 1
  addi	t4, sp, 0
  li	x31, 4
  mul	x31, t3, x31
  add	t4, t4, x31
# Store(Store { value: Value(1073741906), dest: Value(1073741905) })
  li	t3, 3
  sw	t3, 0(t4)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741908) })
  li	t5, 2
  la	x31, n
  sw	t1, 0(x31)
  addi	t1, sp, 0
  li	x31, 4
  mul	x31, t5, x31
  add	t1, t1, x31
# Store(Store { value: Value(1073741910), dest: Value(1073741909) })
  li	t5, 9
  sw	t5, 0(t1)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741912) })
  sw	t2, 12(sp)
  li	t2, 3
  addi	t0, sp, 0
  li	x31, 4
  mul	x31, t2, x31
  add	t0, t0, x31
# Store(Store { value: Value(1073741914), dest: Value(1073741913) })
  li	t2, 2
  sw	t2, 0(t0)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741916) })
  sw	t4, 16(sp)
  li	t4, 4
  addi	t3, sp, 0
  li	x31, 4
  mul	x31, t4, x31
  add	t3, t3, x31
# Store(Store { value: Value(1073741918), dest: Value(1073741917) })
  li	t4, 0
  sw	t4, 0(t3)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741920) })
  sw	t1, 20(sp)
  li	t1, 5
  addi	t5, sp, 0
  li	x31, 4
  mul	x31, t1, x31
  add	t5, t5, x31
# Store(Store { value: Value(1073741922), dest: Value(1073741921) })
  li	t1, 1
  sw	t1, 0(t5)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741924) })
  sw	t0, 24(sp)
  li	t0, 6
  addi	t2, sp, 0
  li	x31, 4
  mul	x31, t0, x31
  add	t2, t2, x31
# Store(Store { value: Value(1073741926), dest: Value(1073741925) })
  li	t0, 6
  sw	t0, 0(t2)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741928) })
  sw	t3, 28(sp)
  li	t3, 7
  addi	t4, sp, 0
  li	x31, 4
  mul	x31, t3, x31
  add	t4, t4, x31
# Store(Store { value: Value(1073741930), dest: Value(1073741929) })
  li	t3, 5
  sw	t3, 0(t4)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741932) })
  sw	t5, 32(sp)
  li	t5, 8
  addi	t1, sp, 0
  li	x31, 4
  mul	x31, t5, x31
  add	t1, t1, x31
# Store(Store { value: Value(1073741934), dest: Value(1073741933) })
  li	t5, 7
  sw	t5, 0(t1)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741936) })
  sw	t2, 36(sp)
  li	t2, 9
  addi	t0, sp, 0
  li	x31, 4
  mul	x31, t2, x31
  add	t0, t0, x31
# Store(Store { value: Value(1073741938), dest: Value(1073741937) })
  li	t2, 8
  sw	t2, 0(t0)
# Alloc(Alloc)
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741941) })
  sw	t4, 40(sp)
  li	t4, 0
  addi	t3, sp, 0
  li	x31, 4
  mul	x31, t4, x31
  add	t3, t3, x31
# Call(Call { callee: Function(9), args: [Value(1073741942)] })
  mv	a0, t3
  sw	t0, 48(sp)
  sw	t1, 44(sp)
# Save global variables.
  call	bubblesort
  mv	t0, a0
# Store(Store { value: Value(1073741943), dest: Value(1073741940) })
  mv	t1, t0
# Jump(Jump { target: BasicBlock(11), args: [] })
# Save global variables.
# Save local variables.
  sw	t1, 4(sp)
  j	.bb8_while_start

.bb8_while_start:
# Load(Load { src: Value(1073741940) })
  lw	t0, 4(sp)
  mv	t1, t0
# Load(Load { src: Value(2) })
  la	x31, n
  lw	t2, 0(x31)
  mv	t3, t2
# Binary(Binary { op: Lt, lhs: Value(1073741946), rhs: Value(1073741947) })
  slt	t1, t1, t3
# Branch(Branch { cond: Value(1073741948), true_bb: BasicBlock(12), false_bb: BasicBlock(13), true_args: [], false_args: [] })
# Save global variables.
  la	x31, n
  sw	t2, 0(x31)
# Save local variables.
  sw	t0, 4(sp)
  bnez	t1, .bb9_while_body
  j	.bb10_while_end

.bb9_while_body:
# Alloc(Alloc)
# Load(Load { src: Value(1073741940) })
  lw	t0, 4(sp)
  mv	t1, t0
# GetElemPtr(GetElemPtr { src: Value(1073741899), index: Value(1073741951) })
  addi	t2, sp, 0
  li	x31, 4
  mul	x31, t1, x31
  add	t2, t2, x31
# Load(Load { src: Value(1073741952) })
  lw	t1, 20(sp)
  lw	t1, 0(t2)
# Store(Store { value: Value(1073741953), dest: Value(1073741950) })
  mv	t3, t1
# Load(Load { src: Value(1073741950) })
  mv	t1, t3
# Call(Call { callee: Function(4), args: [Value(1073741955)] })
  mv	a0, t1
  sw	t0, 4(sp)
  sw	t2, 16(sp)
  sw	t3, 8(sp)
# Save global variables.
  call	putint
  mv	t0, a0
# Store(Store { value: Value(1073741957), dest: Value(1073741950) })
  li	t1, 10
  mv	t2, t1
# Load(Load { src: Value(1073741950) })
  mv	t1, t2
# Call(Call { callee: Function(5), args: [Value(1073741959)] })
  mv	a0, t1
  sw	t0, 28(sp)
  sw	t2, 8(sp)
# Save global variables.
  call	putch
  mv	t0, a0
# Load(Load { src: Value(1073741940) })
  lw	t1, 4(sp)
  mv	t2, t1
# Binary(Binary { op: Add, lhs: Value(1073741961), rhs: Value(1073741962) })
  li	t3, 1
  add	t2, t2, t3
# Store(Store { value: Value(1073741963), dest: Value(1073741940) })
  mv	t1, t2
# Jump(Jump { target: BasicBlock(11), args: [] })
# Save global variables.
# Save local variables.
  sw	t1, 4(sp)
  j	.bb8_while_start

.bb10_while_end:
# Return(Return { value: Some(Value(1073741966)) })
  li	a0, 0
# Save global variables.
# Save local variables.
  j	.main_ret

.main_ret:
  lw	ra, 60(sp)
  addi	sp, sp, 64
  ret

