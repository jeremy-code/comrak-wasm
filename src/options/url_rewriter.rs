use comrak::options::URLRewriter as ComrakURLRewriter;
use js_sys::{Function, JsString};
use serde::Deserializer;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

struct URLRewriter(Function<fn(JsString) -> JsString>);

impl ComrakURLRewriter for URLRewriter {
    fn to_html(&self, url: &str) -> String {
        self.0
            .call1(&JsValue::UNDEFINED, &JsString::from(url))
            .expect("An exception was thrown while attempting to resolve a URLRewriter")
            .as_string()
            .unwrap_or_else(|| url.to_owned())
    }
}

pub fn deserialize<'de, 'c, D>(
    deserializer: D,
) -> Result<Option<Arc<dyn ComrakURLRewriter + 'c>>, D::Error>
where
    D: Deserializer<'de>,
{
    let js_value: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

    if js_value.is_null_or_undefined() {
        return Ok(None);
    }

    let url_rewriter = js_value
        .dyn_into::<Function<fn(JsString) -> JsString>>()
        .map_err(|_| serde::de::Error::custom("Expected a function for the URL rewriter option"))?;

    Ok(Some(Arc::new(URLRewriter(url_rewriter))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::JsString;
    use wasm_bindgen_test::*;

    #[wasm_bindgen(inline_js = r#"
export const create_url_rewriter = () => (url) =>
    url.endsWith(".md") ? url.slice(0, url.length - ".md".length) : url;
"#)]
    extern "C" {
        fn create_url_rewriter() -> Function<fn(JsString) -> JsString>;
    }

    #[wasm_bindgen_test]
    fn test_url_rewriter() {
        let url_rewriter = URLRewriter(create_url_rewriter());
        assert_eq!(
            url_rewriter.to_html("https://url.example/blog/article.md"),
            "https://url.example/blog/article"
        );
        assert_eq!(
            url_rewriter.to_html("https://url.example/blog/assets/image.jpg"),
            "https://url.example/blog/assets/image.jpg"
        );
    }
}
