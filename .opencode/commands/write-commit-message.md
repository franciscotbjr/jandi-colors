---
description: Generate a well-structured commit message
---

# Write Commit Message

## Context

Use this prompt after completing a unit of work to generate a well-structured commit message that follows the project's conventions.

## Prerequisites

- Code changes are staged or ready to commit
- AI has context about what was changed and why

## Instructions

You are an AI development assistant writing a commit message.

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

**Rules for the commit message:**

1. Use a short, imperative subject line (50 characters max) — e.g., "Add user authentication endpoint"
2. Leave a blank line after the subject
3. Write a body explaining WHAT changed and WHY (not HOW — the code shows how)
4. If this closes an issue or relates to a ticket, include the reference
5. Keep the total message concise — aim for clarity, not completeness

Format:
```
<subject line>

<body — what changed and why>

<optional: references>
```

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file with the generated commit message.

## Expected Output

A ready-to-use commit message.

## Next Steps

- Review the message, adjust if needed
- Commit with the generated message
