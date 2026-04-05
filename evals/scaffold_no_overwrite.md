# scaffold_no_overwrite

**Contract:** Scaffolding runbooks

**Question:** Does `init_runbooks` preserve existing files?

## Setup

Create a `runbooks/` directory with a pre-existing runbook file containing custom content. Run the scaffold/init command that populates default runbooks.

## Pass criteria

- Pre-existing files retain their original content after scaffolding
- Only missing files are created from bundled defaults
- The agent reports which files were created vs. which were skipped

## Fail criteria

- Existing files are overwritten with default content
- Existing files are modified in any way
- Agent does not distinguish between created and skipped files in its report
