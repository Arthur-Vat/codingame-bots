# wasi-libc's license files

Rust's target `wasm32-wasip1` links wasi-libc into every WebAssembly file the studio publishes (ADR 0030, decision 7). The toolchain does not ship wasi-libc's license files, so they are kept here, unchanged, from https://github.com/WebAssembly/wasi-libc at commit `165235bc467d5fa52d424f5d82587dfb76ed9d54` (2026-10-11):

- `LICENSE`, `LICENSE-APACHE`, `LICENSE-APACHE-LLVM`, `LICENSE-MIT`: from the repository's root;
- `cloudlibc-LICENSE`: `libc-bottom-half/cloudlibc/LICENSE`;
- `musl-COPYRIGHT`: `libc-top-half/musl/COPYRIGHT`.

`scripts/notices.mjs` copies them into the site's `THIRD-PARTY-NOTICES.txt`.
