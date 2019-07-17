#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  void * ptr = malloc(1);
  char msg[30];
  sprintf(msg, "Ptr: %p\n", ptr);
  write(1, msg, strlen(msg));
  return 0;
}

