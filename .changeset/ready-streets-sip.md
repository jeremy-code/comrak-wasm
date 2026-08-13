---
"comrak-wasm": patch
---

chore: install wasm-opt via npm in CI to avoid outdated Ubuntu installation

- It seems that the binaryen installation in Ubuntu is outdated, causing
  "Uncaught RangeError: failed to grow table." Hence, installing with npm.
  [wasm-bindgen/wasm-pack#1446](https://github.com/wasm-bindgen/wasm-pack/issues/1446)
  [WebAssembly/binaryen#4711](https://github.com/WebAssembly/binaryen/issues/4711)
