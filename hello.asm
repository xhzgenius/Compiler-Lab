  .data
x:
  .word 0

  .text
  .global add
add:
  addi	sp, sp, -32
  sw	ra, 28(sp)

.add_body:
# Some("@y") Alloc(Alloc)
# None Store(Store { value: Value(1073741824), dest: Value(1073741825) })
  sw	a0, 0(sp)
# None Load(Load { src: Value(2) })
  la	t1, x
  lw	t0, 0(t1)
# None Load(Load { src: Value(1073741825) })
  lw	t1, 0(sp)
# None Binary(Binary { op: Add, lhs: Value(1073741827), rhs: Value(1073741828) })
  add	t2, t0, t1
# None Store(Store { value: Value(1073741829), dest: Value(2) })
  la	t3, x
  sw	t2, 0(t3)
# None Load(Load { src: Value(1073741825) })
  lw	t3, 0(sp)
# None Binary(Binary { op: Sub, lhs: Value(1073741831), rhs: Value(1073741832) })
  li	t4, 1
  sub	t4, t3, t4
# None Call(Call { callee: Function(9), args: [Value(1073741833)] })
  sw	t0, 8(sp)
  sw	t1, 12(sp)
  sw	t2, 16(sp)
  sw	t3, 20(sp)
  sw	t4, 24(sp)
  lw	a0, 24(sp)
  call	add
# None Return(Return { value: None })
  j	.add_ret

.add_ret:
  lw	ra, 28(sp)
  addi	sp, sp, 32
  ret

  .global main
main:
  addi	sp, sp, -16
  sw	ra, 12(sp)

.main_body:
# None Call(Call { callee: Function(9), args: [Value(1073741836)] })
  li	a0, 10
  call	add
# None Load(Load { src: Value(2) })
  la	t1, x
  lw	t0, 0(t1)
# None Return(Return { value: Some(Value(1073741838)) })
  mv	a0, t0
  j	.main_ret

.main_ret:
  lw	ra, 12(sp)
  addi	sp, sp, 16
  ret

