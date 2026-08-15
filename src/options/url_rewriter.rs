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
        .dyn_into::<Function<fn(JsString) -> JsString>>()
        .map_err(|_| serde::de::Error::custom("Expected a function for the URL rewriter option"))?;

    Ok(Some(Arc::new(URLRewriter(url_rewriter))))
}
