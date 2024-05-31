  .text
  .global main
main:
  addi	sp, sp, -288

main_body:
  li	t0, 1
  sw	t0, 0(sp)
  li	t0, 2
  sw	t0, 8(sp)
  li	t0, 3
  sw	t0, 16(sp)
  lw	t0, 0(sp)
  lw	t1, 0(sp)
  lw	t2, 0(sp)
  lw	t3, 0(sp)
  lw	t4, 0(sp)
  lw	t5, 0(sp)
  lw	t6, 0(sp)
  lw	a0, 0(sp)
  lw	a1, 0(sp)
  lw	a2, 0(sp)
  lw	a3, 0(sp)
  lw	a4, 0(sp)
  lw	a5, 0(sp)
  lw	a6, 0(sp)
  lw	a7, 0(sp)
  sw	t0, 24(sp)
  lw	t0, 0(sp)
  sw	t1, 28(sp)
  lw	t1, 0(sp)
  sw	t2, 32(sp)
  lw	t2, 0(sp)
  sw	t3, 36(sp)
  lw	t3, 0(sp)
  sw	t4, 40(sp)
  lw	t4, 0(sp)
  sw	t5, 44(sp)
  lw	t5, 0(sp)
  sw	t6, 48(sp)
  lw	t6, 0(sp)
  sw	a0, 52(sp)
  lw	a0, 0(sp)
  sw	a1, 56(sp)
  lw	a1, 0(sp)
  sw	a2, 60(sp)
  lw	a2, 0(sp)
  sw	a3, 64(sp)
  lw	a3, 0(sp)
  sw	a4, 68(sp)
  lw	a4, 0(sp)
  sw	a5, 72(sp)
  lw	a5, 0(sp)
  sw	a6, 76(sp)
  lw	a6, 0(sp)
  sw	a7, 80(sp)
  lw	a7, 0(sp)
  sw	t0, 84(sp)
  lw	t0, 0(sp)
  sw	t1, 88(sp)
  lw	t1, 0(sp)
  sw	t2, 92(sp)
  lw	t2, 0(sp)
  sw	t3, 96(sp)
  add	t3, t1, t2
  sw	t4, 100(sp)
  add	t4, t0, t3
  sw	t5, 104(sp)
  add	t5, a7, t4
  sw	t6, 108(sp)
  add	t6, a6, t5
  sw	a0, 112(sp)
  add	a0, a5, t6
  sw	a1, 116(sp)
  add	a1, a4, a0
  sw	a2, 120(sp)
  add	a2, a3, a1
  sw	a3, 124(sp)
  lw	a3, 120(sp)
  sw	a4, 128(sp)
  add	a4, a3, a2
  sw	a5, 132(sp)
  lw	a5, 116(sp)
  sw	a6, 136(sp)
  add	a6, a5, a4
  sw	a7, 140(sp)
  lw	a7, 112(sp)
  sw	t0, 144(sp)
  add	t0, a7, a6
  sw	t1, 148(sp)
  lw	t1, 108(sp)
  sw	t2, 152(sp)
  add	t2, t1, t0
  sw	t3, 156(sp)
  lw	t3, 104(sp)
  sw	t4, 160(sp)
  add	t4, t3, t2
  sw	t5, 164(sp)
  lw	t5, 100(sp)
  sw	t6, 168(sp)
  add	t6, t5, t4
  sw	a0, 172(sp)
  lw	a0, 96(sp)
  sw	a1, 176(sp)
  add	a1, a0, t6
  sw	a2, 180(sp)
  lw	a2, 92(sp)
  sw	a3, 120(sp)
  add	a3, a2, a1
  sw	a4, 184(sp)
  lw	a4, 88(sp)
  sw	a5, 116(sp)
  add	a5, a4, a3
  sw	a6, 188(sp)
  lw	a6, 84(sp)
  sw	a7, 112(sp)
  add	a7, a6, a5
  sw	t0, 192(sp)
  lw	t0, 80(sp)
  sw	t1, 108(sp)
  add	t1, t0, a7
  sw	t2, 196(sp)
  lw	t2, 76(sp)
  sw	t3, 104(sp)
  add	t3, t2, t1
  sw	t4, 200(sp)
  lw	t4, 72(sp)
  sw	t5, 100(sp)
  add	t5, t4, t3
  sw	t6, 204(sp)
  lw	t6, 68(sp)
  sw	a0, 96(sp)
  add	a0, t6, t5
  sw	a1, 208(sp)
  lw	a1, 64(sp)
  sw	a2, 92(sp)
  add	a2, a1, a0
  sw	a3, 212(sp)
  lw	a3, 60(sp)
  sw	a4, 88(sp)
  add	a4, a3, a2
  sw	a5, 216(sp)
  lw	a5, 56(sp)
  sw	a6, 84(sp)
  add	a6, a5, a4
  sw	a7, 220(sp)
  lw	a7, 52(sp)
  sw	t0, 80(sp)
  add	t0, a7, a6
  sw	t1, 224(sp)
  lw	t1, 48(sp)
  sw	t2, 76(sp)
  add	t2, t1, t0
  sw	t3, 228(sp)
  lw	t3, 44(sp)
  sw	t4, 72(sp)
  add	t4, t3, t2
  sw	t5, 232(sp)
  lw	t5, 40(sp)
  sw	t6, 68(sp)
  add	t6, t5, t4
  sw	a0, 236(sp)
  lw	a0, 36(sp)
  sw	a1, 64(sp)
  add	a1, a0, t6
  sw	a2, 240(sp)
  lw	a2, 32(sp)
  sw	a3, 60(sp)
  add	a3, a2, a1
  sw	a4, 244(sp)
  lw	a4, 28(sp)
  sw	a5, 56(sp)
  add	a5, a4, a3
  sw	a6, 248(sp)
  lw	a6, 24(sp)
  sw	a7, 52(sp)
  add	a7, a6, a5

main_ret:
  ret
