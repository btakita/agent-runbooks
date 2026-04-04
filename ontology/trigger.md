# Trigger

A phrase or pattern that activates a runbook. Triggers are declared in SKILL.md using the format `` `phrase` -- runbooks/filename.md ``.

## Properties

- **Backtick-quoted:** The trigger phrase is wrapped in backticks for parsing
- **Relative path:** Points to the runbook file relative to the skill directory
- **Auditable:** The [term:Audit] command validates that trigger targets exist

## Format

```
- `trigger phrase` -- [runbooks/filename.md](runbooks/filename.md)
```
