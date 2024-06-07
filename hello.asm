  .data
a:
  .zero 4

b:
  .zero 4

c:
  .zero 4

d:
  .zero 4

e:
  .zero 4

f:
  .zero 4

g:
  .zero 4

h:
  .zero 4

i:
  .zero 4

j:
  .zero 4

k:
  .zero 4

l:
  .zero 4

m:
  .zero 4

n:
  .zero 4

o:
  .zero 4

p:
  .zero 4

q:
  .zero 4

  .text
  .global func
func:
  addi	sp, sp, -240
  sw	ra, 236(sp)

.func_body:
# Alloc(Alloc)
# Store(Store { value: Value(1073741824), dest: Value(1073741836) })
  mv	t0, a0
# Alloc(Alloc)
# Store(Store { value: Value(1073741825), dest: Value(1073741838) })
  mv	t1, a1
# Alloc(Alloc)
# Store(Store { value: Value(1073741826), dest: Value(1073741840) })
  mv	t2, a2
# Alloc(Alloc)
# Store(Store { value: Value(1073741827), dest: Value(1073741842) })
  mv	t3, a3
# Alloc(Alloc)
# Store(Store { value: Value(1073741828), dest: Value(1073741844) })
  mv	t4, a4
# Alloc(Alloc)
# Store(Store { value: Value(1073741829), dest: Value(1073741846) })
  mv	t5, a5
# Alloc(Alloc)
# Store(Store { value: Value(1073741830), dest: Value(1073741848) })
  sw	t0, 148(sp)
  mv	t0, a6
# Alloc(Alloc)
# Store(Store { value: Value(1073741831), dest: Value(1073741850) })
  sw	t1, 156(sp)
  mv	t1, a7
# Alloc(Alloc)
# Store(Store { value: Value(1073741832), dest: Value(1073741852) })
  sw	t2, 56(sp)
  lw	t2, 240(sp)
  sw	t3, 132(sp)
  mv	t3, t2
# Alloc(Alloc)
# Store(Store { value: Value(1073741833), dest: Value(1073741854) })
  sw	t4, 208(sp)
  lw	t4, 244(sp)
  sw	t5, 188(sp)
  mv	t5, t4
# Alloc(Alloc)
# Store(Store { value: Value(1073741834), dest: Value(1073741856) })
  sw	t0, 164(sp)
  lw	t0, 248(sp)
  sw	t1, 180(sp)
  mv	t1, t0
# Alloc(Alloc)
# Store(Store { value: Value(1073741835), dest: Value(1073741858) })
  sw	t2, 240(sp)
  lw	t2, 252(sp)
  sw	t3, 112(sp)
  mv	t3, t2
# Load(Load { src: Value(1073741836) })
  sw	t4, 244(sp)
  lw	t4, 148(sp)
  sw	t5, 88(sp)
  mv	t5, t4
# Load(Load { src: Value(1073741838) })
  sw	t0, 248(sp)
  lw	t0, 156(sp)
  sw	t1, 200(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741860), rhs: Value(1073741861) })
  sw	t2, 252(sp)
  add	t2, t5, t1
# Load(Load { src: Value(1073741840) })
  sw	t3, 36(sp)
  lw	t3, 56(sp)
  sw	t4, 148(sp)
  mv	t4, t3
# Binary(Binary { op: Add, lhs: Value(1073741862), rhs: Value(1073741863) })
  sw	t5, 232(sp)
  add	t5, t2, t4
# Load(Load { src: Value(1073741842) })
  sw	t0, 156(sp)
  lw	t0, 132(sp)
  sw	t1, 72(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741864), rhs: Value(1073741865) })
  sw	t2, 16(sp)
  add	t2, t5, t1
# Load(Load { src: Value(1073741844) })
  sw	t3, 56(sp)
  lw	t3, 208(sp)
  sw	t4, 12(sp)
  mv	t4, t3
# Binary(Binary { op: Add, lhs: Value(1073741866), rhs: Value(1073741867) })
  sw	t5, 68(sp)
  add	t5, t2, t4
# Load(Load { src: Value(1073741846) })
  sw	t0, 132(sp)
  lw	t0, 188(sp)
  sw	t1, 196(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741868), rhs: Value(1073741869) })
  sw	t2, 80(sp)
  add	t2, t5, t1
# Load(Load { src: Value(1073741848) })
  sw	t3, 208(sp)
  lw	t3, 164(sp)
  sw	t4, 20(sp)
  mv	t4, t3
# Binary(Binary { op: Add, lhs: Value(1073741870), rhs: Value(1073741871) })
  sw	t5, 120(sp)
  add	t5, t2, t4
# Load(Load { src: Value(1073741850) })
  sw	t0, 188(sp)
  lw	t0, 180(sp)
  sw	t1, 48(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741872), rhs: Value(1073741873) })
  sw	t2, 100(sp)
  add	t2, t5, t1
# Load(Load { src: Value(1073741852) })
  sw	t3, 164(sp)
  lw	t3, 112(sp)
  sw	t4, 24(sp)
  mv	t4, t3
# Binary(Binary { op: Add, lhs: Value(1073741874), rhs: Value(1073741875) })
  sw	t5, 84(sp)
  add	t5, t2, t4
# Load(Load { src: Value(1073741854) })
  sw	t0, 180(sp)
  lw	t0, 88(sp)
  sw	t1, 76(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741876), rhs: Value(1073741877) })
  sw	t2, 28(sp)
  add	t2, t5, t1
# Load(Load { src: Value(1073741856) })
  sw	t3, 112(sp)
  lw	t3, 200(sp)
  sw	t4, 44(sp)
  mv	t4, t3
# Binary(Binary { op: Add, lhs: Value(1073741878), rhs: Value(1073741879) })
  sw	t5, 104(sp)
  add	t5, t2, t4
# Load(Load { src: Value(1073741858) })
  sw	t0, 88(sp)
  lw	t0, 36(sp)
  sw	t1, 4(sp)
  mv	t1, t0
# Binary(Binary { op: Add, lhs: Value(1073741880), rhs: Value(1073741881) })
  sw	t2, 140(sp)
  add	t2, t5, t1
# Return(Return { value: Some(Value(1073741882)) })
  mv	a0, t2
  j	.func_ret

.func_ret:
# Save global variables.
  lw	ra, 236(sp)
  addi	sp, sp, 240
  ret

  .global main
main:
  addi	sp, sp, -80
  sw	ra, 76(sp)

.main_body:
# Load(Load { src: Value(2) })
  la	x31, a
  lw	t0, 0(x31)
  mv	t1, t0
# Load(Load { src: Value(4) })
  la	x31, b
  lw	t2, 0(x31)
  mv	t3, t2
# Load(Load { src: Value(6) })
  la	x31, c
  lw	t4, 0(x31)
  mv	t5, t4
# Load(Load { src: Value(8) })
  la	x31, a
  sw	t0, 0(x31)
  la	x31, d
  lw	t0, 0(x31)
  sw	t1, 36(sp)
  mv	t1, t0
# Load(Load { src: Value(10) })
  la	x31, b
  sw	t2, 0(x31)
  la	x31, e
  lw	t2, 0(x31)
  sw	t3, 60(sp)
  mv	t3, t2
# Load(Load { src: Value(12) })
  la	x31, c
  sw	t4, 0(x31)
  la	x31, f
  lw	t4, 0(x31)
  sw	t5, 44(sp)
  mv	t5, t4
# Load(Load { src: Value(14) })
  la	x31, d
  sw	t0, 0(x31)
  la	x31, g
  lw	t0, 0(x31)
  sw	t1, 28(sp)
  mv	t1, t0
# Load(Load { src: Value(16) })
  la	x31, e
  sw	t2, 0(x31)
  la	x31, h
  lw	t2, 0(x31)
  sw	t3, 32(sp)
  mv	t3, t2
# Load(Load { src: Value(18) })
  la	x31, f
  sw	t4, 0(x31)
  la	x31, i
  lw	t4, 0(x31)
  sw	t5, 24(sp)
  mv	t5, t4
# Load(Load { src: Value(20) })
  la	x31, g
  sw	t0, 0(x31)
  la	x31, j
  lw	t0, 0(x31)
  sw	t1, 48(sp)
  mv	t1, t0
# Load(Load { src: Value(22) })
  la	x31, h
  sw	t2, 0(x31)
  la	x31, k
  lw	t2, 0(x31)
  sw	t3, 52(sp)
  mv	t3, t2
# Load(Load { src: Value(24) })
  la	x31, i
  sw	t4, 0(x31)
  la	x31, l
  lw	t4, 0(x31)
  sw	t5, 56(sp)
  mv	t5, t4
# Call(Call { callee: Function(9), args: [Value(1073741884), Value(1073741885), Value(1073741886), Value(1073741887), Value(1073741888), Value(1073741889), Value(1073741890), Value(1073741891), Value(1073741892), Value(1073741893), Value(1073741894), Value(1073741895)] })
  lw	a0, 36(sp)
  lw	a1, 60(sp)
  lw	a2, 44(sp)
  lw	a3, 28(sp)
  lw	a4, 32(sp)
  lw	a5, 24(sp)
  lw	a6, 48(sp)
  lw	a7, 52(sp)
  la	x31, j
  sw	t0, 0(x31)
  lw	t0, 56(sp)
  sw	t0, 0(sp)
  sw	t1, 4(sp)
  sw	t3, 8(sp)
  sw	t5, 12(sp)
  sw	t0, 56(sp)
  sw	t1, 64(sp)
  la	x31, k
  sw	t2, 0(x31)
  sw	t3, 16(sp)
  la	x31, l
  sw	t4, 0(x31)
  sw	t5, 40(sp)
# Save global variables.
  call	func
# Return(Return { value: Some(Value(1073741896)) })
  j	.main_ret

.main_ret:
# Save global variables.
  lw	ra, 76(sp)
  addi	sp, sp, 80
  ret

