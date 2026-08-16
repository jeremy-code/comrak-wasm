# comrak-wasm

## 0.0.6

### Patch Changes

- 405b656: fix: enable panic="unwind" and propagate errors in url_rewriter.rs, broken_link_callback.rs

    - Enable panic=unwind for better error handling
    - url_rewriter.rs, broken_link_callback.rs now panic when an error is thrown in their callbacks

- a5de851: test: add integration tests
- 95d17f6: chore: export escape_commonmark_inline, escape_commonmark_link_destination
- f65f07d: test: add unit tests for broken_link_callback.rs, url_rewriter.rs
- d6ce613: chore: use explicit Function types in broken_link_callback.rs, url_rewriter.rs
- 22032ac: refactor: move deserialize_options to an inline function
- 505ea7d: chore: enable SIMD WebAssembly SIMD
- 00e7c19: build: update wasm-bindgen-test to 0.3.77

## 0.0.5

### Patch Changes

- 62402d1: chore: install wasm-opt via npm in CI to avoid outdated Ubuntu installation

    - It seems that the binaryen installation in Ubuntu is outdated, causing
      "Uncaught RangeError: failed to grow table." Hence, installing with npm.
      [wasm-bindgen/wasm-pack#1446](https://github.com/wasm-bindgen/wasm-pack/issues/1446)
      [WebAssembly/binaryen#4711](https://github.com/WebAssembly/binaryen/issues/4711)

## 0.0.4

### Patch Changes

- 2ff4e0b: chore: add comments to structs/interfaces

## 0.0.3

### Patch Changes

- d3866a7: chore(deps): update deps
- 4822b8e: feat: rewrite broken_link_callback to directly deserialize/serialize structs
- ca598f6: feat: make options params optional, allow null in options

## 0.0.2

### Patch Changes

- 056075c: refactor: rewrite for improved clarity and consistency

    - In lib.rs, imports from comrak use "comrak::" syntax. Everywhere else, it uses `as Comrak*` syntax. The reason for this is because Tsify seems to not have an easy way to rename a type
    - Use more explicit naming schemes
    - Remove the js from js_url_rewriter
    - Use is_null_or_undefined

- 04f881b: feat: support BrokenLinkCallback function as an option

    - Create BrokenLinkCallback to deserialize from `js_sys::Function` to `Option<Arc<dyn BrokenLinkCallback + 'c>>`
    - Update broken_link_callback in Options.extension to deserialize with `broken_link_callback` module

- 7495b98: feat: support URLRewriter function as an option

    - Create JsURLRewriter to deserialize from `js_sys::Function` to `Option<Arc<dyn URLRewriter + 'c>>`
    - Update image_url_rewriter and link_url_rewriter in Options.extension to deserialize with `js_url_rewriter` module

## 0.0.1

### Patch Changes

- bdf3396: feat: add version export in lib.rs
- 792d7b8: chore: use comrak:: imports to simplify import names
- ea62e3b: chore: rename all members in Options struct to camelCase
- ff519db: chore: rename Helper struct to OptionsHelper
