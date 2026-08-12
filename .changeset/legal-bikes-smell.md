---
"comrak-wasm": patch
---

feat: support BrokenLinkCallback function as an option

- Create BrokenLinkCallback to deserialize from `js_sys::Function` to `Option<Arc<dyn BrokenLinkCallback + 'c>>`
- Update broken_link_callback in Options.extension to deserialize with `broken_link_callback` module
