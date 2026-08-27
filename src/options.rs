mod broken_link_callback;
mod url_rewriter;

use comrak::Options as ComrakOptions;
use comrak::adapters::HeadingAdapter as ComrakHeadingAdapter;
use comrak::adapters::{CodefenceRendererAdapter, SyntaxHighlighterAdapter};
use comrak::options::{
    AlertStyleType as ComrakAlertStyleType, BrokenLinkCallback, Extension as ComrakExtension,
    ListStyleType as ComrakListStyleType, Parse as ComrakParse, Plugins as ComrakPlugins,
    Render as ComrakRender, RenderPlugins as ComrakRenderPlugins, URLRewriter,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tsify::Tsify;

#[derive(Tsify, Deserialize, Default)]
#[serde(default)]
#[serde(remote = "ComrakOptions")]
#[serde(rename_all = "camelCase")]
/// Umbrella options struct.
pub struct Options<'c> {
    #[serde(with = "Extension")]
    #[tsify(type = "Extension")]
    /// Enable CommonMark extensions.
    pub extension: ComrakExtension<'c>,
    #[serde(with = "Parse")]
    #[tsify(type = "Parse")]
    /// Configure parse-time options.
    pub parse: ComrakParse<'c>,
    #[serde(with = "Render")]
    #[tsify(type = "Render")]
    /// Configure render-time options.
    pub render: ComrakRender,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(default)]
#[serde(remote = "ComrakExtension")]
#[serde(rename_all = "camelCase")]
/// Options to select extensions.
pub struct Extension<'c> {
    /// Enables the
    /// [strikethrough extension](https://github.github.com/gfm/#strikethrough-extension-)
    /// from the GFM spec.
    pub strikethrough: bool,
    /// Enables the
    /// [tagfilter extension](https://github.github.com/gfm/#disallowed-raw-html-extension-)
    /// from the GFM spec.
    pub tagfilter: bool,
    /// Enables the [table extension](https://github.github.com/gfm/#tables-extension-)
    /// from the GFM spec.
    pub table: bool,
    /// Enables the [autolink extension](https://github.github.com/gfm/#autolinks-extension-)
    /// from the GFM spec.
    pub autolink: bool,
    /// Enables the
    /// [task list items extension](https://github.github.com/gfm/#task-list-items-extension-)
    /// from the GFM spec.
    ///
    /// Note that the spec does not define the precise output, so only the bare essentials are
    /// rendered.
    pub tasklist: bool,
    /// Enables the superscript Comrak extension.
    pub superscript: bool,
    /// Enables the header IDs Comrak extension, with the given ID prefix.
    ///
    /// When set, each heading gains an anchor element with an `id` attribute
    /// formed by prefixing the slugified heading text. This is useful for
    /// namespacing heading anchors to avoid collisions when rendered Markdown
    /// is embedded alongside other content on a page (e.g. GitHub uses the
    /// prefix `"user-content-"` for this purpose).
    pub header_id_prefix: Option<String>,
    /// When enabled alongside [`header_id_prefix`](#structfield.header_id_prefix), the header ID
    /// prefix is also applied to the `href` anchor in the generated link.
    ///
    /// Has no effect if `header_id_prefix` is `None`.
    pub header_id_prefix_in_href: bool,
    /// Enables the footnotes extension per `cmark-gfm`.
    ///
    /// For usage, see `src/tests.rs`. The extension is modelled after
    /// [Kramdown](https://kramdown.gettalong.org/syntax.html#footnotes).
    pub footnotes: bool,
    /// Enables the inline footnotes extension.
    ///
    /// Allows inline footnote syntax `^[content]` where the content can include
    /// inline markup. Inline footnotes are automatically converted to regular
    /// footnotes with auto-generated names and share the same numbering sequence.
    ///
    /// Requires `footnotes` to be enabled as well.
    pub inline_footnotes: bool,
    /// Enables the description lists extension.
    ///
    /// Each term must be defined in one paragraph, followed by a blank line,
    /// and then by the details. Details begins with a colon.
    ///
    /// Not (yet) compatible with render.sourcepos.
    ///
    /// ```markdown
    /// First term
    ///
    /// : Details for the **first term**
    ///
    /// Second term
    ///
    /// : Details for the **second term**
    ///
    ///     More details in second paragraph.
    /// ```
    pub description_lists: bool,
    /// Enables the front matter extension.
    ///
    /// Front matter, which begins with the delimiter string at the beginning of the file and ends
    /// at the end of the next line that contains only the delimiter, is passed through unchanged
    /// in markdown output and omitted from HTML output.
    ///
    /// ```markdown
    /// ---
    /// layout: post
    /// title: Formatting Markdown with Comrak
    /// ---
    ///
    /// # Shorter Title
    ///
    /// etc.
    /// ```
    pub front_matter_delimiter: Option<String>,
    /// Enables the multiline block quote extension.
    ///
    /// Place `>>>` before and after text to make it into
    /// a block quote.
    ///
    /// ```markdown
    /// Paragraph one
    ///
    /// >>>
    /// Paragraph two
    ///
    /// - one
    /// - two
    /// >>>
    /// ```
    pub multiline_block_quotes: bool,
    /// Enables GitHub style alerts
    ///
    /// ```md
    /// > [!note]
    /// > Something of note
    /// ```
    pub alerts: bool,
    /// Enables math using dollar syntax.
    ///
    /// ```markdown
    /// Inline math $1 + 2$ and display math $$x + y$$
    ///
    /// $$
    /// x^2
    /// $$
    /// ```
    pub math_dollars: bool,
    /// Enables math using LaTeX-style delimiters.
    ///
    /// ```markdown
    /// Inline math \(1 + 2\) and display math \[x + y\]
    /// ```
    pub math_latex: bool,
    /// Enables math using code syntax.
    ///
    /// ````markdown
    /// Inline math $`1 + 2`$
    ///
    /// ```math
    /// x^2
    /// ```
    pub math_code: bool,
    /// Phrases wrapped inside of ':' blocks will be replaced with emojis.
    pub shortcodes: bool,
    /// Enables wikilinks using title after pipe syntax.
    ///
    /// ```markdown
    /// [[url|link label]]
    /// ```
    ///
    /// When both this option and [`wikilinks_title_before_pipe`][0] are enabled, this option takes
    /// precedence.
    ///
    /// [0]: Self::wikilinks_title_before_pipe
    pub wikilinks_title_after_pipe: bool,
    /// Enables wikilinks using title before pipe syntax.
    ///
    /// ```markdown
    /// [[link label|url]]
    /// ```
    ///
    /// When both this option and [`wikilinks_title_after_pipe`][0] are enabled,
    /// [`wikilinks_title_after_pipe`][0] takes precedence.
    ///
    /// [0]: Self::wikilinks_title_after_pipe
    pub wikilinks_title_before_pipe: bool,
    /// Enables underlines using double underscores
    ///
    /// ```md
    /// __underlined text__
    /// ```
    pub underline: bool,
    /// Enables subscript text using single tildes.
    ///
    /// If the strikethrough option is also enabled, this overrides the single
    /// tilde case to output subscript text.
    ///
    /// ```md
    /// H~2~O
    /// ```
    pub subscript: bool,
    /// Enables spoilers using double vertical bars
    ///
    /// ```md
    /// Darth Vader is ||Luke's father||
    /// ```
    pub spoiler: bool,
    /// Requires at least one space after a `>` character to generate a blockquote,
    /// and restarts blockquote nesting across unique lines of input
    ///
    /// ```md
    /// >implying implications
    ///
    /// > one
    /// > > two
    /// > three
    /// ```
    pub greentext: bool,
    #[serde(with = "url_rewriter")]
    #[tsify(type = "(url: string) => string")]
    /// Wraps embedded image URLs using a function or custom trait object.
    pub image_url_rewriter: Option<Arc<dyn URLRewriter + 'c>>,
    #[serde(with = "url_rewriter")]
    #[tsify(type = "(url: string) => string")]
    /// Wraps link URLs using a function or custom trait object.
    pub link_url_rewriter: Option<Arc<dyn URLRewriter + 'c>>,
    /// Recognizes many emphasis that appear in CJK contexts but are not recognized by plain CommonMark.
    ///
    /// ```md
    /// **この文は重要です。**但这句话并不重要。
    /// ```
    pub cjk_friendly_emphasis: bool,
    /// Enables block scoped subscript that acts similar to a header.
    ///
    /// ```md
    /// -# subtext
    /// ```
    pub subtext: bool,
    /// Enables highlighting (mark) using `==`.
    ///
    /// ```md
    /// Hey, ==this is important!==
    /// ```
    pub highlight: bool,
    /// Enables inserted text using `++`.
    ///
    /// ```md
    /// This is ++added text++
    /// ```
    pub insert: bool,
    /// Enables Phoenix HEEx template syntax support.
    ///
    /// Recognizes Phoenix HEEx directives, tags, and inline expressions.
    pub phoenix_heex: bool,
    /// Enables the container block directive extension.
    ///
    /// Container block directives are container blocks that start and end with `:::`.
    /// The info string after the opening `:::` is used as the block type.
    ///
    /// ```md
    /// :::warning
    /// A paragraph.
    ///
    /// - item one
    /// - item two
    /// :::
    /// ```
    pub block_directive: bool,
    /// Parse attributes in setext and ATX headers.
    pub header_attributes: bool,
    /// Parse attributes in fenced code blocks' info strings.
    pub fenced_code_attributes: bool,
    /// Parse attributes immediately following inline code spans.
    pub inline_code_attributes: bool,
    /// Parse attributes immediately following links and images.
    pub link_attributes: bool,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(default)]
#[serde(remote = "ComrakParse")]
#[serde(rename_all = "camelCase")]
/// Options for parser functions.
pub struct Parse<'c> {
    /// Punctuation (quotes, full-stops and hyphens) are converted into 'smart' punctuation.
    pub smart: bool,
    /// The default info string for fenced code blocks.
    pub default_info_string: Option<String>,
    /// Whether or not a simple `x` or `X` is used for tasklist or any other symbol is allowed.
    pub relaxed_tasklist_matching: bool,
    /// Whether tasklist items can be parsed in table cells. At present, the
    /// tasklist item must be the only content in the cell. Both tables and
    /// tasklists much be enabled for this to work.
    pub tasklist_in_table: bool,
    /// Relax parsing of autolinks, allow links to be detected inside brackets
    /// and allow all url schemes. It is intended to allow a very specific type of autolink
    /// detection, such as `[this http://and.com that]` or `{http://foo.com}`, on a best can basis.
    pub relaxed_autolinks: bool,
    /// Ignore setext headings in input.
    pub ignore_setext: bool,
    #[serde(with = "broken_link_callback")]
    #[tsify(
        type = "(brokenLinkReference: BrokenLinkReference) => ResolvedReference | undefined | null"
    )]
    /// In case the parser encounters any potential links that have a broken
    /// reference (e.g `[foo]` when there is no `[foo]: url` entry at the
    /// bottom) the provided callback will be called with the reference name,
    /// both in normalized form and unmodified, and the returned pair will be
    /// used as the link destination and title if not [`None`].
    pub broken_link_callback: Option<Arc<dyn BrokenLinkCallback + 'c>>,
    /// Leave footnote definitions in place in the document tree, rather than
    /// reordering them to the end.  This will also cause unreferenced footnote
    /// definitions to remain in the tree, rather than being removed.
    ///
    /// Comrak's default formatters expect this option to be turned off, so use
    /// with care if you use the default formatters.
    pub leave_footnote_definitions: bool,
    /// Leave escaped characters in an `Escaped` node in the document tree.
    pub escaped_char_spans: bool,
    /// When enabled, the [`column`][crate::nodes::LineColumn::column] values in
    /// [`Sourcepos`][crate::nodes::Sourcepos] are counted as Unicode characters
    /// (i.e. `char`s) rather than as UTF-8 bytes.
    ///
    /// By default, column values follow cmark behaviour: each byte of a
    /// multi-byte UTF-8 character counts as a separate column. Enabling this
    /// option converts those byte-based columns to character-based columns after
    /// parsing, so that a 3-byte character such as `好` occupies only one
    /// column position instead of three.
    pub sourcepos_chars: bool,
}

#[derive(Tsify, Deserialize, Default)]
#[serde(remote = "ComrakListStyleType")]
#[serde(rename_all = "camelCase")]
/// Options for bulleted list rendering in markdown. See [`Render::list_style`] for more details.
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
/// Options for alert rendering in markdown. See [`Render::alert_style`] for more details.
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
/// Options for formatter functions.
pub struct Render {
    /// [Soft line breaks](http://spec.commonmark.org/0.27/#soft-line-breaks) in the input
    /// translate into hard line breaks in the output.
    pub hardbreaks: bool,
    /// GitHub-style `<pre lang="xyz">` is used for fenced code blocks with info tags.
    pub github_pre_lang: bool,
    /// Enable full info strings for code blocks.
    pub full_info_string: bool,
    /// The wrap column when outputting CommonMark.
    pub width: usize,
    /// Allow rendering of raw HTML and potentially dangerous links.
    pub r#unsafe: bool,
    /// Escape raw HTML instead of clobbering it.
    pub escape: bool,
    /// Set the type of [bullet list marker](https://spec.commonmark.org/0.30/#bullet-list-marker) to use.
    /// Options are:
    ///
    /// * [`ListStyleType::Dash`] to use `-` (default)
    /// * [`ListStyleType::Plus`] to use `+`
    /// * [`ListStyleType::Star`] to use `*`
    #[serde(with = "ListStyleType")]
    #[tsify(type = "ListStyleType")]
    pub list_style: ComrakListStyleType,
    /// Include source position attributes in HTML and XML output.
    ///
    /// Sourcepos information is reliable for core block items excluding
    /// lists and list items, all inlines, and most extensions.
    /// The description lists extension still has issues; see
    /// <https://github.com/kivikakk/comrak/blob/3bb6d4ce/src/tests/description_lists.rs#L60-L125>.
    pub sourcepos: bool,
    /// Wrap escaped characters in a `<span>` to allow any
    /// post-processing to recognize them.
    ///
    /// Enabling this option will cause the `escaped_char_spans` parse option to
    /// be enabled.
    pub escaped_char_spans: bool,
    /// Ignore empty links in input.
    pub ignore_empty_links: bool,
    /// Enables GFM quirks in HTML output which break CommonMark compatibility.
    pub gfm_quirks: bool,
    /// Prefer fenced code blocks when outputting CommonMark.
    pub prefer_fenced: bool,
    /// Render the image as a figure element with the title as its caption.
    pub figure_with_caption: bool,
    /// Add classes to the output of the tasklist extension. This allows tasklists to be styled.
    pub tasklist_classes: bool,
    #[serde(with = "AlertStyleType")]
    #[tsify(type = "AlertStyleType")]
    /// How to render alert blocks. Options are:
    ///
    /// * [`AlertStyleType::Specific`] to use `div`s with `markdown-` prefixed classes (default)
    /// * [`AlertStyleType::Semantic`] to use `aside`s with an `admonition` class
    pub alert_style: ComrakAlertStyleType,
    /// Render ordered list with a minimum marker width.
    /// Having a width lower than 3 doesn't do anything.
    pub ol_width: usize,
    /// Minimise escapes used in CommonMark output (`-t commonmark`) by removing
    /// each individually and seeing if the resulting document roundtrips.
    /// Brute-force and expensive, but produces nicer output. Note that the
    /// result may not in fact be minimal.
    pub experimental_minimize_commonmark: bool,
    /// Suppress pretty-printing newlines between block-level HTML elements.
    ///
    /// Normally comrak puts a `\n` after closing tags like `</p>`, `</li>`,
    /// etc. With this option on, those newlines are omitted.
    pub compact_html: bool,
}

#[derive(Default, Tsify, Deserialize)]
#[serde(default)]
#[serde(remote = "ComrakPlugins")]
pub struct Plugins<'p> {
    #[serde(with = "RenderPlugins")]
    #[tsify(type = "RenderPlugins")]
    pub render: ComrakRenderPlugins<'p>,
}

#[derive(Default, Tsify, Deserialize)]
#[serde(default)]
#[serde(remote = "ComrakRenderPlugins")]
#[serde(rename_all = "camelCase")]
pub struct RenderPlugins<'p> {
    #[serde(with = "crate::adapters::codefence_renderer_adapter")]
    #[tsify(type = "Map<String, CodefenceRendererAdapter>")]
    pub codefence_renderers: HashMap<String, &'p dyn CodefenceRendererAdapter>,
    #[serde(with = "crate::adapters::syntax_highlighter_adapter")]
    #[tsify(type = "SyntaxHighlighterAdapter | null | undefined")]
    pub codefence_syntax_highlighter: Option<&'p dyn SyntaxHighlighterAdapter>,
    #[serde(with = "crate::adapters::heading_adapter")]
    #[tsify(type = "HeadingAdapter | null | undefined")]
    pub heading_adapter: Option<&'p dyn ComrakHeadingAdapter>,
}
