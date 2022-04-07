#include "bench.h"


void do_map(iter_t iterations){
	while (iterations-- > 0){
		register char	 *where=0;
		register size_t size = 4096;
		mmap(0, size, PROT_READ|PROT_WRITE,MAP_FIXED|MAP_ANONYMOUS| MAP_SHARED, -1, 0);
		
		for (int i=0;i<4096;i++){
			*where='a';
			where+=1;
		}
	    munmap(0, size);	
	}
}

int
main()
{	
	/*if (optind == ac - 2) 
		state.file = av[optind + 1];*/
		
		benchmpm(do_map, 10000,1);
	    //dprintf(1,"test %f",ret);
	    return(0);
}