all: asm

c: test.c 
	riscv64-unknown-elf-gcc -o test -T linker.ld test.c -nostdlib
	riscv64-unknown-elf-objcopy -O binary test test.bin

asm: test_add.s test_load_store.s
	riscv64-unknown-elf-as -o test_add.o test_add.s
	riscv64-unknown-elf-as -o test_load_store.o test_load_store.s
	riscv64-unknown-elf-ld -o test_add -T linker.ld test_add.o
	riscv64-unknown-elf-ld -o test_load_store -T linker.ld test_load_store.o
	riscv64-unknown-elf-objcopy -O binary test_add test_add.bin
	riscv64-unknown-elf-objcopy -O binary test_load_store test_load_store.bin

clean:
	rm -rf *.bin *.o test_add test_load_store
