# Design

## Two domains

JS domain + Wasm domain

## target triple

`js32-unknown-unknown`

## tool attributes

```rust
#[js32::js]
struct Foo;

#[js32::wasm]
struct Bar;

#[js32::host]
struct Element;

#[js32::intrinsic]
fn rust_str_to_js_str(s: &str) -> string;
```

Two options for implementations here:

1. register_tool_attributes 原生
2. extern + `js32_sys` 中 macro (jvm)

rough thought：

1. 最好是有宏方便转换和好写。
2. 但是要考虑到 extern 的表达能力存在问题
3. `import`/`export` 似乎还缺乏？
4. field/RefMut/get_mut 的设计考量
5. js/wasm 等似乎终究要通过 tool attr 表达更好
6. `extern` 模式是否会与 linkage extern "C" 存在切实冲突？

## class extends

We can natively express class extends in JS domain.

Two possible designs

```rust
#[js(intrinsic = "extends")]
trait Extends: Deref + DerefMut {
    type Super;
}

struct Element { }

impl Extends for Element {
    type Super = Node;
}

// mozjs 模式，需要实际组合持有
#[js(extends = "Node")]
struct Element {
    node: Node
}
```

## js types

Use `array`/`map`/`string` (lower case type should be allowed), not `Vec`, `HashMap`, `String`.

## Wasm opt-in

TODO

## cargo js tool

Learn from `cargo oxide`

## Vite support

## shadow linear memory

When observation of rust layout happens, the full type should fall back to Wasm.

## MLIR dialects

`js32` + `js`

## Optimization: region analysis + SoA

Like arena of gc, reduce alloc and gc.

Use cases:

- remark markdown ast.
- react-reconciler FiberNode

## Optimization: JIT warmup

Considering the V8 ignition/sparkplug/maglev internal intrinsics.

## Optimization: SIMD Hint

Also the V8 way.

## Relooper

- Emscripten
- V8 stackifier
- Beyond relooper

## Optimization: POJO vs class

`new Point(1, 2)` 为例：

- Object Header 和原型链的指针空间
- `new` 的构造函数 Frame 开销，比数组/字面量慢 3-5 倍
- 原型链查找 `point.distance()` 触发 Inline cache miss
- GC 回收压力：UI 创建数万小 struct 导致新生代 Minor GC Pause

按照开销原则多级别降级

1. **作用域不逃逸 => SRA 标量替换**：纯局部变量零分配
2. **内部数据结构 => PackedArray**：`[x, y]` 
   - V8 对纯数字、句柄数组有优化 `PACKED_DOUBLE_ELEMENTS`, `PACKED_SMI_ELEMENTS`
   - Monophorized Index: `p[0]` 会被 TurboFan 降级为 CPU `MOV`
3. **密集数值类型 => Wasm / TypedArray** => Memory Offset Direct Read
   - 大规模连续 struct `Vec<Particle>`, 直接打包到 Wasm Memory 或者 `SharedArrayBuffer`
4. **纯数据 POJO => 平铺字面量** `{ x: 1, y: 2 }` 无原型链
   - 需要和第三方库交互时，或者字段较多时
   - V8 可以直接推断出固定隐藏类
5. 手动标注/导出 => JSClass
   - 如 `WebComponent` / `AudioWorklet` 或者对外公开 SDK
   - 或者要求 private field, 或者 instanceof 的时候
6. 生成闭包
   - 适合少量实例、高频调用

## Example: particle system

omitted
