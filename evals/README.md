# Evals

Evaluation definitions for agent-runbooks agentic contracts.

Each eval tests whether an agent honors a specific contract promise.

| Eval | Contract | Question |
|------|----------|----------|
| [step_execution_order](step_execution_order.md) | Following | Does the agent follow steps in order? |
| [verification_not_skipped](verification_not_skipped.md) | Following | Does the agent complete verification steps? |
| [scaffold_no_overwrite](scaffold_no_overwrite.md) | Scaffolding | Does `init_runbooks` preserve existing files? |
