use std::fs;
use std::path::{Path, PathBuf};

use markdown_core::{Block, ParseOutput, inlines_to_plain_text, parse_markdown_with_diagnostics};

#[test]
fn fixture_harness_matches_expected_snapshots() {
    let fixture_dir = workspace_path("tests/fixtures/core");
    let expected_dir = workspace_path("tests/fixtures/expected-core");

    let mut fixtures = fs::read_dir(&fixture_dir)
        .expect("fixture directory should exist")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "expected at least one core fixture in {}",
        fixture_dir.display()
    );

    for fixture_path in fixtures {
        let stem = fixture_path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("fixture file should have valid stem");
        let expected_path = expected_dir.join(format!("{stem}.txt"));
        let source = fs::read_to_string(&fixture_path).expect("fixture should be readable");
        let parsed =
            parse_markdown_with_diagnostics(&source).expect("fixture parse should succeed");
        let actual = semantic_snapshot(&parsed);
        let expected = fs::read_to_string(&expected_path)
            .expect("expected snapshot should exist and readable");

        assert_eq!(
            expected, actual,
            "semantic snapshot mismatch for fixture '{}'",
            stem
        );
    }
}

fn workspace_path(relative: &str) -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("../../").join(relative)
}

fn semantic_snapshot(parsed: &ParseOutput) -> String {
    let mut out = String::new();

    out.push_str("HEADINGS\n");
    for heading in &parsed.document.metadata.headings {
        out.push_str(&format!("H{}: {}\n", heading.level, heading.text));
    }

    out.push_str("BLOCKS\n");
    for block in &parsed.document.blocks {
        snapshot_block(block, &mut out, 0);
    }

    out.push_str("DIAGNOSTICS\n");
    for diagnostic in &parsed.diagnostics {
        out.push_str(&format!(
            "{:?}|{}|{}|line={:?}|col={:?}\n",
            diagnostic.severity,
            diagnostic.code,
            diagnostic.message,
            diagnostic.line,
            diagnostic.column
        ));
    }

    out
}

fn snapshot_block(block: &Block, out: &mut String, depth: usize) {
    let pad = "  ".repeat(depth);
    match block {
        Block::Heading { level, children } => {
            out.push_str(&format!(
                "{pad}Heading(level={level}, text={})\n",
                inlines_to_plain_text(children)
            ));
        }
        Block::Paragraph { children } => {
            out.push_str(&format!(
                "{pad}Paragraph(text={})\n",
                inlines_to_plain_text(children)
            ));
        }
        Block::BlockQuote(blocks) => {
            out.push_str(&format!("{pad}BlockQuote\n"));
            for b in blocks {
                snapshot_block(b, out, depth + 1);
            }
        }
        Block::List { ordered, items, .. } => {
            out.push_str(&format!(
                "{pad}List(ordered={ordered}, items={})\n",
                items.len()
            ));
        }
        Block::CodeFence { language, code, .. } => {
            out.push_str(&format!(
                "{pad}CodeFence(language={:?}, code={:?})\n",
                language, code
            ));
        }
        Block::Table { headers, rows } => {
            let header_texts: Vec<String> =
                headers.iter().map(|c| inlines_to_plain_text(c)).collect();
            let row_texts: Vec<Vec<String>> = rows
                .iter()
                .map(|row| row.iter().map(|c| inlines_to_plain_text(c)).collect())
                .collect();
            out.push_str(&format!(
                "{pad}Table(headers={:?}, rows={:?})\n",
                header_texts, row_texts
            ));
        }
        Block::ThematicBreak => out.push_str(&format!("{pad}ThematicBreak\n")),
        Block::Html(html) => out.push_str(&format!("{pad}Html({:?})\n", html)),
        Block::Math(math) => out.push_str(&format!("{pad}Math({:?})\n", math)),
        Block::Frontmatter(fm) => out.push_str(&format!("{pad}Frontmatter({:?})\n", fm)),
    }
}
