# A simple repo demonstrating a bug in bindgen

Bug description:

When bindgen creates bindings to C unions which have typedef aliases, and the default
alias style is `TypeAlias`, everything works as expected.  However, if the default alias style is `NewType` or `NewTypeDeref`, and we ask bindgen to `derive_debug` and `impl_debug`, then the union will have a `Debug` implementation but the alias newtype wrapper will not.

Expected behavior: bindgen should create `Debug` implementations for both the original union and its alias newtype wrapper.

Actual behavior: bindgen creates a `Debug` implementation only for the union type, and not for its alias newtype wrapper.

The same error is observed for structs which contain unions and have a typedef alias.