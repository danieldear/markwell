use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn views_markdown_file() {
    let fixture = workspace_fixture("tests/fixtures/sample.md");

    Command::cargo_bin("md")
        .expect("md binary should exist")
        .arg("view")
        .arg(&fixture)
        .arg("--no-pager")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sample Document"));
}

#[test]
fn views_markdown_file_with_direct_invocation() {
    let fixture = workspace_fixture("tests/fixtures/sample.md");

    Command::cargo_bin("md")
        .expect("md binary should exist")
        .arg(&fixture)
        .arg("--no-pager")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sample Document"));
}

#[test]
fn supports_explicit_pager_command() {
    let fixture = workspace_fixture("tests/fixtures/sample.md");

    Command::cargo_bin("md")
        .expect("md binary should exist")
        .arg("view")
        .arg(&fixture)
        .arg("--pager")
        .arg("cat")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sample Document"));
}

#[test]
fn renders_mermaid_from_markdown_file() {
    let fixture = workspace_fixture("tests/fixtures/big.md");

    Command::cargo_bin("md")
        .expect("md binary should exist")
        .arg(&fixture)
        .arg("--no-pager")
        .arg("--plain")
        .assert()
        .success()
        .stdout(predicate::str::contains("Write"))
        .stdout(predicate::str::contains("Preview"))
        .stdout(predicate::str::contains("Commit"))
        .stdout(predicate::str::contains("parser"));
}

#[test]
fn returns_error_for_missing_file() {
    let missing_file = workspace_fixture("tests/fixtures/missing.md");

    Command::cargo_bin("md")
        .expect("md binary should exist")
        .arg("view")
        .arg(&missing_file)
        .arg("--no-pager")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to read markdown file"));
}

#[test]
fn matches_plain_output_snapshot() {
    let fixture = workspace_fixture("tests/fixtures/sample.md");
    let expected_snapshot =
        fs::read_to_string(workspace_fixture("tests/fixtures/expected-cli/basic.txt"))
            .expect("expected CLI snapshot should exist");

    let assert = Command::cargo_bin("md")
        .expect("md binary should exist")
        .arg("view")
        .arg(&fixture)
        .arg("--no-color")
        .arg("--no-pager")
        .assert()
        .success();

    let output = String::from_utf8(assert.get_output().stdout.clone())
        .expect("output should be valid utf-8");
    assert_eq!(expected_snapshot, output);
}

fn workspace_fixture(relative: &str) -> String {
    format!("{}/../../{relative}", env!("CARGO_MANIFEST_DIR"))
}
