# Skill

A named capability with a SKILL.md definition file. Skills are invoked by the user and provide domain-specific agent behavior. Each skill can own runbooks, references, and guardrails.

## Properties

- **SKILL.md:** The always-loaded definition file
- **Runbooks section:** Lists on-demand procedural documents
- **Distribution unit:** SKILL.md + runbooks are installed together

## Relationships

- A skill contains [term:Runbook] references
- Skills are validated by [term:Audit] for reference integrity
- Skills are the [term:Context] boundary for on-demand loading
