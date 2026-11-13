# js32

A JavaScript codegen backend for Rust, and also a JavaScript optimizer, using the shared MLIR infrastructure.

## Architecture

```mermaid
swimlane-beta LR
  subgraph Rust
    rust[Rust source]
    mir[Rust MIR]
  end

  subgraph IR js32 dialect
    NpmIR[js32 IR distributed]
    js32[js32 dialect]
    js32o[js32 optimized]
  end

  subgraph TypeScript
    User[User JS/TS]
    jsir[js dialect]
    jso[js dialect optimized]
    jsout[The final JS/TS output]
  end

rust --rustc--> mir
mir --lower--> js32
NpmIR --> js32
User --> jsir
jsir --merge and lift--> js32
js32 -- optimize --> js32o
js32o -- lower --> jso
jso -- export --> jsout
User -- may be distributed along with npm packages --> NpmIR
```

## methodology shift

`js32` should be a general js optimizer rather considering complex gc and linear memory interaction. The Wasm should be part of the gc, no a separate global linear one. The language only describe the gc lifetime, and let the compiler decide the representation, and use optimization hints to direct a wasm transformation, rather than considering a global one.

Rust is considered only as a frontend for the js32 compiler, not affecting and intervening the overall optimization pipeline and dialect decision.

Rust types can be natively represented by defining the allocator interface. Take for example the `Vec<T>` case.

```rust
struct Vec<T> {
    ptr: *mut T,
    size: usize,
    capacity: usize,
}
```

Rust does not requires that ptr can safely access ptr out of range, so we can have a separate `ArrayBuffer` when
`GLOBAL.alloc` is required.

```rust
Vec<string> {
    ptr: ArrayBuffer, // no difference with NonNull<u8>
    size,
    capacity,
}
```

The compiler may js primitive types specially. e.g.

```rust
Vec<string>

  // is equivalent to layout of

Vec<string> {
    ptr: Array<string>,
    size,
    capacity,
}

// size and capacity can be later optimized by mlir pass, not here.
```

We only modify the pointer semantic for rust, which is a minimal patch interface. The only drawback is that pointer should not be coerced to usize `ptr as usize`. As the pointer now do not have a global index. And this pattern is always discouraged for rust.

The wasm should be driven by hints like `simd` and etc.
