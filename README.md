# Agent Runbooks

A convention for externalizing step-by-step procedures from AI agent instruction files (CLAUDE.md, AGENTS.md, .cursorrules, etc.) into on-demand runbook files.

## The Problem

AI agent instruction files serve two purposes in tension:

1. **Policy** — conventions, architecture decisions, rules that apply to every interaction
2. **Procedure** — step-by-step instructions that apply to specific workflows

Policy is always-on context and belongs in the instruction file. Procedure is on-demand context — it should be loaded when needed and absent when not.

Inlining procedures into your instruction file means paying the full token cost on every interaction, even when the procedure is irrelevant.

## The Convention

Place runbooks in a `runbooks/` directory under whichever agent config directory you use:

```
your-project/
├── .agent/runbooks/            # or .agents/runbooks/, .ai/runbooks/
│   ├── precommit.md
│   ├── prerelease.md
│   ├── deploy.md
│   └── incident-response.md
├── CLAUDE.md                  # (or AGENTS.md, .cursorrules, etc.)
└── ...
```

The parent directory name is an emerging convention — `.agent/`, `.agents/`, `.ai/`, and tool-specific directories (`.claude/`, `.cursor/`) all work. What matters is the pattern: externalize procedures into on-demand files.

Each runbook is a standalone markdown file with a clear title, description, and procedural steps. Your instruction file references them with a single line:

```markdown
Before committing, follow `.agent/runbooks/precommit.md`.
```

25 lines of inline procedure replaced by 1 line of reference. The agent reads the runbook on demand when it actually needs the procedure.

### Emerging directory conventions

| Convention | Examples | Status |
|-----------|----------|--------|
| `.ai/` | [AgentInfra](https://github.com/JayCheng113/AgentInfra), [dot-ai](https://github.com/luisrudge/dot-ai) (archived), [agnostic-ai](https://github.com/betagouv/agnostic-ai) | Early-stage |
| `.agents/` | [ACS spec](https://acs.jackby03.com/), [.agents Protocol](https://dotagentsprotocol.com/) | Multiple competing specs |
| `.agent/` | [dotagent](https://github.com/johnlindquist/dotagent) (122 stars) | Most-starred proposal |
| Tool-specific | `.claude/`, `.cursor/rules/`, `.windsurf/rules/` | Established but vendor-locked |

Pick whichever aligns with your existing setup. The runbooks pattern works with any of them.

## Runbook Format

Runbooks are plain markdown. No special syntax required. Recommended structure:

```markdown
# Procedure Name

One-line description of when this procedure applies.

## Steps

1. First step
   - Sub-step detail
   - Commands: `make check`

2. Second step
   ...
```

## Runbooks vs Rules

AI coding tools use "rules" (`.cursor/rules/`, `.windsurf/rules/`, `.rules`) for **declarative policy** — conventions, coding standards, and architectural guidelines that shape agent behavior. Rules are typically always-loaded or conditionally attached.

Runbooks are different: they contain **imperative procedures** — step-by-step instructions for specific tasks. The distinction matters because it tells the agent what to expect:

- **Rules** = "use snake_case for Python functions" (policy, always relevant)
- **Runbooks** = "step 1: run `make check`, step 2: verify output, step 3: stage files..." (procedure, relevant only during that task)

Loading mode (eager/lazy) is orthogonal — both rules and runbooks can be loaded on demand. The content type is what differs.

## Design Principles

1. **One procedure per file.** Each runbook covers exactly one workflow.
2. **Runbooks are imperative, instruction files are declarative.** The instruction file says *what* (conventions, rules). Runbooks say *how* (step-by-step procedures).
3. **Reference, don't duplicate.** One-line pointer in the instruction file; full procedure in the runbook.
4. **Self-contained.** A runbook should make sense without reading the instruction file first.
5. **Version-controlled.** Runbooks are code. They belong in the repo.

## Cross-Harness Compatibility

This convention works with any AI coding assistant that can read project files:

| Tool | Instruction File | How It Loads Runbooks |
|------|-----------------|----------------------|
| Claude Code | CLAUDE.md / AGENTS.md | Agent reads referenced files on demand |
| Cursor | `.cursor/rules/*.mdc` | Agent Requested mode or file reference |
| GitHub Copilot | `.github/copilot-instructions.md` | Scoped instructions with `applyTo` globs |
| Windsurf | `.windsurf/rules/*.md` | Agent reads referenced files |
| Gemini CLI | GEMINI.md | `@file.md` import syntax |
| Aider | CONVENTIONS.md | `--read` flag or `.aider.conf.yml` |
| Codex (OpenAI) | AGENTS.md | Nested per-directory + file reads |

## Examples

See the [`examples/`](examples/) directory for real-world runbooks:

- [`precommit.md`](examples/precommit.md) — Pre-commit checklist (test, lint, audit, review)
- [`prerelease.md`](examples/prerelease.md) — Multi-target release procedure

## Related Work

- [AGENTS.md](https://agents.md/) — Universal instruction file spec (Linux Foundation)
- [Claude Code Skills](https://claude.com/blog/equipping-agents-for-the-real-world-with-agent-skills) — Progressive disclosure for agent procedures
- [AgentInfra](https://github.com/JayCheng113/AgentInfra) — `.ai/` directory with three-layer loading
- [dotagent](https://github.com/johnlindquist/dotagent) — Universal parser/converter for AI rules across IDEs
- [ACS](https://acs.jackby03.com/) — Agentic Collaboration Standard proposing `.agents/`

## Install

### Quick install (prebuilt binary)

```bash
curl -fsSL https://raw.githubusercontent.com/btakita/agent-runbooks/main/install.sh | sh
```

### Cargo

```bash
cargo install agent-runbooks
```

### From source

```bash
git clone https://github.com/btakita/agent-runbooks.git
cd agent-runbooks
cargo install --path .
```

## License

[CC0 1.0](LICENSE) — Public domain. Use this convention however you like.
