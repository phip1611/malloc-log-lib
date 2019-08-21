#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(void) {
    char const msg[] = "main() started: doing first malloc():\n";
    write(1, msg, strlen(msg));

    void *ptr = malloc(1);
    char msg1[30];
    char msg2[30];
    sprintf(msg1, "Malloc: Ptr: %p\n", ptr);
    write(1, msg1, strlen(msg1));

    free(ptr);
    sprintf(msg2, "Freed Ptr: %p\n", ptr);
    write(1, msg2, strlen(msg2));

    char const msg3[] = "main(): doing 3 more malloc's + free's:\n";
    write(1, msg3, strlen(msg3));
    // some more tests
    void *ptrs[3];
    ptrs[0] = malloc(1);
    ptrs[1] = malloc(2);
    ptrs[2] = malloc(3);
    free(ptrs[0]);
    free(ptrs[1]);
    free(ptrs[2]);

    return 0;
}

