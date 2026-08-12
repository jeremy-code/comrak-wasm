use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

struct BrokenLinkCallback(js_sys::Function);

#[derive(Tsify, Serialize)]
struct BrokenLinkReference {
    /// The normalized reference link label. Unicode case folding is applied;
    /// see <https://github.com/commonmark/commonmark-spec/issues/695> for a
    /// discussion on the details of what this exactly means.
    pub normalized: String,

    /// The original text in the link label.
    pub original: String,
}

/*
 * Due to how structs are formatted data-wise, both `url` and `title` have to be
 * present in the object. If either of them are empty string, that property will
 * not be present in the output
 */
#[derive(Tsify, Deserialize)]
struct ResolvedReference {
    /// The destination URL of the reference link.
    pub url: String,

    /// The text of the link.
    pub title: String,
}

impl comrak::options::BrokenLinkCallback for BrokenLinkCallback {
    fn resolve(
        &self,
        broken_link_reference: comrak::options::BrokenLinkReference,
    ) -> Option<comrak::ResolvedReference> {
        let reference_value = BrokenLinkReference {
            normalized: broken_link_reference.normalized.to_string(),
            original: broken_link_reference.original.to_string(),
        };

        let result = self
            .0
            .call1(
                &JsValue::UNDEFINED,
                &serde_wasm_bindgen::to_value(&reference_value).ok()?,
            )
            .ok()?;

        if result.is_undefined() || result.is_null() {
            return None;
        }

        let resolved: ResolvedReference = serde_wasm_bindgen::from_value(result).ok()?;
        Some(comrak::ResolvedReference {
            url: resolved.url,
            title: resolved.title,
        })
    }
}

pub fn deserialize<'de, 'c, D>(
    deserializer: D,
) -> Result<Option<Arc<dyn comrak::options::BrokenLinkCallback + 'c>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

    if value.is_undefined() || value.is_null() {
        return Ok(None);
    }

    let func = value.dyn_into::<js_sys::Function>().map_err(|_| {
        serde::de::Error::custom("Expected a function for the broken link callback option")
    })?;

    Ok(Some(Arc::new(BrokenLinkCallback(func))))
}
