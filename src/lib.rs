mod options;

use options::Options;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct OptionsHelper<'c>(#[serde(with = "Options")] comrak::Options<'c>);

#[wasm_bindgen]
pub fn version() -> String {
    comrak::version().to_owned()
}

#[wasm_bindgen(js_name = markdownToCommonmarkXml)]
pub fn markdown_to_commonmark_xml(
    md: &str,
    // Otherwise, wasm_bindgen outputs `any` in TypeScript
    // `options` is optional, but can't be represented by wasm_bindgen
    #[wasm_bindgen(unchecked_param_type = "Options | undefined")] options: JsValue,
) -> Result<String, JsError> {
    let comrak_options: comrak::Options = match options.is_undefined() {
        false => serde_wasm_bindgen::from_value(options).map(|OptionsHelper(options)| options)?,
        true => comrak::Options::default(),
    };

    Ok(comrak::markdown_to_commonmark_xml(md, &comrak_options))
}

#[wasm_bindgen(js_name = markdownToHtml)]
pub fn markdown_to_html(
    md: &str,
    #[wasm_bindgen(unchecked_param_type = "Options | undefined")] options: JsValue,
) -> Result<String, JsError> {
    let comrak_options: comrak::Options = match options.is_undefined() {
        false => serde_wasm_bindgen::from_value(options).map(|OptionsHelper(options)| options)?,
        true => comrak::Options::default(),
    };

    Ok(comrak::markdown_to_html(md, &comrak_options))
}
