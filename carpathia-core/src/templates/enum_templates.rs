#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InitTemplate {
    RustLib,
    Svg,
    Markdown,
    Mermaid,
    None,
}
