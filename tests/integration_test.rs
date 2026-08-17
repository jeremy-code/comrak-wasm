use comrak_wasm::*;

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_escape_commonmark_inline() {
    assert_eq!(
        escape_commonmark_inline("![Image](https://example.com)",),
        r#"\!\[Image\]\(https://example\.com\)"#,
    );
    assert_eq!(
        escape_commonmark_inline(
            r#"In the Wikipedia page for "Scare quotes," the article claims that the editor Greil Marcus had "described scare quotes as 'the enemy.'""#,
        ),
        r#"In the Wikipedia page for \"Scare quotes,\" the article claims that the editor Greil Marcus had \"described scare quotes as 'the enemy\.'\""#,
    );
}

#[wasm_bindgen_test]
fn test_escape_commonmark_link_destination() {
    assert_eq!(
        escape_commonmark_link_destination(r#"link destination) <example>"#,),
        r#"<link destination) \<example\>>"#,
    );
}

#[wasm_bindgen_test]
fn test_markdown_to_html() {
    let result = markdown_to_html(
        include_str!("testdata/test-markdown.md"),
        JsValue::UNDEFINED,
    );
    let html = include_str!("testdata/test-html.html");
    assert_eq!(result.unwrap(), html);
}

#[wasm_bindgen_test]
fn test_markdown_to_html_with_options() {
    let options = js_sys::JSON::parse(include_str!("testdata/options.json")).unwrap();
    let result = markdown_to_html(include_str!("testdata/test-markdown-options.md"), options);
    let html = include_str!("testdata/test-html-options.html");
    assert_eq!(result.unwrap(), html);
}

#[wasm_bindgen_test]
fn test_markdown_to_commonmark_xml() {
    let result = markdown_to_commonmark_xml(
        include_str!("testdata/test-markdown.md"),
        JsValue::UNDEFINED,
    );
    let commonmark_xml = include_str!("testdata/test-commonmark-xml.xml");
    assert_eq!(result.unwrap(), commonmark_xml);
}

#[wasm_bindgen_test]
fn test_markdown_to_commonmark_xml_with_options() {
    let options = js_sys::JSON::parse(include_str!("testdata/options.json")).unwrap();
    let result =
        markdown_to_commonmark_xml(include_str!("testdata/test-markdown-options.md"), options);
    let commonmark_xml = include_str!("testdata/test-commonmark-xml-options.xml");
    assert_eq!(result.unwrap(), commonmark_xml);
}

#[wasm_bindgen(inline_js = r#"
const escapeHtmlTag = (htmlTag) => htmlTag.replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));

export const create_plugins = () => ({
    render: {
        codefenceRenderers: new Map([["mermaid", { write: (lang, meta, code) => `<figure class="${lang}" data-meta="${meta}">${code}</figure>\n` } ]]),
        codefenceSyntaxHighlighter: {
            writeHighlighted: (lang, code) => `<!--${lang}-->${code}</span>`,
            writePreTag: (attributes) =>
                `<pre data-json="${escapeHtmlTag(JSON.stringify(Object.fromEntries(attributes)))}">`,
            writeCodeTag: (attributes) =>
                `<code data-json="${escapeHtmlTag(JSON.stringify(Object.fromEntries(attributes)))}">`,
        },
        headingAdapter: {
            enter: (heading) => `<h${heading.level} data-heading="true">`,
            exit: (heading) => `</h${heading.level}>`,
        },
    },
});
"#)]
extern "C" {
    fn create_plugins() -> JsValue;
}

#[wasm_bindgen_test]
fn test_markdown_to_html_with_plugins() {
    let options = js_sys::JSON::parse(include_str!("testdata/options.json")).unwrap();
    let result = markdown_to_html_with_plugins(
        include_str!("testdata/test-markdown-plugins.md"),
        options,
        create_plugins(),
    );
    let html = include_str!("testdata/test-html-plugins.html");
    assert_eq!(result.unwrap(), html);
}
