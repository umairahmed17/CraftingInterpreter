#include "memory.h"
#include <stdio.h>
#include <stdlib.h>
#include <sys/cdefs.h>

void *reallocate(void *pointer, size_t oldSize, size_t newSize) {
  if (newSize == 0) {
    free(pointer);
    return NULL;
  }
  void *result = realloc(pointer, newSize);
  if (result == NULL) {
    exit(EXIT_FAILURE);
  }
  return result;
}
