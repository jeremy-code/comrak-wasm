---
"comrak-wasm": patch
---

refactor: rewrite for improved clarity and consistency

- In lib.rs, imports from comrak use "comrak::" syntax. Everywhere else, it uses `as Comrak*` syntax. The reason for this is because Tsify seems to not have an easy way to rename a type
- Use more explicit naming schemes
- Remove the js from js_url_rewriter
- Use is_null_or_undefined
