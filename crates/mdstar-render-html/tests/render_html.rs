use mdstar_core::parse_markdown;
use mdstar_render_html::render_html;

#[test]
fn renders_semantic_html() {
    let source = include_str!("../../../tests/fixtures/sample.md");
    let doc = parse_markdown(source).expect("expected parse success");

    let html = render_html(&doc);

    assert!(html.contains("<h1>Sample Document</h1>"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<code class=\"language-rust\">"));
}
