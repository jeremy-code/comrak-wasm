use comrak::ResolvedReference as ComrakResolvedReference;
use comrak::options::{
    BrokenLinkCallback as ComrakBrokenLinkCallback,
    BrokenLinkReference as ComrakBrokenLinkReference,
};
use js_sys::Function;
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize)]
#[serde(remote = "ComrakBrokenLinkReference")]
/// Struct to the broken link callback, containing details on the link reference
/// which failed to find a match.
struct BrokenLinkReference<'l> {
    /// The normalized reference link label. Unicode case folding is applied;
    /// see <https://github.com/commonmark/commonmark-spec/issues/695> for a
    /// discussion on the details of what this exactly means.
    pub normalized: &'l str,

    /// The original text in the link label.
    pub original: &'l str,
}

#[derive(Serialize)]
struct BrokenLinkReferenceHelper<'l>(
    #[serde(with = "BrokenLinkReference")] ComrakBrokenLinkReference<'l>,
);

#[derive(Tsify, Deserialize)]
#[serde(remote = "ComrakResolvedReference")]
/// A reference link's resolved details.
///
/// @remarks
/// Due to how structs are formatted data-wise, both `url` and `title` have to
/// be present in the object. If either of them are empty string, that property
/// will not be present in the output
///
/// @see {@link https://github.com/jeremy-code/comrak-wasm/issues/5}
struct ResolvedReference {
    /// The destination URL of the reference link.
    pub url: String,

    /// The text of the link.
    pub title: String,
}

#[derive(Deserialize)]
struct ResolvedReferenceHelper(#[serde(with = "ResolvedReference")] ComrakResolvedReference);

struct BrokenLinkCallback(Function<fn(JsValue) -> JsValue>);

impl ComrakBrokenLinkCallback for BrokenLinkCallback {
    fn resolve(
        &self,
        broken_link_reference: ComrakBrokenLinkReference,
    ) -> Option<ComrakResolvedReference> {
        let js_value = self
            .0
            .call1(
                &JsValue::UNDEFINED,
                &serde_wasm_bindgen::to_value(&BrokenLinkReferenceHelper(broken_link_reference))
                    .ok()?,
            )
            .expect("An exception was thrown while attempting to resolve a BrokenLinkCallback");

        if js_value.is_null_or_undefined() {
            return None;
        }

        let resolved_reference = serde_wasm_bindgen::from_value(js_value)
            .map(|ResolvedReferenceHelper(resolved_reference)| resolved_reference)
            .expect("Invalid resolved reference");

        Some(resolved_reference)
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

    let broken_link_callback = js_value
        .dyn_into::<Function<fn(JsValue) -> JsValue>>()
        .map_err(|_| {
            serde::de::Error::custom("Expected a function for the broken link callback option")
        })?;

    Ok(Some(Arc::new(BrokenLinkCallback(broken_link_callback))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen(inline_js = r#"
export const create_broken_link_callback = () => (broken_link_reference) => {
    if (
        broken_link_reference.normalized === "example" &&
        broken_link_reference.original === "example"
    ) {
        return {
            url: "http://example.com",
            title: "Example",
        };
    }

    return null;
};
"#)]
    extern "C" {
        fn create_broken_link_callback() -> Function<fn(JsValue) -> JsValue>;
    }

    #[wasm_bindgen_test]
    fn test_broken_link_callback() {
        let broken_link_callback = BrokenLinkCallback(create_broken_link_callback());

        let resolved_reference = broken_link_callback.resolve(ComrakBrokenLinkReference {
            normalized: "example",
            original: "example",
        });

        let Some(ComrakResolvedReference { url, title }) = resolved_reference else {
            panic!("expected broken link to be resolved");
        };

        assert_eq!(url, "http://example.com");
        assert_eq!(title, "Example");

        let unresolved_reference = broken_link_callback.resolve(ComrakBrokenLinkReference {
            normalized: "casefolded",
            original: "cAseFolDed",
        });

        assert!(unresolved_reference.is_none());
    }
}
