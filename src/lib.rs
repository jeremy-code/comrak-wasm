mod nodes;
mod options;

use nodes::NodeValue;
use options::Options;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct OptionsHelper<'c>(#[serde(with = "Options")] comrak::Options<'c>);

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

#[derive(Serialize)]
struct NodeValueHelper(#[serde(with = "NodeValue")] comrak::nodes::NodeValue);

#[wasm_bindgen]
pub fn parse_document() {
    let arena = comrak::Arena::new();
    let document = comrak::parse_document(
        &arena,
        "# Hello world

This is **Markdown**",
        &comrak::Options::default(),
    );
    // The nodes can't be deserialized directly due to being cyclic (reference
    // to parent and child), so theoretically, walk down all descendantss
    // and make the connections in JS manually
    for node in document.descendants() {
        let js_value = serde_wasm_bindgen::to_value(&NodeValueHelper(node.data().value.clone()));
        // Document
        // { Heading: { level: 1, setext: false, closed: false } }
        // { Text: 'Hello world' }
        // Paragraph
        // { Text: 'This is ' }
        // Strong
        // { Text: 'Markdown' }
        log(&js_value.unwrap())
    }
}

#[wasm_bindgen]
/// Return the version of the crate.
pub fn version() -> String {
    comrak::version().to_owned()
}

#[wasm_bindgen(js_name = escapeCommmonmarkInline)]
/// Escapes the input, rendering it suitable for inclusion in a CommonMark
/// document in a place where regular inline parsing is occurring. Note that
/// this is not minimal --- there will be more escaping backslashes in the
/// output than is strictly necessary. The rendering will not be affected,
/// however.
pub fn escape_commonmark_inline(text: &str) -> String {
    comrak::escape_commonmark_inline(text)
}

#[wasm_bindgen(js_name = escapeCommonmarkLinkDestination)]
/// Escapes the input URL, rendering it suitable for inclusion as a [link
/// destination] per the CommonMark spec.
///
/// [link destination]: https://spec.commonmark.org/0.31.2/#link-destination
pub fn escape_commonmark_link_destination(url: &str) -> String {
    comrak::escape_commonmark_link_destination(url)
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
