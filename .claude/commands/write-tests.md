# Write Tests

## Context

Use this prompt when you need the AI to write tests for existing or newly implemented code. This is useful for improving test coverage or when tests weren't written alongside implementation.

## Prerequisites

- AI has access to the **Project Definition** (for testing framework and conventions)
- The code to be tested exists
- Optionally: a specification with test scenarios defined

## Instructions

You are an AI development assistant writing tests for existing or new code.

**Before starting:** Check `.stateful-spec/memory.md` for an Open Session section. If found, you are working within an open implementation cycle — all contributions must be registered under that iteration.

1. Use the testing framework and patterns specified in the Project Definition
2. Follow the project's test naming convention
3. Write tests for:
   - **Happy path** — Normal, expected usage
   - **Edge cases** — Boundary values, empty inputs, large inputs, null/undefined
   - **Error cases** — Invalid inputs, failure scenarios, error handling
4. Each test should be independent — no test should depend on another test's state
5. Use descriptive test names that explain what is being verified
6. Include setup/teardown if needed (fixtures, mocks, test data)
7. Keep tests focused — one assertion concept per test

Place tests in the location specified by the Project Definition (co-located with source, tests/ directory, etc.).

**Session tracking:** If an Open Session was detected, append a timestamped entry to the **Session Log** in the referenced iteration file summarizing what tests were written.

## Expected Output

Complete test file(s) with tests covering happy path, edge cases, and error cases, following the project's testing conventions.

## Next Steps

- Review the generated tests
- Run them to verify they pass
- Add any additional scenarios specific to your domain knowledge
