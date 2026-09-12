pub mod codefence_renderer_adapter;
pub mod heading_adapter;
pub mod syntax_highlighter_adapter;
use comrak::adapters::HeadingMeta as ComrakHeadingMeta;
use serde::Serialize;
use tsify::Tsify;

#[derive(Tsify, Serialize)]
#[serde(remote = "ComrakHeadingMeta")]
/// The struct passed to the [`HeadingAdapter`] for custom heading implementations.
pub struct HeadingMeta {
    /// The level of the heading; from 1 to 6 for ATX headings, 1 or 2 for setext headings.
    pub level: u8,
    /// The content of the heading as a "flattened" string&mdash;flattened in the sense that any
    /// `<strong>` or other tags are removed. In the Markdown heading `## This is **bold**`, for
    /// example, the would be the string `"This is bold"`.
    pub content: String,
}

#[derive(Serialize)]
struct HeadingMetaHelper(#[serde(with = "HeadingMeta")] ComrakHeadingMeta);
