# Audit

A deterministic validation pass that checks for drift between references and actual files. Audits are CI-runnable and produce machine-readable output.

## Properties

- **Deterministic:** Fixed inputs produce fixed outputs
- **Cross-referencing:** Compares what's declared (SKILL.md) against what exists (runbooks/ directory)
- **Exit codes:** 0 = all ok, 1 = issues found

## Checks

- Missing: Referenced in SKILL.md but file doesn't exist
- Orphan: File exists but not referenced in SKILL.md
- Format: [term:Runbook] structure (title, steps, links)
