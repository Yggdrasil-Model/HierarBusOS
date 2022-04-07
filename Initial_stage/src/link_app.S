
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 20
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_8_start
    .quad app_9_start
    .quad app_10_start
    .quad app_11_start
    .quad app_12_start
    .quad app_13_start
    .quad app_14_start
    .quad app_15_start
    .quad app_16_start
    .quad app_17_start
    .quad app_18_start
    .quad app_19_start
    .quad app_19_end

    .global _app_names
_app_names:
    .string "exit"
    .string "fantastic_text"
    .string "forktest"
    .string "forktest2"
    .string "forktest_simple"
    .string "forktree"
    .string "hello_world"
    .string "initproc"
    .string "matrix"
    .string "sleep"
    .string "sleep_simple"
    .string "stack_overflow"
    .string "user_shell"
    .string "usertests"
    .string "yield"
    .string "lat_mmap"
    .string "lat_fork"
    .string "lat_syscall"
    .string "hello"
    .string "lat_exec"

    .section .data
    .global app_0_start
    .global app_0_end
    .align 3
app_0_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/exit"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 3
app_1_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/fantastic_text"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .align 3
app_2_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/forktest"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
    .align 3
app_3_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/forktest2"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
    .align 3
app_4_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/forktest_simple"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
    .align 3
app_5_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/forktree"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
    .align 3
app_6_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/hello_world"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
    .align 3
app_7_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/initproc"
app_7_end:

    .section .data
    .global app_8_start
    .global app_8_end
    .align 3
app_8_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/matrix"
app_8_end:

    .section .data
    .global app_9_start
    .global app_9_end
    .align 3
app_9_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/sleep"
app_9_end:

    .section .data
    .global app_10_start
    .global app_10_end
    .align 3
app_10_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/sleep_simple"
app_10_end:

    .section .data
    .global app_11_start
    .global app_11_end
    .align 3
app_11_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/stack_overflow"
app_11_end:

    .section .data
    .global app_12_start
    .global app_12_end
    .align 3
app_12_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/user_shell"
app_12_end:

    .section .data
    .global app_13_start
    .global app_13_end
    .align 3
app_13_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/usertests"
app_13_end:

    .section .data
    .global app_14_start
    .global app_14_end
    .align 3
app_14_start:
    .incbin "../user_bus/target/riscv64gc-unknown-none-elf/release/yield"
app_14_end:

    .section .data
    .global app_15_start
    .global app_15_end
    .align 3
app_15_start:
    .incbin "../cbenchmark/ctarget/lat_mmap"
app_15_end:

    .section .data
    .global app_16_start
    .global app_16_end
    .align 3
app_16_start:
    .incbin "../cbenchmark/ctarget/lat_fork"
app_16_end:

    .section .data
    .global app_17_start
    .global app_17_end
    .align 3
app_17_start:
    .incbin "../cbenchmark/ctarget/lat_syscall"
app_17_end:

    .section .data
    .global app_18_start
    .global app_18_end
    .align 3
app_18_start:
    .incbin "../cbenchmark/ctarget/hello"
app_18_end:

    .section .data
    .global app_19_start
    .global app_19_end
    .align 3
app_19_start:
    .incbin "../cbenchmark/ctarget/lat_exec"
app_19_end:
