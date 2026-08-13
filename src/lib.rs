mod options;

use options::Options;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct OptionsHelper<'c>(#[serde(with = "Options")] comrak::Options<'c>);

#[wasm_bindgen]
/// Return the version of the crate.
pub fn version() -> String {
    comrak::version().to_owned()
}

#[wasm_bindgen(js_name = markdownToCommonmarkXml)]
/// Render Markdown to CommonMark XML.
///
/// See <https://github.com/commonmark/commonmark-spec/blob/master/CommonMark.dtd>.
pub fn markdown_to_commonmark_xml(
    md: &str,
    // Otherwise, wasm_bindgen outputs `any` in TypeScript
    #[wasm_bindgen(unchecked_optional_param_type = "Options | undefined | null")] options: JsValue,
) -> Result<String, JsError> {
    let comrak_options: comrak::Options = match options.is_null_or_undefined() {
        false => serde_wasm_bindgen::from_value(options).map(|OptionsHelper(options)| options)?,
        true => comrak::Options::default(),
    };

    Ok(comrak::markdown_to_commonmark_xml(md, &comrak_options))
}

#[wasm_bindgen(js_name = markdownToHtml)]
/// Render Markdown to HTML.
///
/// See the documentation of the crate root for an example.
pub fn markdown_to_html(
    md: &str,
    #[wasm_bindgen(unchecked_optional_param_type = "Options | undefined | null")] options: JsValue,
) -> Result<String, JsError> {
    let comrak_options: comrak::Options = match options.is_null_or_undefined() {
        false => serde_wasm_bindgen::from_value(options).map(|OptionsHelper(options)| options)?,
        true => comrak::Options::default(),
    };

    Ok(comrak::markdown_to_html(md, &comrak_options))
}
