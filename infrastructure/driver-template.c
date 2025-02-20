#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

#define ERROR_ARGUMENTS "wrong number of arguments\n"

long asm_main(void *heap) asm("asm_main");

int main(int argc, char *argv[]) {
  long val;

  long heapsize = 1024 * 1024 * 450;
  void *heap = calloc(heapsize, sizeof(void));

  if (argc != 1 + 0) {
    write(STDOUT_FILENO, ERROR_ARGUMENTS, sizeof(ERROR_ARGUMENTS));
    return 1;
  }

  val = asm_main(heap);

  free(heap);

  return val;
}
