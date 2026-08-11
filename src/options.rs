use comrak::options::{
    AlertStyleType as ComrakAlertStyleType, BrokenLinkCallback, Extension as ComrakExtension,
    ListStyleType as ComrakListStyleType, Parse as ComrakParse, Render as ComrakRender,
    URLRewriter,
};
use serde::Deserialize;
use std::sync::Arc;
use tsify::Tsify;

use comrak::Options as ComrakOptions;

#[derive(Tsify, Deserialize, Default)]
#[serde(default)]
#[serde(remote = "ComrakOptions")]
pub struct Options<'c> {
    #[serde(with = "Extension")]
    #[tsify(type = "Extension")]
    pub extension: ComrakExtension<'c>,
    #[serde(with = "Parse")]
    #[tsify(type = "Parse")]
    pub parse: ComrakParse<'c>,
    #[serde(with = "Render")]
    #[tsify(type = "Render")]
    pub render: ComrakRender,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(default)]
#[serde(remote = "ComrakExtension")]
#[serde(rename_all = "camelCase")]
pub struct Extension<'c> {
    pub strikethrough: bool,
    pub tagfilter: bool,
    pub table: bool,
    pub autolink: bool,
    pub tasklist: bool,
    pub superscript: bool,
    pub header_id_prefix: Option<String>,
    pub header_id_prefix_in_href: bool,
    pub footnotes: bool,
    pub inline_footnotes: bool,
    pub description_lists: bool,
    pub front_matter_delimiter: Option<String>,
    pub multiline_block_quotes: bool,
    pub alerts: bool,
    pub math_dollars: bool,
    pub math_latex: bool,
    pub math_code: bool,
    pub shortcodes: bool,
    pub wikilinks_title_after_pipe: bool,
    pub wikilinks_title_before_pipe: bool,
    pub underline: bool,
    pub subscript: bool,
    pub spoiler: bool,
    pub greentext: bool,
    #[serde(skip)]
    pub image_url_rewriter: Option<Arc<dyn URLRewriter + 'c>>,
    #[serde(skip)]
    pub link_url_rewriter: Option<Arc<dyn URLRewriter + 'c>>,
    pub cjk_friendly_emphasis: bool,
    pub subtext: bool,
    pub highlight: bool,
    pub insert: bool,
    pub phoenix_heex: bool,
    pub block_directive: bool,
    pub header_attributes: bool,
    pub fenced_code_attributes: bool,
    pub inline_code_attributes: bool,
    pub link_attributes: bool,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(default)]
#[serde(remote = "ComrakParse")]
#[serde(rename_all = "camelCase")]
pub struct Parse<'c> {
    pub smart: bool,
    pub default_info_string: Option<String>,
    pub relaxed_tasklist_matching: bool,
    pub tasklist_in_table: bool,
    pub relaxed_autolinks: bool,
    pub ignore_setext: bool,
    #[serde(skip)]
    pub broken_link_callback: Option<Arc<dyn BrokenLinkCallback + 'c>>,
    pub leave_footnote_definitions: bool,
    pub escaped_char_spans: bool,
    pub sourcepos_chars: bool,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(remote = "ComrakListStyleType")]
#[serde(rename_all = "camelCase")]
pub enum ListStyleType {
    /// The `-` character
    #[default]
    Dash = 45,
    /// The `+` character
    Plus = 43,
    /// The `*` character
    Star = 42,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(remote = "ComrakAlertStyleType")]
#[serde(rename_all = "camelCase")]
pub enum AlertStyleType {
    /// `div`s with `class="markdown-alert markdown-alert-<type>"`
    #[default]
    Specific,
    /// `aside`s with `class="admonition <type>"`, matching `docutils`' output
    Semantic,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
#[serde(remote = "ComrakRender")]
pub struct Render {
    pub hardbreaks: bool,
    pub github_pre_lang: bool,
    pub full_info_string: bool,
    pub width: usize,
    pub r#unsafe: bool,
    pub escape: bool,
    #[serde(with = "ListStyleType")]
    #[tsify(type = "ListStyleType")]
    pub list_style: ComrakListStyleType,
    pub sourcepos: bool,
    pub escaped_char_spans: bool,
    pub ignore_empty_links: bool,
    pub gfm_quirks: bool,
    pub prefer_fenced: bool,
    pub figure_with_caption: bool,
    pub tasklist_classes: bool,
    #[serde(with = "AlertStyleType")]
    #[tsify(type = "AlertStyleType")]
    pub alert_style: ComrakAlertStyleType,
    pub ol_width: usize,
    pub experimental_minimize_commonmark: bool,
    pub compact_html: bool,
}
