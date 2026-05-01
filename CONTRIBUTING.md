# Contributing

Thanks for contributing to MD Star.

## Development Setup

```bash
git clone <repo-url>
cd markdown
cargo test
```

## Workflow

1. Create a feature branch.
2. Make focused changes with tests.
3. Run required checks locally:
   - `cargo fmt --all`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test --workspace --all-features`
4. Open a pull request with:
   - summary of behavior changes,
   - test evidence,
   - risks/known limitations.

## Commit Guidance

- Keep commits small and reviewable.
- Prefer one logical change per commit.
- Use clear, imperative commit messages.

## Code Expectations

- Keep parser/render behavior deterministic.
- Avoid breaking cross-surface rendering parity.
- Add fixtures for every user-visible syntax change.
- Preserve non-panicking behavior in user-facing paths.

## Pull Request Checklist

- [ ] tests added or updated
- [ ] docs updated if behavior changed
- [ ] CI checks pass
- [ ] no unrelated refactors

