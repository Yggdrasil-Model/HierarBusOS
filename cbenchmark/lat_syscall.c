/*
 * lat_syscall.c - time simple system calls
 *
 * Copyright (c) 1996 Larry McVoy.  Distributed under the FSF GPL with
 * additional restriction that results may published only if
 * (1) the benchmark is unmodified, and
 * (2) the version in the sccsid below is included in the report.
 */


#include "bench.h"


#include <stdio.h>                                

void
do_getpid(iter_t iterations)
{
	while (iterations-- > 0) {
		getpid();
	}
}

void
do_write(iter_t iterations)
{
	char	c;
	while (iterations-- > 0) {
		write(1, &c,1);		
	}
}

void
do_read(iter_t iterations)
{
	char	c;

	while (iterations-- > 0) {
		read(1, &c, 1);
		}
}
void do_gettime(iter_t iterations){
	struct timespec stop={0,0};
	while (iterations-- >0){
		clock_gettime(0,&stop);
	}
}



/*void
do_stat(int iterations, void *cookie)
{
	struct _state *pState = (struct _state*)cookie;
	struct	stat sbuf;

	while (iterations-- > 0) {
		if (stat(pState->file, &sbuf) == -1) {
			perror(pState->file);
			return;
		}
	}
}

void
do_fstat(int iterations, void *cookie)
{
	struct _state *pState = (struct _state*)cookie;
	struct	stat sbuf;

	while (iterations-- > 0) {
		if (fstat(pState->fd, &sbuf) == -1) {
			perror("fstat");
			return;
		}
	}
}

void
do_openclose(int iterations, void *cookie)
{
	struct _state *pState = (struct _state*)cookie;
	int	fd;

	while (iterations-- > 0) {
		fd = open(pState->file, 0);
		if (fd == -1) {
			perror(pState->file);
			return;
		}
		close(fd);
	}
}*/



int
main()
{	   //测试总线内部消息传递开销
		//benchmpm(do_gettime, 1,1);

		benchmpm(do_gettime, 1000,1);
		benchmpm(do_write, 10000,1);
	    return(0);
}
