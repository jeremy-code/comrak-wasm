# comrak-wasm

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
