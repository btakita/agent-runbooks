# Notification

A cross-document message that appends information to a target document's exchange component and signals the receiving agent to re-evaluate affected conclusions.

## Properties

- **Visible:** Written directly to the document (not deferred to a queue)
- **Source-tagged:** Includes the originating document/session
- **Re-evaluation directive:** Tells the receiving agent which conclusions may be invalidated

## Format

```markdown
> **[NOTIFY from <source>]** (<timestamp>)
> <message>
>
> **Re-evaluate:** <affected conclusions>
```

## Relationships

- Notifications are delivered by `agent-doc notify` (append to exchange + snapshot update)
- Distinguished from claims log (async queue) by immediacy and visibility
