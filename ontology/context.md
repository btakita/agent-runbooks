# Context

Any on-demand document that a skill loads when relevant. Context is the generic term -- runbooks, references, and guardrails are specific types of context.

## Types

- **Procedural** ([term:Runbook]): Step-by-step procedures
- **Factual** (Reference): Configuration details, API specs, environment info
- **Behavioral** ([term:Guardrail]): Rules that prevent mistakes

## Properties

- **On-demand:** Loaded only when the agent determines relevance
- **Zero always-loaded cost:** Unlike guardrails in auto memory, pure contexts have no trigger cost
- **Scoped:** Belongs to a specific [term:Skill] or domain
