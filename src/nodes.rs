use comrak::nodes::{LineColumn as ComrakLineColumn, Sourcepos as ComrakSourcepos};
use serde::Serialize;
use tsify::Tsify;

#[derive(Tsify, Serialize)]
#[serde(remote = "ComrakLineColumn")]
pub struct LineColumn {
    /// The 1-based line number of the character.
    pub line: usize,
    /// The 1-based column number of the character.
    ///
    /// By default this is counted in UTF-8 bytes (so a 3-byte character
    /// increments the column by 3).  Enable
    /// [`parse.sourcepos_chars`][crate::options::Parse#structfield.sourcepos_chars] to have
    /// it reported as a Unicode character count instead.
    pub column: usize,
}

#[derive(Tsify, Serialize)]
#[serde(remote = "ComrakSourcepos")]
pub struct Sourcepos {
    #[serde(with = "LineColumn")]
    #[tsify(type = "LineColumn")]
    pub start: ComrakLineColumn,
    #[tsify(type = "LineColumn")]
    #[serde(with = "LineColumn")]
    pub end: ComrakLineColumn,
}
