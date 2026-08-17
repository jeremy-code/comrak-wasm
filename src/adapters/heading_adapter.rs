use comrak::adapters::{HeadingAdapter as ComrakHeadingAdapter, HeadingMeta};
use comrak::nodes::Sourcepos as ComrakSourcepos;
use js_sys::Function;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::adapters::HeadingMetaHelper;
use crate::nodes::Sourcepos;

#[derive(Serialize)]
pub struct SourceposHelper(#[serde(with = "Sourcepos")] ComrakSourcepos);

#[derive(Tsify, Deserialize)]
struct HeadingAdapter {
    #[tsify(type = "(heading: HeadingMeta, sourcepos: Sourcepos | undefined) => string")]
    #[serde(with = "serde_wasm_bindgen::preserve")]
    enter: Function,
    #[tsify(type = "(heading: HeadingMeta) => string")]
    #[serde(with = "serde_wasm_bindgen::preserve")]
    exit: Function,
}

impl ComrakHeadingAdapter for HeadingAdapter {
    fn enter(
        &self,
        output: &mut dyn fmt::Write,
        heading: &HeadingMeta,
        sourcepos: Option<ComrakSourcepos>,
    ) -> fmt::Result {
        let js_heading =
            serde_wasm_bindgen::to_value(&HeadingMetaHelper(heading.to_owned())).unwrap();
        let js_sourcepos = match sourcepos {
            Some(i) => serde_wasm_bindgen::to_value(&SourceposHelper(i)).unwrap(),
            None => JsValue::UNDEFINED,
        };

        let string_output = self
            .enter
            .call2(&JsValue::UNDEFINED, &js_heading, &js_sourcepos)
            .ok()
            .and_then(|result| result.as_string())
            .unwrap();
        output.write_str(&string_output)
    }

    fn exit(&self, output: &mut dyn fmt::Write, heading: &HeadingMeta) -> fmt::Result {
        let js_heading =
            serde_wasm_bindgen::to_value(&HeadingMetaHelper(heading.to_owned())).unwrap();
        let string_output = self
            .exit
            .call1(&JsValue::UNDEFINED, &js_heading)
            .ok()
            .and_then(|result| result.as_string())
            .unwrap();
        output.write_str(&string_output)
    }
}

pub fn deserialize<'de, 'p, D>(
    deserializer: D,
) -> Result<Option<&'p dyn ComrakHeadingAdapter>, D::Error>
where
    D: Deserializer<'de>,
{
    let js_value: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

    if js_value.is_null_or_undefined() {
        return Ok(None);
    }
    let heading_adapter: HeadingAdapter =
        serde_wasm_bindgen::from_value(js_value).map_err(|err| {
            serde::de::Error::custom(format!("Failed to deserialize heading adapter: {err}"))
        })?;

    // TODO: Find a better way to do this without Box::leak
    Ok(Some(Box::leak(Box::new(heading_adapter))))
}
