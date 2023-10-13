ori x1, x1, 0x103
sh x1, 0(x0)
sb x1, 8(x0)
lb x2, 0(x0)
lw x4, 8(x0)
sub x3,x2,x4
sw x1, 8(x0)
lh x2, 0(x0)
lw x4, 8(x0)
sub x5,x2,x4
add x10,x3,x5
