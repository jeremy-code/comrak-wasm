use comrak::options::URLRewriter as ComrakURLRewriter;
use serde::Deserializer;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

struct URLRewriter(js_sys::Function);

impl ComrakURLRewriter for URLRewriter {
    fn to_html(&self, url: &str) -> String {
        self.0
            .call1(&JsValue::UNDEFINED, &JsValue::from_str(url))
            .ok()
            .and_then(|result| result.as_string())
            .unwrap_or_else(|| url.to_string())
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
        .dyn_into::<js_sys::Function>()
        .map_err(|_| serde::de::Error::custom("Expected a function for the URL rewriter option"))?;

    Ok(Some(Arc::new(URLRewriter(url_rewriter))))
}
