use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::HeexNode")]
pub enum HeexNode {
    Directive,
    Comment,
    MultilineComment,
    Expression,
    Tag(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeHeexBlock")]
pub struct NodeHeexBlock {
    pub literal: String,
    #[serde(with = "HeexNode")]
    pub node: comrak::nodes::HeexNode,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeShortCode")]
pub struct NodeShortCode {
    pub code: String,

    pub emoji: String,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeMultilineBlockQuote")]
pub struct NodeMultilineBlockQuote {
    pub fence_length: usize,
    pub fence_offset: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeMath")]
pub struct NodeMath {
    pub dollar_math: bool,
    pub display_math: bool,
    pub literal: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(remote = "comrak::nodes::AlertType")]
pub enum AlertType {
    #[default]
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeWikiLink")]
pub struct NodeWikiLink {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeAlert")]
pub struct NodeAlert {
    #[serde(with = "AlertType")]
    pub alert_type: comrak::nodes::AlertType,
    pub title: Option<String>,
    pub multiline: bool,
    pub fence_length: usize,
    pub fence_offset: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeBlockDirective")]
pub struct NodeBlockDirective {
    pub fence_length: usize,
    pub fence_offset: usize,
    pub info: String,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeFootnoteReference")]
pub struct NodeFootnoteReference {
    pub name: String,
    pub texts: Vec<(String, usize)>,
    pub ref_num: u32,
    pub ix: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::LineColumn")]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::Sourcepos")]
pub struct Sourcepos {
    #[serde(with = "LineColumn")]
    pub start: comrak::nodes::LineColumn,
    #[serde(with = "LineColumn")]
    pub end: comrak::nodes::LineColumn,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeTaskItem")]
pub struct NodeTaskItem {
    pub symbol: Option<char>,
    #[serde(with = "Sourcepos")]
    pub symbol_sourcepos: comrak::nodes::Sourcepos,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeFootnoteDefinition")]
pub struct NodeFootnoteDefinition {
    pub name: String,
    pub total_references: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeCode")]
pub struct NodeCode {
    pub num_backticks: usize,
    pub literal: String,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::Attributes")]
pub struct Attributes {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub pairs: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::TableAlignment")]
pub enum TableAlignment {
    None,
    Left,
    Center,
    Right,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeTable")]
pub struct NodeTable {
    #[serde(with = "table_alignments")]
    pub alignments: Vec<comrak::nodes::TableAlignment>,
    pub num_columns: usize,
    pub num_rows: usize,
    pub num_nonempty_cells: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeLink")]
pub struct NodeLink {
    pub url: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeHeading")]
pub struct NodeHeading {
    pub level: u8,
    pub setext: bool,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(remote = "comrak::nodes::ListType")]
pub enum ListType {
    #[default]
    Bullet,
    Ordered,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(remote = "comrak::nodes::ListDelimType")]
pub enum ListDelimType {
    #[default]
    Period,
    Paren,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeList")]
pub struct NodeList {
    #[serde(with = "ListType")]
    pub list_type: comrak::nodes::ListType,
    pub marker_offset: usize,
    pub padding: usize,
    pub start: usize,
    #[serde(with = "ListDelimType")]
    pub delimiter: comrak::nodes::ListDelimType,
    pub bullet_char: u8,
    pub tight: bool,
    pub is_task_list: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeDescriptionItem")]
pub struct NodeDescriptionItem {
    pub marker_offset: usize,
    pub padding: usize,
    pub tight: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeCodeBlock")]
pub struct NodeCodeBlock {
    pub fenced: bool,
    pub fence_char: u8,
    pub fence_length: usize,
    pub fence_offset: usize,
    pub info: String,
    pub literal: String,
    pub closed: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeHtmlBlock")]
pub struct NodeHtmlBlock {
    pub block_type: u8,
    pub literal: String,
}

mod table_alignments {
    use super::TableAlignment;
    use serde::{Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Vec<comrak::nodes::TableAlignment>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeqVisitor;
        impl<'de> serde::de::Visitor<'de> for SeqVisitor {
            type Value = Vec<comrak::nodes::TableAlignment>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of table alignments")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(TableAlignmentSeed)? {
                    values.push(value);
                }
                Ok(values)
            }
        }

        struct TableAlignmentSeed;
        impl<'de> serde::de::DeserializeSeed<'de> for TableAlignmentSeed {
            type Value = comrak::nodes::TableAlignment;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                TableAlignment::deserialize(deserializer)
            }
        }

        deserializer.deserialize_seq(SeqVisitor)
    }

    pub fn serialize<S>(
        value: &Vec<comrak::nodes::TableAlignment>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for item in value {
            seq.serialize_element(&TableAlignmentRef(item))?;
        }
        seq.end()
    }

    struct TableAlignmentRef<'a>(&'a comrak::nodes::TableAlignment);

    impl Serialize for TableAlignmentRef<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            TableAlignment::serialize(self.0, serializer)
        }
    }
}

mod boxed_node_code_block {
    use super::NodeCodeBlock;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Box<comrak::nodes::NodeCodeBlock>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeCodeBlock::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeCodeBlock>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeCodeBlock::serialize(value.as_ref(), serializer)
    }
}

mod boxed_node_heex_block {
    use super::NodeHeexBlock;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Box<comrak::nodes::NodeHeexBlock>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeHeexBlock::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeHeexBlock>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeHeexBlock::serialize(value.as_ref(), serializer)
    }
}

mod boxed_node_table {
    use super::NodeTable;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<comrak::nodes::NodeTable>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeTable::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeTable>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeTable::serialize(value.as_ref(), serializer)
    }
}

mod boxed_node_link {
    use super::NodeLink;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<comrak::nodes::NodeLink>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeLink::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeLink>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeLink::serialize(value.as_ref(), serializer)
    }
}

mod boxed_node_footnote_reference {
    use super::NodeFootnoteReference;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Box<comrak::nodes::NodeFootnoteReference>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeFootnoteReference::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeFootnoteReference>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeFootnoteReference::serialize(value.as_ref(), serializer)
    }
}

mod boxed_node_short_code {
    use super::NodeShortCode;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Box<comrak::nodes::NodeShortCode>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeShortCode::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeShortCode>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeShortCode::serialize(value.as_ref(), serializer)
    }
}

mod boxed_node_alert {
    use super::NodeAlert;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<comrak::nodes::NodeAlert>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeAlert::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeAlert>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeAlert::serialize(value.as_ref(), serializer)
    }
}

mod boxed_node_block_directive {
    use super::NodeBlockDirective;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Box<comrak::nodes::NodeBlockDirective>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NodeBlockDirective::deserialize(deserializer).map(Box::new)
    }

    pub fn serialize<S>(
        value: &Box<comrak::nodes::NodeBlockDirective>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeBlockDirective::serialize(value.as_ref(), serializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "comrak::nodes::NodeValue")]
pub enum NodeValue {
    Document,
    FrontMatter(String),
    BlockQuote,
    #[serde(with = "NodeList")]
    List(comrak::nodes::NodeList),
    #[serde(with = "NodeList")]
    Item(comrak::nodes::NodeList),
    DescriptionList,
    #[serde(with = "NodeDescriptionItem")]
    DescriptionItem(comrak::nodes::NodeDescriptionItem),
    DescriptionTerm,
    DescriptionDetails,
    #[serde(with = "boxed_node_code_block")]
    CodeBlock(Box<comrak::nodes::NodeCodeBlock>),
    #[serde(with = "NodeHtmlBlock")]
    HtmlBlock(comrak::nodes::NodeHtmlBlock),
    #[serde(with = "boxed_node_heex_block")]
    HeexBlock(Box<comrak::nodes::NodeHeexBlock>),
    Paragraph,
    #[serde(with = "NodeHeading")]
    Heading(comrak::nodes::NodeHeading),
    ThematicBreak,
    #[serde(with = "NodeFootnoteDefinition")]
    FootnoteDefinition(comrak::nodes::NodeFootnoteDefinition),
    #[serde(with = "boxed_node_table")]
    Table(Box<comrak::nodes::NodeTable>),
    TableRow(bool),
    TableCell,
    Text(Cow<'static, str>),
    #[serde(with = "NodeTaskItem")]
    TaskItem(comrak::nodes::NodeTaskItem),
    SoftBreak,
    LineBreak,
    #[serde(with = "NodeCode")]
    Code(comrak::nodes::NodeCode),
    HtmlInline(String),
    HeexInline(String),
    Raw(String),
    Emph,
    Strong,
    Strikethrough,
    Highlight,
    Insert,
    Superscript,
    #[serde(with = "boxed_node_link")]
    Link(Box<comrak::nodes::NodeLink>),
    #[serde(with = "boxed_node_link")]
    Image(Box<comrak::nodes::NodeLink>),
    #[serde(with = "boxed_node_footnote_reference")]
    FootnoteReference(Box<comrak::nodes::NodeFootnoteReference>),
    #[serde(with = "boxed_node_short_code")]
    ShortCode(Box<comrak::nodes::NodeShortCode>),
    #[serde(with = "NodeMath")]
    Math(comrak::nodes::NodeMath),
    #[serde(with = "NodeMultilineBlockQuote")]
    MultilineBlockQuote(comrak::nodes::NodeMultilineBlockQuote),
    Escaped,
    #[serde(with = "NodeWikiLink")]
    WikiLink(comrak::nodes::NodeWikiLink),
    Underline,
    Subscript,
    SpoileredText,
    EscapedTag(&'static str),
    #[serde(with = "boxed_node_alert")]
    Alert(Box<comrak::nodes::NodeAlert>),
    Subtext,
    #[serde(with = "boxed_node_block_directive")]
    BlockDirective(Box<comrak::nodes::NodeBlockDirective>),
}

// #[derive(Serialize, Deserialize, Default)]
// pub struct Ast {
//     pub value: NodeValue,
//     pub sourcepos: Sourcepos,
//     pub attrs: Option<Box<Attributes>>,
//     pub content: String,
//     pub open: bool,
//     pub last_line_blank: bool,
//     pub table_visited: bool,
//     pub line_offsets: Vec<usize>,
// }
