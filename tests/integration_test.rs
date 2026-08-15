use comrak_wasm::*;

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

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
