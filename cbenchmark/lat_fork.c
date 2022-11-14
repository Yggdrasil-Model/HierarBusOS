#include "bench.h"

#define	PROG "hello"

void do_fork(iter_t iterations){
    int child_pid;
    int exitcode=0;
    while (iterations-- > 0)
    {
       switch (child_pid = fork()) {
		case 0: 	/* child */
			exit(1);

		default:
			waitpid(child_pid, &exitcode,0);
		}
		child_pid = 0;
    }
    
}
int
main()
{	
	/*if (optind == ac - 2) 
		state.file = av[optind + 1];*/
		
		//benchmpm(do_fork, 3,1);
        benchmpm(do_fork, 10,1);
	    //dprintf(1,"test %f",ret);
	    return(0);
}