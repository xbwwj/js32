# Other rustc codegen backends

- [rust itself](https://github.com/rust-lang/rust.git) the golden standard, includes `llvm`, `gcc`, `cranelift`
- [rustc_codegen_c](https://github.com/rust-lang/rustc_codegen_c) the simplest one, outdated
- [rustic/rustc_codegen_c](https://github.com/rustic-compiler/rustc_codegen_c)
- [rustc_codegen_clr](https://github.com/fractalfir/rustc_codegen_clr) .NET
- [rustc_codegen_jvm](https://github.com/IntegralPilot/rustc_codegen_jvm) JVM
- [nvidia/cuda-rust](https://github.com/NVIDIA/cuda-rust)
- [Rust-GPU/rust-cuda](https://github.com/rust-gpu/rust-cuda) less competitive since Nvidia official
- [rust-gpu](https://github.com/rust-gpu/rust-gpu) spir-v
- [GoldStrikeArch/topcoat](https://github.com/tokio-rs/topcoat/issues/275)

## rustc_codegen_jvm

The JVM backend use runtime conversion between JVM value and Rust layout, which I think is not the correct way.

Is also use a custom target triple `jvm-unknown-unknown`. We should use `js32-unknown-unknown` alike.

## cuda-rust (nvidia)

The `cargo-oxide` command is a good standard for toolchain distribution.

## GoldStrikeArch/topcoat

This also targets `rustc_codegen_js`. But it is fully vibed, and have very bad semantic design compared to us at the time of writing. For example, `Vec` and `str` are directly mapped to JS `Array` and `string`. Though it have made faster progress at the beginning, it will get into numerous difficulties expanding to real world case.
