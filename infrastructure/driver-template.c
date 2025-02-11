#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

#define MAX_DIGITS_INT 20

typedef enum { false, true } bool;

void println_i64(int64_t value) asm("println_i64");

void println_i64(int64_t value) {
  char buf[MAX_DIGITS_INT + 1];
  char *start = &buf[MAX_DIGITS_INT];
  *start = '\n';

  bool negative = false;

  if (value < 0) {
    negative = true;
    value = -value;
  }

  int64_t prev_value;

  do {
    prev_value = value;
    value /= 10;
    start--;
    *start = '0' + (prev_value - value * 10);
  } while (value);

  if (negative) {
    start--;
    *start = '-';
  }

  write(STDOUT_FILENO, start, &buf[MAX_DIGITS_INT] - start + 1);
}

#define ERROR_ARGUMENTS "wrong number of arguments\n"

long asm_main(void *heap) asm("asm_main");

int main(int argc, char *argv[]) {
  long val;

  long heapsize = 1024 * 1024 * 32;
  void *heap = calloc(heapsize, sizeof(void));

  if (argc != 1 + 0) {
    write(STDOUT_FILENO, ERROR_ARGUMENTS, sizeof(ERROR_ARGUMENTS));
    return 1;
  }

  val = asm_main(heap);

  free(heap);

  return val;
}
