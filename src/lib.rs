mod options;

use comrak::Options as ComrakOptions;
use options::Options;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct OptionsHelper<'c>(#[serde(with = "Options")] ComrakOptions<'c>);

#[wasm_bindgen(js_name = markdownToCommonmarkXml)]
pub fn markdown_to_commonmark_xml(
    md: &str,
    // Otherwise, wasm_bindgen outputs `any` in TypeScript
    // `options` is optional, but can't be represented by wasm_bindgen
    #[wasm_bindgen(unchecked_param_type = "Options | undefined")] options: JsValue,
) -> Result<String, JsError> {
    let overrides: ComrakOptions = match options.is_undefined() {
        false => serde_wasm_bindgen::from_value(options).map(|OptionsHelper(options)| options)?,
        true => ComrakOptions::default(),
    };

    Ok(comrak::markdown_to_commonmark_xml(md, &overrides))
}

#[wasm_bindgen(js_name = markdownToHtml)]
pub fn markdown_to_html(
    md: &str,
    #[wasm_bindgen(unchecked_param_type = "Options | undefined")] options: JsValue,
) -> Result<String, JsError> {
    let overrides: ComrakOptions = match options.is_undefined() {
        false => serde_wasm_bindgen::from_value(options).map(|OptionsHelper(options)| options)?,
        true => ComrakOptions::default(),
    };

    Ok(comrak::markdown_to_html(md, &overrides))
}
