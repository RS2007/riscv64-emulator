all: compile

compile: test_add.s
	riscv64-unknown-elf-as -o test.o test_add.s
	riscv64-unknown-elf-ld -o test -T linker.ld test.o
	riscv64-unknown-elf-objcopy -O binary test test.bin
