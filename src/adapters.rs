pub mod codefence_renderer_adapter;
pub mod heading_adapter;
use comrak::adapters::HeadingMeta as ComrakHeadingMeta;
use serde::Serialize;
use tsify::Tsify;

#[derive(Tsify, Serialize)]
#[serde(remote = "ComrakHeadingMeta")]
pub struct HeadingMeta {
    pub level: u8,
    pub content: String,
}

#[derive(Serialize)]
struct HeadingMetaHelper(#[serde(with = "HeadingMeta")] ComrakHeadingMeta);
