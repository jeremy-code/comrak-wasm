use comrak::ResolvedReference as ComrakResolvedReference;
use comrak::options::{
    BrokenLinkCallback as ComrakBrokenLinkCallback,
    BrokenLinkReference as ComrakBrokenLinkReference,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

struct BrokenLinkCallback(js_sys::Function);

// TODO: There has be a better way to do this. `BrokenLinkReference` I
// understand needing a separate struct, since its members are type `&'l str`,
// but there should be a way to deserialize `ResolvedReference` into its comrak
// struct without the intermediary.

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

impl ComrakBrokenLinkCallback for BrokenLinkCallback {
    fn resolve(
        &self,
        broken_link_reference: ComrakBrokenLinkReference,
    ) -> Option<ComrakResolvedReference> {
        let js_value = self
            .0
            .call1(
                &JsValue::UNDEFINED,
                &serde_wasm_bindgen::to_value(&BrokenLinkReference {
                    normalized: broken_link_reference.normalized.to_string(),
                    original: broken_link_reference.original.to_string(),
                })
                .ok()?,
            )
            .ok()?;

        if js_value.is_null_or_undefined() {
            return None;
        }

        let resolved_reference: ResolvedReference =
            serde_wasm_bindgen::from_value(js_value).ok()?;

        Some(ComrakResolvedReference {
            url: resolved_reference.url,
            title: resolved_reference.title,
        })
    }
}

pub fn deserialize<'de, 'c, D>(
    deserializer: D,
) -> Result<Option<Arc<dyn ComrakBrokenLinkCallback + 'c>>, D::Error>
where
    D: Deserializer<'de>,
{
    let js_value: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

    if js_value.is_null_or_undefined() {
        return Ok(None);
    }

    let broken_link_callback = js_value.dyn_into::<js_sys::Function>().map_err(|_| {
        serde::de::Error::custom("Expected a function for the broken link callback option")
    })?;

    Ok(Some(Arc::new(BrokenLinkCallback(broken_link_callback))))
}
