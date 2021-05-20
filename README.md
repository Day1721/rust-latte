# rust-latte
Compiler for basic latte programing language

# compilation and running
cargo and used libraries are required to compile.
compile with "make" command,
run using provided executable "latc_llvm"

Writen in rust,
Libraries used:
  - Lalrpop: parser generator
  - colored: colored console output, for yellow line in error messages
  - Other like derive-new, derive-getters and regex

# File structure
source files can be found in "src" directory, which contains:
  - checker: type-checking and (few) optimizations,
  - data: basic data types for further usage in other parts of compiler,
  - parser: text-to-ast, parsing part of solution,
  - translator: translates ast to llvm,
  - scripts: contains executable, which is copied as a result executable compiler

by now, no extentions have been implemented yet