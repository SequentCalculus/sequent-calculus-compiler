#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

#define MAX_DIGITS_INT 20

void println_i64(int64_t value) {
  char buf[MAX_DIGITS_INT + 1];
  char *start = &buf[MAX_DIGITS_INT];
  *start = '\n';

  int64_t prev_value;

  do {
    prev_value = value;
    value /= 10;
    start--;
    *start = '0' + (prev_value - value * 10);
  } while (value);

  if (prev_value < 0) {
    start--;
    *start = '-';
  }

  write(STDOUT_FILENO, start, &buf[MAX_DIGITS_INT] - start + 1);
}

long asm_main0(void *heap);
long asm_main1(void *heap, int64_t input1);
long asm_main2(void *heap, int64_t input1, int64_t input2);
long asm_main3(void *heap, int64_t input1, int64_t input2, int64_t input3);
long asm_main4(void *heap, int64_t input1, int64_t input2, int64_t input3,
               int64_t input4);
long asm_main5(void *heap, int64_t input1, int64_t input2, int64_t input3,
               int64_t input4, int64_t input5);

#define ERROR_ARGUMENTS "too many arguments\n"

int main(int argc, char *argv[]) {
  int input1, input2, input3, input4, input5 = 10;
  long val;

  long heapsize = 1024 * 1024 * 32;
  void *heap = calloc(heapsize, sizeof(void));

  switch (argc) {
  case 1:
    val = asm_main0(heap);
    break;
  case 2:
    input1 = atoi(argv[1]);
    val = asm_main1(heap, input1);
    break;
  case 3:
    input1 = atoi(argv[1]);
    input2 = atoi(argv[2]);
    val = asm_main2(heap, input1, input2);
    break;
  case 4:
    input1 = atoi(argv[1]);
    input2 = atoi(argv[2]);
    input3 = atoi(argv[3]);
    val = asm_main3(heap, input1, input2, input3);
    break;
  case 5:
    input1 = atoi(argv[1]);
    input2 = atoi(argv[2]);
    input3 = atoi(argv[3]);
    input4 = atoi(argv[4]);
    val = asm_main4(heap, input1, input2, input3, input4);
    break;
  case 6:
    input1 = atoi(argv[1]);
    input2 = atoi(argv[2]);
    input3 = atoi(argv[3]);
    input4 = atoi(argv[4]);
    input5 = atoi(argv[5]);
    val = asm_main5(heap, input1, input2, input3, input4, input5);
    break;
  default:
    write(STDOUT_FILENO, ERROR_ARGUMENTS, sizeof(ERROR_ARGUMENTS));
    return 1;
  }

  free(heap);

  return val;
}
