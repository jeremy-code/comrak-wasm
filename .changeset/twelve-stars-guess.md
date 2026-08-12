---
"comrak-wasm": patch
---

feat: support URLRewriter function as an option

- Create JsURLRewriter to deserialize from `js_sys::Function` to `Option<Arc<dyn URLRewriter + 'c>>`
- Update image_url_rewriter and link_url_rewriter in Options.extension to deserialize with `js_url_rewriter` module
