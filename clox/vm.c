#include <stdio.h>
#include "vm.h"
#include "chunk.h"


VM vm;

void initVM() {}

void freeVM() {}

static InterpretResult op_return(VM *vm) { return INTERPRET_OK; }

static InterpretResult op_constant(VM *vm) {
  Value constant = READ_CONSTANT();
  printValue(constant);
  printf("\n");
  return INTERPRET_OK;
}

InterpretResult (*lookup_table[])(VM *vm) = {
    [OP_RETURN] = op_return,
    [OP_CONSTANT] = op_constant,
};

static InterpretResult run() {
  for (;;) {
    uint8_t instruction;
    switch (instruction = READ_BYTE()) {
    case OP_RETURN:
      return INTERPRET_OK;
    }
  }
#undef READ_BYTE
#undef READ_CONSTANT
}

InterpretResult interpret(Chunk *chunk) {
  vm.chunk = chunk;
  vm.ip = vm.chunk->code;
  return run();
}
