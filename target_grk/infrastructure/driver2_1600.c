#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

#define ERROR_ARGUMENTS "wrong number of arguments\n"

int asm_main(void *heap, int64_t input1, int64_t input2) asm("asm_main");

int main(int argc, char *argv[]) {
  int val;

  uint64_t heapsize = UINT64_C(1024 * 1024) * 1600;
  void *heap = calloc(heapsize, sizeof(void));

  if (argc != 1 + 2) {
    write(STDOUT_FILENO, ERROR_ARGUMENTS, sizeof(ERROR_ARGUMENTS));
    return 1;
  }

  val = asm_main(heap, atoi(argv[1]), atoi(argv[2]));

  free(heap);

  return val;
}
