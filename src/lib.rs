mod options;

use options::Options;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct OptionsHelper<'c>(#[serde(with = "Options")] comrak::Options<'c>);

#[inline]
fn deserialize_options<'c>(
    js_value: JsValue,
) -> Result<comrak::Options<'c>, serde_wasm_bindgen::Error> {
    match js_value.is_null_or_undefined() {
        false => serde_wasm_bindgen::from_value(js_value).map(|OptionsHelper(options)| options),
        true => Ok(comrak::Options::default()),
    }
}

#[wasm_bindgen]
/// Return the version of the crate.
pub fn version() -> js_sys::JsString {
    js_sys::JsString::from(comrak::version())
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
    let comrak_options = deserialize_options(options)?;

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
    let comrak_options = deserialize_options(options)?;

    Ok(comrak::markdown_to_html(md, &comrak_options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;
    use wasm_bindgen_test::*;

    #[wasm_bindgen(inline_js = r#"
export const create_options = () => ({
    extension: {
        strikethrough: true,
        imageUrlRewriter: (url) =>
            new URL(url, "https://www.example.com").toString(),
    },
    render: {
        alertStyle: "semantic",
    },
    parse: {
        smart: true,
    },
});
"#)]
    extern "C" {
        fn create_options() -> JsValue;
    }

    #[wasm_bindgen_test]
    fn test_deserialize_options() {
        let options = deserialize_options(create_options()).unwrap();
        assert!(options.extension.strikethrough);
        assert_matches!(
            options.render.alert_style,
            comrak::options::AlertStyleType::Semantic
        );
        assert!(options.parse.smart);

        let image_url_rewriter = options.extension.image_url_rewriter.as_ref().unwrap();

        assert_eq!(
            image_url_rewriter.to_html("image.png"),
            "https://www.example.com/image.png"
        );
    }
}
