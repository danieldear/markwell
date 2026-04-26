use markdown_core::{
    Block, Document, DocumentMetadata, HeadingMetadata, Inline, MarkdownError, MarkdownRsParser,
    ParseOutput, ParserAdapter, parse_markdown, parse_markdown_with_diagnostics,
    parse_with_adapter,
};

#[test]
fn parses_heading_list_and_code_fence() {
    let input = r#"# Title

- first item
- second item

```rust
fn demo() {}
```
"#;

    let doc = parse_markdown(input).expect("expected markdown to parse");

    assert!(matches!(
        doc.blocks.first(),
        Some(Block::Heading { level: 1, .. })
    ));
    assert!(
        doc.blocks
            .iter()
            .any(|block| matches!(block, Block::List { ordered: false, .. }))
    );
    assert!(doc.blocks.iter().any(|block| matches!(
        block,
        Block::CodeFence { language, code, .. }
            if language.as_deref() == Some("rust") && code.contains("fn demo")
    )));
}

#[test]
fn heading_children_contain_text_inline() {
    let doc = parse_markdown("# Hello **world**").expect("parse ok");
    let Some(Block::Heading { level: 1, children }) = doc.blocks.first() else {
        panic!("expected H1");
    };
    assert!(
        children
            .iter()
            .any(|i| matches!(i, Inline::Text(t) if t == "Hello "))
    );
    assert!(children.iter().any(|i| matches!(i, Inline::Strong(_))));
}

#[test]
fn paragraph_preserves_bold_and_italic() {
    let doc = parse_markdown("This is **bold** and *italic*.").expect("parse ok");
    let Some(Block::Paragraph { children }) = doc.blocks.first() else {
        panic!("expected paragraph");
    };
    assert!(children.iter().any(|i| matches!(i, Inline::Strong(_))));
    assert!(children.iter().any(|i| matches!(i, Inline::Emphasis(_))));
}

#[test]
fn parses_blockquote() {
    let doc = parse_markdown("> quoted").expect("parse ok");
    assert!(doc.blocks.iter().any(|b| matches!(b, Block::BlockQuote(_))));
}

#[test]
fn list_items_have_checked_field_for_tasks() {
    let input = "- [x] done\n- [ ] todo\n";
    let doc = parse_markdown(input).expect("parse ok");
    let Some(Block::List { items, .. }) = doc.blocks.first() else {
        panic!("expected list");
    };
    assert_eq!(items[0].checked, Some(true));
    assert_eq!(items[1].checked, Some(false));
}

#[test]
fn extracts_heading_metadata() {
    let input = "# One\n\n## Two\n\nBody";
    let doc = parse_markdown(input).expect("expected parse success");

    assert_eq!(doc.metadata.headings.len(), 2);
    assert_eq!(doc.metadata.headings[0].level, 1);
    assert_eq!(doc.metadata.headings[0].text, "One");
    assert_eq!(doc.metadata.headings[1].level, 2);
    assert_eq!(doc.metadata.headings[1].text, "Two");
}

#[test]
fn produces_diagnostics_for_tabs_and_trailing_spaces() {
    let input = "\t# Title\nline with trailing space \n";
    let output = parse_markdown_with_diagnostics(input).expect("expected parse success");

    assert!(
        output
            .diagnostics
            .iter()
            .any(|d| d.code == "MD001" && d.line == Some(1))
    );
    assert!(
        output
            .diagnostics
            .iter()
            .any(|d| d.code == "MD002" && d.line == Some(2))
    );
}

#[test]
fn supports_parser_adapter_abstraction() {
    struct FakeParser;

    impl ParserAdapter for FakeParser {
        fn parse(&self, _input: &str) -> Result<ParseOutput, MarkdownError> {
            Ok(ParseOutput {
                document: Document {
                    blocks: vec![Block::Heading {
                        level: 1,
                        children: vec![Inline::Text("FromAdapter".to_string())],
                    }],
                    metadata: DocumentMetadata {
                        headings: vec![HeadingMetadata {
                            level: 1,
                            text: "FromAdapter".to_string(),
                        }],
                    },
                },
                diagnostics: Vec::new(),
            })
        }
    }

    let output = parse_with_adapter(&FakeParser, "# ignored").expect("expected adapter to run");
    assert!(matches!(
        output.document.blocks.first(),
        Some(Block::Heading { level: 1, .. })
    ));
}

#[test]
fn parser_adapter_is_publicly_usable() {
    let parser = MarkdownRsParser;
    let output = parse_with_adapter(&parser, "# Title").expect("expected parse success");
    assert_eq!(output.document.metadata.headings.len(), 1);
}
