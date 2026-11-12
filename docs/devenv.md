# How to setup a development env

To setup the env, the stable rust requires `RUSTC_BOOTSTRAP` env to expose the `rustc_private` symbols.

```toml
# .cargo/config.toml
[env]
RUSTC_BOOTSTRAP = "1"
```

And we need to lock the rust version as we still depend on the internal api from rustc_middle.

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.99.1"
components = [ "rust-src", "rustc-dev", "rust-analyzer", "clippy", "rustfmt", "llvm-tools" ]
```

We already have these two setup in this repo.

And for downstream users, the `cargo js build` should add the env for them. And the type check should not depend on rustc internals.
