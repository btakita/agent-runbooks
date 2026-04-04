# Guardrail

A behavioral rule that prevents mistakes before they happen. Guardrails are always-loaded (via auto memory index) because they need to fire proactively, not reactively.

## Properties

- **Proactive:** Loaded at conversation start, not on-demand
- **Low cost:** ~1 line in MEMORY.md index per guardrail
- **Preventive:** "Don't do X" rather than "How to do X"

## Relationships

- Guardrails are a type of [term:Context] (behavioral context)
- Distinguished from [term:Runbook] by loading strategy (always vs on-demand)
- May migrate to hook-based injection when tooling matures
