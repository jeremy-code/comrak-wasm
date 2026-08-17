use crate::nodes::SourceposHelper;
use comrak::adapters::CodefenceRendererAdapter as ComrakCodefenceRendererAdapter;
use comrak::nodes::Sourcepos as ComrakSourcepos;
use js_sys::Function;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_CODEFENCE_RENDERER: &'static str = r#"
    export type CodefenceRendererAdapter = (lang: string, meta: string, code: string, sourcepos: Sourcepos | undefined) => string;
"#;

#[derive(Deserialize)]
struct NestedFunction {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub inner: Function,
}

struct CodefenceRenderer(Function);

impl ComrakCodefenceRendererAdapter for CodefenceRenderer {
    fn write(
        &self,
        output: &mut dyn fmt::Write,
        lang: &str,
        meta: &str,
        code: &str,
        sourcepos: Option<ComrakSourcepos>,
    ) -> fmt::Result {
        let sourcepos_js = sourcepos
            .and_then(|sourcepos| serde_wasm_bindgen::to_value(&SourceposHelper(sourcepos)).ok())
            .unwrap_or(JsValue::UNDEFINED);

        let html = self
            .0
            .call4(
                &JsValue::UNDEFINED,
                &JsValue::from_str(lang),
                &JsValue::from_str(meta),
                &JsValue::from_str(code),
                &sourcepos_js,
            )
            .ok()
            .and_then(|result| result.as_string());

        match html {
            Some(html) => output.write_str(&html),
            None => comrak::html::escape(output, code),
        }
    }
}

pub fn deserialize<'de, 'p, D>(
    deserializer: D,
) -> Result<HashMap<String, &'p dyn ComrakCodefenceRendererAdapter>, D::Error>
where
    D: Deserializer<'de>,
{
    let js_value: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

    if js_value.is_null_or_undefined() {
        return Ok(HashMap::new());
    }

    let heading_adapter: HashMap<String, NestedFunction> = serde_wasm_bindgen::from_value(js_value)
        .map_err(|err| {
            serde::de::Error::custom(format!("Failed to deserialize heading adapter: {err}"))
        })?;

    let contacts: HashMap<String, &'p dyn ComrakCodefenceRendererAdapter> = heading_adapter
        .into_iter()
        .map(|(lang, nested_function)| {
            let renderer: &'p dyn ComrakCodefenceRendererAdapter =
                Box::leak(Box::new(CodefenceRenderer(nested_function.inner)));
            (lang, renderer)
        })
        .collect();

    Ok(contacts)
}
