all: compile

compile: test_add.s
	riscv64-unknown-elf-as -o exec test_add.s
	riscv64-unknown-elf-objcopy -O binary exec test.bin
