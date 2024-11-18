#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

struct ret_val_mem {
  void* mem;
  long val;
};

struct ret_val_mem asm_main0(void *heap);
struct ret_val_mem asm_main1(void *heap, int64_t input1);
struct ret_val_mem asm_main2(void *heap, int64_t input1, int64_t input2);
struct ret_val_mem asm_main3(void *heap, int64_t input1, int64_t input2, int64_t input3);
struct ret_val_mem asm_main4(void *heap, int64_t input1, int64_t input2, int64_t input3, int64_t input4);
struct ret_val_mem asm_main5(void *heap, int64_t input1, int64_t input2, int64_t input3, int64_t input4, int64_t input5);

int main(int argc, char *argv[]) {
  int input1, input2, input3, input4, input5 = 10;
  struct ret_val_mem val_mem;
  long heapsize = 1024 * 1024 * 32;
  void* heap = calloc(heapsize, sizeof(void));
  switch (argc) {
    case 1:
      val_mem = asm_main0(heap);
      break;
    case 2:
      input1 = atoi(argv[1]);
      val_mem = asm_main1(heap, input1);
      break;
    case 3:
      input1 = atoi(argv[1]);
      input2 = atoi(argv[2]);
      val_mem = asm_main2(heap, input1, input2);
      break;
    case 4:
      input1 = atoi(argv[1]);
      input2 = atoi(argv[2]);
      input3 = atoi(argv[3]);
      val_mem = asm_main3(heap, input1, input2, input3);
      break;
    case 5:
      input1 = atoi(argv[1]);
      input2 = atoi(argv[2]);
      input3 = atoi(argv[3]);
      input4 = atoi(argv[4]);
      val_mem = asm_main4(heap, input1, input2, input3, input4);
      break;
    case 6:
      input1 = atoi(argv[1]);
      input2 = atoi(argv[2]);
      input3 = atoi(argv[3]);
      input4 = atoi(argv[4]);
      input5 = atoi(argv[5]);
      val_mem = asm_main5(heap, input1, input2, input3, input4, input5);
      break;
    default:
      printf("arguments are too many\n");
      return 1;
  }
  printf("%li\n", val_mem.val);
  free(heap);
  return 0;
}
