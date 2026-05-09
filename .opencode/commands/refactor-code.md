---
description: Safely restructure code without changing behavior
---

# Refactor Code

## Context

Use this prompt when you need to restructure existing code without changing its external behavior. The AI will help plan and execute the refactoring safely.

## Prerequisites

- AI has access to the **Project Definition**
- You know what code needs refactoring and why
- Existing tests pass (baseline for behavior preservation)

## Instructions

You are an AI development assistant helping refactor code safely.

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

1. Analyze the current code and identify the specific problems
2. Propose a refactoring plan with ordered steps — each step should be independently committable
3. For each step, explain what changes and why
4. Implement the refactoring step by step
5. After each step, confirm that:
   - External behavior is unchanged
   - All existing tests should still pass
   - The code follows the Project Definition's conventions
6. If new tests are needed to cover the refactored code, write them
7. Do NOT change any public API, interface, or behavior unless explicitly discussed

If you think the refactoring should also change behavior (e.g., fix a bug discovered during refactoring), flag it separately — do not mix behavior changes with structural refactoring.

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file summarizing what was refactored.

## Expected Output

1. Analysis of current problems
2. Step-by-step refactoring plan
3. Refactored code (incrementally, step by step)
4. Confirmation of behavioral preservation at each step

## Next Steps

- Review each refactoring step
- Run tests after each step to confirm behavior is preserved
- Commit each step independently if possible
