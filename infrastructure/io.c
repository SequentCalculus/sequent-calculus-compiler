#include <stdbool.h>
#include <stdint.h>
#include <unistd.h>

#define MAX_DIGITS_INT 20

void print_i64(int64_t value) asm("print_i64");
void println_i64(int64_t value) asm("println_i64");

void print_i64(int64_t value) {
  char buf[MAX_DIGITS_INT];
  char *start = &buf[MAX_DIGITS_INT];

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

  write(STDOUT_FILENO, start, &buf[MAX_DIGITS_INT] - start);
}

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
