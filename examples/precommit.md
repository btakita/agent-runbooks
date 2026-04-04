# Precommit

Steps to run before committing code changes.

## Checklist

1. **Run tests and linting**
   - Rust: `make check` (clippy + test)
   - JS/TS: `bun test` or `npm test`
   - Python: `pytest`

2. **Add tests for new behavior**
   - New functions/methods need unit tests
   - Bug fixes need regression tests
   - Edge cases identified during implementation need coverage

3. **Audit instruction files** (if changed)
   - `module-harness audit` (validates CLAUDE.md, AGENTS.md, SPEC.md consistency)
   - Verify CLAUDE.md reflects any architectural changes

4. **Review diff**
   - No secrets, credentials, or API keys in the diff
   - No debug/temporary code left behind
   - No unrelated changes bundled in
