all: asm

c: test.c 
	riscv64-unknown-elf-gcc -o test -T linker.ld test.c -nostdlib
	riscv64-unknown-elf-objcopy -O binary test test.bin

asm: test_add.s test_load_store.s
	riscv64-unknown-elf-as -o test_add.o test_add.s
	riscv64-unknown-elf-as -o test_load_store.o test_load_store.s
	riscv64-unknown-elf-as -o test_load_store2.o test_load_store2.s
	riscv64-unknown-elf-as -o test_load_store3.o test_load_store3.s
	riscv64-unknown-elf-as -o test_load_store4.o test_load_store4.s
	riscv64-unknown-elf-as -o test_slt_family.o test_slt_family.s
	riscv64-unknown-elf-ld -o test_add.elf -T linker.ld test_add.o
	riscv64-unknown-elf-ld -o test_load_store.elf -T linker.ld test_load_store.o
	riscv64-unknown-elf-ld -o test_load_store2.elf -T linker.ld test_load_store2.o
	riscv64-unknown-elf-ld -o test_load_store3.elf -T linker.ld test_load_store3.o
	riscv64-unknown-elf-ld -o test_load_store4.elf -T linker.ld test_load_store4.o
	riscv64-unknown-elf-ld -o test_slt_family.elf -T linker.ld test_slt_family.o
	riscv64-unknown-elf-objcopy -O binary test_add.elf test_add.bin
	riscv64-unknown-elf-objcopy -O binary test_load_store.elf test_load_store.bin
	riscv64-unknown-elf-objcopy -O binary test_load_store2.elf test_load_store2.bin
	riscv64-unknown-elf-objcopy -O binary test_load_store3.elf test_load_store3.bin
	riscv64-unknown-elf-objcopy -O binary test_load_store4.elf test_load_store4.bin
	riscv64-unknown-elf-objcopy -O binary test_slt_family.elf test_slt_family.bin

clean:
	rm -rf *.bin *.o test_add test_load_store
