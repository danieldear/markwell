use markdown_core::parse_markdown;
use markdown_render_terminal::{
    RenderOptions, preprocess_mermaid_blocks, render_markdown, render_terminal,
};

#[test]
fn renders_basic_blocks_for_terminal() {
    let source = include_str!("../../../tests/fixtures/sample.md");
    let doc = parse_markdown(source).expect("expected parse success");

    let output = render_terminal(
        &doc,
        RenderOptions {
            color: false,
            ..RenderOptions::default()
        },
    );

    assert!(output.contains("Sample Document"));
    assert!(output.contains("first item"));
    assert!(output.contains("fn demo()"));
}

#[test]
fn renders_mermaid_blocks_in_preprocessing() {
    let input = r#"# Diagram

```mermaid
graph LR
    A[Build] --> B[Test]
```
"#;

    let output = preprocess_mermaid_blocks(input, RenderOptions::default());
    assert!(output.contains("```mermaid"));
    assert!(output.contains("Build"));
    assert!(output.contains("Test"));
}

#[test]
fn renders_markdown_tables_with_terminal_borders() {
    let input = r#"| Name | Value |
| ---- | ----- |
| A    | 1     |
"#;

    let output = render_markdown(
        input,
        RenderOptions {
            color: false,
            ..RenderOptions::default()
        },
    );

    assert!(output.contains("Name"));
    assert!(output.contains("Value"));
    assert!(output.contains("A"));
}

#[test]
fn can_render_mermaid_in_ascii_mode() {
    let input = r#"```mermaid
graph LR
    A[Build] --> B[Deploy]
```
"#;

    let preprocessed = preprocess_mermaid_blocks(
        input,
        RenderOptions {
            ascii_mermaid: true,
            ..RenderOptions::default()
        },
    );

    assert!(preprocessed.is_ascii());
    assert!(preprocessed.contains("Build"));
    assert!(preprocessed.contains("Deploy"));
}
