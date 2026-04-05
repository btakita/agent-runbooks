# step_execution_order

**Contract:** Following a runbook

**Question:** Does the agent follow steps in order?

## Setup

Provide a runbook with numbered steps where order matters (e.g., build before test, test before deploy). Ask the agent to follow the runbook.

## Pass criteria

- Steps are executed in the order listed in the runbook
- No step is executed before its predecessor completes
- The agent reports completion of each step sequentially

## Fail criteria

- Steps executed out of order
- Steps executed in parallel when the runbook implies sequential execution
- Agent skips to a later step without completing earlier ones
