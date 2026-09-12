use comrak::nodes::{LineColumn as ComrakLineColumn, Sourcepos as ComrakSourcepos};
use serde::Serialize;
use tsify::Tsify;

#[derive(Tsify, Serialize)]
#[serde(remote = "ComrakLineColumn")]
/// Represents the 1-based line and column positions of a given character.
///
/// By default, the `column` value is measured in UTF-8 byte offsets (1-based),
/// matching cmark behavior. This means multi-byte UTF-8 characters
/// (for example, `ö` or `好`) increase the column count by their byte
/// length rather than by Rust's `char` count.
///
/// Enable [`parse.sourcepos_chars`][crate::options::Parse#structfield.sourcepos_chars] to have
/// column values reported as a Unicode character count instead.
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
/// Represents the position in the source Markdown this node was rendered from.
pub struct Sourcepos {
    #[serde(with = "LineColumn")]
    #[tsify(type = "LineColumn")]
    /// The line and column of the first character of this node.
    pub start: ComrakLineColumn,
    #[tsify(type = "LineColumn")]
    #[serde(with = "LineColumn")]
    /// The line and column of the last character of this node.
    pub end: ComrakLineColumn,
}

#[derive(Serialize)]
pub struct SourceposHelper(#[serde(with = "Sourcepos")] pub ComrakSourcepos);
