# Other project that uses MLIR

We learned a lot about MLIR usage from these projects.

- C/C++
  - [ClangIR](https://clang.llvm.org/docs/CIR/)
  - [Polygeist](https://github.com/llvm/Polygeist)
  - [trailofbits/vast](https://github.com/trailofbits/vast)
- Rust
  - [Rust Goal - ML optimizations](https://goals.rust-lang.org/2026/high-level-ml.html)
  - [triton-rs](https://github.com/nikhilmitrax/triton-rs)
  - [vaivaswatha/rustc_codegen_pliron](https://github.com/vaivaswatha/rustc_codegen_pliron)
  - [haydenflinner/rustc_codegen_pliron](https://github.com/haydenflinner/rustc_codegen_pliron)
- Python
  - [Pylir](https://github.com/Pylir/Pylir)
  - [numba-mlir](https://github.com/numba/numba-mlir)
  - [torch-mlir](https://github.com/llvm/torch-mlir)
  - [Huawei/PyDSL](https://github.com/Huawei-CPLLab/PyDSL)
  - [filia](https://github.com/SRI-CSL/filia) also js
- JS
  - [jsir](https://github.com/google/jsir)
  - [aho/aot](https://github.com/iho/typescript-aot-native-compiler)
  - [tstz](https://github.com/woxjro/tstz)
- Java
  - [openjdk/babylon](https://github.com/openjdk/babylon)
  - [bruno-ah-um/tosa](https://github.com/bruno-ah-um/openjdk-babylon-mlir-tosa)
- Custom Language
  - [reussir](https://github.com/reussir-lang/reussir)

The following are notes on some highlight ones.

## JSIR

JSIR is mainly used for decompilation (Hermes bytecode) and deobfuscation.

The direction: js -> babel -> ast -> jsir/jshir -> MLIR transforms and analysis -> jsir/hshir -> ast -> js.

Some of its IR design seem to be weird (e.g. `jsir.identifier` and `jsir.identifier_ref`).

Only a `jsir.any` type is used.
