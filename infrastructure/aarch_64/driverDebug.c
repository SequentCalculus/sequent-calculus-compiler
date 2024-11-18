#include <stdio.h>
#include <stdlib.h>

struct ret_val_mem {
  void* mem;
  long val;
};

struct ret_val_mem asm_main0(void *heap);

int main() {
  struct ret_val_mem val_mem;
  int heapsize = 1024 * 1024;
  void* heap = calloc(heapsize, sizeof(void));
  val_mem = asm_main0(heap);
  printf("val: %li\n", val_mem.val);
  printf("mem: %p\n", val_mem.mem);
  printf("mem start: %p", heap);
  int i;
  int limit = 128;
  printf("\n[");
  for (i = 0; i <= limit; i = i + 16) {
    void** mem_mem = heap + i;
    int* mem = heap + i + 8;
    printf("%p, ", *mem_mem);
    printf("%i, ", *mem);
  }
  printf("...]");
  free(heap);
  return 0;
}
