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
