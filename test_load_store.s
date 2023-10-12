main:
    addi    x8,x8,496
    addi    x1,x0,100
    sw      x8,0(x1)
    addi    x7,x7,200
    sw      x1,0(x7) 
    lw      x4,0(x7)
