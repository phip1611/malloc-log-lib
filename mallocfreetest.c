#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  void * ptr = malloc(1);
  char msg1[30];
  char msg2[30];
  sprintf(msg1, "Malloc: Ptr: %p\n", ptr);
  write(1, msg1, strlen(msg1));

  free(ptr);
  sprintf(msg2, "Freed Ptr: %p\n", ptr);
  write(1, msg2, strlen(msg2));
  return 0;
}

