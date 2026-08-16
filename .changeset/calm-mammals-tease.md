---
"comrak-wasm": patch
---

fix: enable panic="unwind" and propagate errors in url_rewriter.rs, broken_link_callback.rs

- Enable panic=unwind for better error handling
- url_rewriter.rs, broken_link_callback.rs now panic when an error is thrown in their callbacks
