---
name: resolve-issue
description: Solve a GitHub issue by creating a new branch, implementing the fix or feature, and creating a pull request.
---

Solve the issue ${issue}.

## Guardrails

- Create a new branch from the default branch before making any changes. Use the naming convention `fix/<issue-number>-short-description` or `feat/<issue-number>-short-description`. Use `fix/` for bug fixes and `feat/` for new features.
- Only make changes directly related to the issue. If you discover unrelated improvements, report them as separate issues instead of fixing them here.
- If a step fails 3 times consecutively, stop and report the error to the user. Do not continue or loop indefinitely.

## Steps

1. Create a new branch from the default branch.
2. Check the issue.
3. Summarize the issue.
4. Refer to the summary and create to-dos.
5. Refer to the to-dos and solve them one by one.
   1. Review the to-do and decide what to do.
   2. Implement the features.
   3. Compile, build, lint, or check the types. If it fails, analyze the error, fix the code, and retry.
   4. Write unit tests.
   5. Run the unit tests. If they fail, analyze the error, fix the code or tests, and retry.
   6. Write integration tests.
   7. Run the integration tests. If they fail, analyze the error, fix the code or tests, and retry.
   8. Check consistency between the to-do, the summary, and the code.
   9. Mark the to-do as done.
   10. Commit the changed files.
6. Create a change summary.
7. Create a pull request based on the change summary. If a pull request template exists in the repository, use it. You may add additional notes at the end of the pull request body.
