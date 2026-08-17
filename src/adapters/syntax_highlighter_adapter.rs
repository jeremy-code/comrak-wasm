use comrak::adapters::SyntaxHighlighterAdapter as ComrakSyntaxHighlighterAdapter;
use js_sys::Function;
use serde::Deserialize;
use serde::Deserializer;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SyntaxHighlighterAdapter {
    #[tsify(type = "(lang: string | undefined, code: string) => string")]
    #[serde(with = "serde_wasm_bindgen::preserve")]
    write_highlighted: Function<fn(JsValue, JsValue) -> JsValue>,
    #[tsify(type = "(attributes: Map<string, string>) => string")]
    #[serde(with = "serde_wasm_bindgen::preserve")]
    write_pre_tag: Function<fn(JsValue) -> JsValue>,
    #[tsify(type = "(attributes: Map<string, string>) => string")]
    #[serde(with = "serde_wasm_bindgen::preserve")]
    write_code_tag: Function<fn(JsValue) -> JsValue>,
}

impl ComrakSyntaxHighlighterAdapter for SyntaxHighlighterAdapter {
    fn write_highlighted(
        &self,
        output: &mut dyn fmt::Write,
        lang: Option<&str>,
        code: &str,
    ) -> fmt::Result {
        let lang_js = lang.map_or(JsValue::UNDEFINED, JsValue::from_str);

        let html = self
            .write_highlighted
            .call2(&JsValue::UNDEFINED, &lang_js, &JsValue::from_str(code))
            .ok()
            .and_then(|result| result.as_string());

        match html {
            Some(html) => output.write_str(&html),
            // Fall back to writing the escaped, unhighlighted code so that
            // content isn't silently dropped if the adapter fails. Never
            // return `Err` here: comrak's top-level render functions unwrap
            // this Result, so an `Err` would panic (and abort) instead of
            // just losing highlighting for this block.
            None => comrak::html::escape(output, code),
        }
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn fmt::Write,
        attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        let js_attributes = serde_wasm_bindgen::to_value(&attributes)
            .unwrap_or_else(|_| js_sys::Map::new().upcast_into());

        let html = self
            .write_pre_tag
            .call1(&JsValue::UNDEFINED, &js_attributes)
            .ok()
            .and_then(|result| result.as_string());

        output.write_str(&html.unwrap_or_else(|| "<pre>".to_owned()))
    }

    fn write_code_tag(
        &self,
        output: &mut dyn fmt::Write,
        attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        let js_attributes = serde_wasm_bindgen::to_value(&attributes)
            .unwrap_or_else(|_| js_sys::Map::new().upcast_into());
        let html = self
            .write_code_tag
            .call1(&JsValue::UNDEFINED, &js_attributes)
            .ok()
            .and_then(|result| result.as_string());

        output.write_str(&html.unwrap_or_else(|| "<code>".to_owned()))
    }
}

pub fn deserialize<'de, 'p, D>(
    deserializer: D,
) -> Result<Option<&'p dyn ComrakSyntaxHighlighterAdapter>, D::Error>
where
    D: Deserializer<'de>,
{
    let js_value: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

    if js_value.is_null_or_undefined() {
        return Ok(None);
    }

    let syntax_highlighter_adapter: SyntaxHighlighterAdapter =
        serde_wasm_bindgen::from_value(js_value).map_err(|err| {
            serde::de::Error::custom(format!(
                "Failed to deserialize syntax highlighter adapter: {err}"
            ))
        })?;

    Ok(Some(Box::leak(Box::new(syntax_highlighter_adapter))))
}
