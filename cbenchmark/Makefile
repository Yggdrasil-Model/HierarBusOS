CC:=riscv64-linux-musl-gcc
AR=riscv64-linux-musl-ar
ARCREATE=cr
RANLIB=riscv64-linux-musl-ranlib

#CC:=riscv64-unknown-linux-gnu-gcc
#AR=riscv64-unknown-linux-gnu-ar
#ARCREATE=cr
#RANLIB=riscv64-unknown-linux-gnu-ranlib

#CC:=gcc
#AR=ar
#ARCREATE=cr
#RANLIB=ranlib

LIBOBJS=lib_timing.o
INCS =	bench.h timing.h

#CLFAGS += -march=rv64gc  -mabi=lp64
TARGET =hello  lat_syscall lat_fork lat_exec lat_mmap lat_mmap2
all:$(TARGET) 

#lib
lmbench.a: $(LIBOBJS)
	$(AR) $(ARCREATE) ./clib/lmbench.a ./clib/lib_timing.o  
	$(RANLIB) ./clib/lmbench.a

lib_timing.o : lib_timing.c $(INCS)
	$(CC) -c lib_timing.c -o ./clib/lib_timing.o

#test

lat_syscall:  lat_syscall.c $(INCS) lmbench.a
	$(CC)  -static  -O0 -o ./ctarget/lat_syscall  lat_syscall.c ./clib/lmbench.a

lat_fork:  lat_fork.c $(INCS) lmbench.a
	$(CC)  -static  -O0 -o ./ctarget/lat_fork lat_fork.c ./clib/lmbench.a

lat_exec: lat_exec.c $(INCS) lmbench.a
	$(CC)  -static  -O0 -o ./ctarget/lat_exec lat_exec.c ./clib/lmbench.a


lat_mmap: lat_mmap.c $(INCS) lmbench.a
	$(CC)  -static  -O0 -o ./ctarget/lat_mmap lat_mmap.c ./clib/lmbench.a

lat_mmap2: lat_mmap2.c $(INCS) lmbench.a
	$(CC)  -static  -O0 -o ./ctarget/lat_mmap2 lat_mmap2.c ./clib/lmbench.a 

hello: hello.c $(INCS) lmbench.a
	$(CC)  -static  -O0 -o ./ctarget/hello hello.c ./clib/lmbench.a
clean:
	rm ./ctarget/hello  ./ctarget/lat_syscall ./clib/lmbench.a \
	./clib/lib_timing.o   ./ctarget/lat_fork ./ctarget/lat_exec  \
	./ctarget/lat_mmap ./ctarget/lat_mmap2

.PHONY:all
