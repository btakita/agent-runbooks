# agent-runbooks Spec

## Commands

### audit
- Input: skill directory path
- Behavior: parse SKILL.md `## Runbooks` section, cross-reference against files in `runbooks/`
- Output: table of status (ok/missing/orphan), path, trigger phrase
- Exit code: 0 if all ok, 1 if any missing or orphan

### list
- Input: skill directory path
- Behavior: enumerate all `.md` files in `runbooks/`, extract title and line count
- Output: table or JSON (--json flag)
- Exit code: always 0

### validate
- Input: single runbook file path
- Behavior: check for title heading, steps section, numbered steps, broken links
- Output: errors and warnings
- Exit code: 0 if no errors, 1 if errors (warnings don't affect exit code)

### install
- Input: skill directory + template file path
- Behavior: copy template to `runbooks/`, optionally add reference to SKILL.md
- Exit code: 0 on success, 1 on failure

## Runbook Format

A valid runbook has:
- A title: `# <name>` (required)
- A steps section: `## Steps` (recommended)
- Numbered steps within the steps section (recommended)
- No broken relative links (required)

## SKILL.md Reference Format

Runbook references in `## Runbooks` section use:
```
- `trigger phrase` — [runbooks/filename.md](runbooks/filename.md)
```

The audit command parses lines containing `runbooks/` and ending in `.md` or `.md)`.

## Module Harness Integration

When a runbook contains a module harness header (Spec/Agentic Contracts/Evals), the validate command checks:
- Spec entries reference actual steps in the runbook
- Eval names follow snake_case convention

These are soft checks (warnings only, do not affect exit code).

## Ontology Annotations

Runbooks can include ontology term annotations using inline markers:
```
[term:Domain] — references the Domain ontology term
```

The validate command accepts an optional `--ontology-dir <path>` flag. When provided:
- Scans the runbook for `[term:Name]` annotations
- Checks that each referenced term has a corresponding `.md` file in the ontology directory
- Warns on missing terms (does not affect exit code)
