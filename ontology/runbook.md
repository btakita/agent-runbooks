# Runbook

A procedural document containing ordered steps to accomplish a specific operation. Runbooks are on-demand context -- loaded only when executing the procedure they describe.

## Properties

- **Deterministic trigger:** Each runbook has a trigger phrase that activates it
- **Sequential steps:** Numbered steps executed in order
- **Co-located:** Lives alongside the skill that owns it (`runbooks/` directory)
- **On-demand:** Not loaded into agent context until needed

## Relationships

- A runbook is a type of [term:Context] (procedural context)
- Runbooks are referenced from a [term:Skill] via trigger phrases
- Validated by [term:Audit] for drift between references and files
