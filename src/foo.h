#include <stdint.h>

union Union {
  uint8_t bytes[4];
  uint32_t word;
};

typedef union Union UnionAlias;

struct SimpleStruct {
  uint32_t i;
};

typedef struct SimpleStruct SimpleStructAlias;

struct StructContainingUnion {
  union Union u;
};

typedef struct StructContainingUnion AliasOfStructContainingUnion;

// If we uncomment the following struct, the generated bindings will not even
// compile.

// struct StructContainingUnionAlias {
//   UnionAlias ua;
// };