# Compatibility

The WebAssembly features that are enabled for this package include [those enabled by default in Rust/LLVM](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html#enabled-webassembly-features) (`multivalue`, `mutable-globals`, `reference-types`, `sign-ext`, `nontrapping-fptoint`, `bulk-memory`) and fixed-width SIMD.

A table of the support of those features based on [WebAssembly feature data](https://webassembly.org/features/) can be found below. The minimum supported environments are: Chrome 96, Firefox 89, Safari 15, Node.js 17.2, and Deno 1.16.

| Feature                                                      | Chrome | Firefox | Safari   | Node.js | Deno  |
| :----------------------------------------------------------- | ------ | ------- | -------- | ------- | ----- |
| [Bulk Memory Operations][bulk-memory]                        | 75     | 79      | 15       | 12.5    | 0.4   |
| [Multi-value][multivalue]                                    | 85     | 78      | 13.1     | 15      | 1.3.2 |
| [Import/Export of Mutable Globals][mutable-globals]          | 74     | 61      | 13.1     | 12      | 0.1   |
| [Reference Types][reference-types]                           | 96     | 79      | 15       | 17.2    | 1.16  |
| [Non-trapping float-to-int Conversions][nontrapping-fptoint] | 75     | 64      | 15       | 12.5    | 0.4   |
| [Sign-extension Operators][sign-ext]                         | 74     | 62      | 14.1[^1] | 12      | 0.1   |
| [Fixed-width SIMD][simd128]                                  | 91     | 89      | 16.4     | 16.4    | 1.9   |
|                                                              | 96     | 89      | 16.4     | 17.2    | 1.16  |

[bulk-memory]: https://github.com/WebAssembly/bulk-memory-operations/blob/master/proposals/bulk-memory-operations/Overview.md
[multivalue]: https://github.com/WebAssembly/spec/blob/master/proposals/multi-value/Overview.md
[mutable-globals]: https://github.com/WebAssembly/mutable-global/blob/master/proposals/mutable-global/Overview.md
[nontrapping-fptoint]: https://github.com/WebAssembly/spec/blob/master/proposals/nontrapping-float-to-int-conversion/Overview.md
[sign-ext]: https://github.com/WebAssembly/spec/blob/master/proposals/sign-extension-ops/Overview.md
[simd128]: https://github.com/WebAssembly/simd/blob/master/proposals/simd/SIMD.md

[^1]: Supported in desktop Safari since 14.1 and iOS Safari since 14.5

As of [Browserslist](https://browsersl.ist/#q=fully+supports+wasm-bulk-memory+and+fully+supports+wasm-multi-value+and+fully+supports+wasm-mutable-globals+and+fully+supports+wasm-reference-types+and+fully+supports+wasm-nontrapping-fptoint+and+fully+supports+wasm-signext+and+fully+supports+wasm-simd) 4.28.8, these Wasm features are supported by 94.9 % of global browsers.
