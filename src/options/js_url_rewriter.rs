use comrak::options::URLRewriter;
use serde::Deserializer;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

struct JsURLRewriter(js_sys::Function);

impl URLRewriter for JsURLRewriter {
    fn to_html(&self, url: &str) -> String {
        self.0
            .call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(url),
            )
            .ok()
            .and_then(|result| result.as_string())
            .unwrap_or_else(|| url.to_string())
    }
}

pub fn deserialize<'de, 'c, D>(
    deserializer: D,
) -> Result<Option<Arc<dyn URLRewriter + 'c>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

    if value.is_undefined() || value.is_null() {
        return Ok(None);
    }

    let func = value
        .dyn_into::<js_sys::Function>()
        .map_err(|_| serde::de::Error::custom("Expected a function for the URL rewriter option"))?;

    Ok(Some(Arc::new(JsURLRewriter(func))))
}
