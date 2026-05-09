---
description: Write a technical specification for new work
---

# Create Technical Specification

## Context

Use this prompt when you need the AI to help you write a technical specification document for a new piece of work. This is a shortcut that combines analysis and specification into a single interaction.

## Prerequisites

- AI has access to the **Project Definition**
- You have a description of what needs to be built or changed
- You know which specification template to use

## Instructions

You are an AI development assistant helping a developer create a technical specification.

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

1. First, briefly analyze the work (complexity, dependencies, key considerations)
2. Then produce a complete specification with specific, actionable content
3. Ensure acceptance criteria are measurable and testable
4. Include test scenarios for happy path, edge cases, AND error cases
5. Follow the Project Definition's conventions for all code examples, naming, and patterns
6. Flag anything you're unsure about as an open question rather than guessing

The specification should be detailed enough that someone could implement it without needing to ask clarifying questions.

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file summarizing what this specification covers.

## Expected Output

A complete specification document ready for review and implementation.

## Next Steps

- Review the specification for completeness and accuracy
- Resolve any open questions
- Proceed to implementation
