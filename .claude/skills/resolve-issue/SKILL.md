---
name: resolve-issue
description: >
  Solve a GitHub issue end-to-end: research it, branch, implement, test, and open
  a pull request. Use this whenever the user asks to resolve, fix, implement,
  tackle, or work an issue by number, URL, or description (e.g. "resolve issue
  #123", "fix #45", "work on the bug in issue 12", "can you take care of
  https://github.com/.../issues/99", "implement the feature request in issue
  7"). Also trigger when the user pastes a GitHub issue link and asks you to
  handle it, or asks you to "pick up" or "close out" an issue.
---

# Resolve Issue

Turn a GitHub issue into a merged-ready pull request, the way a careful
contributor would: understand it fully before touching code, keep the change
scoped to what was asked, and verify behavior rather than just compiling it.
This skill is intentionally not tied to any one language or toolchain — it
tells you *what* to check at each step and *where* to find the project's own
answer (its CLAUDE.md/AGENTS.md, or its manifest files), rather than assuming
Rust, Node, Python, or any specific command.

The issue reference comes from whatever the user gave you: a bare number, a
full URL, or a description you'll need to search for with `gh issue list`. If
none of that resolves to exactly one issue, ask which one before proceeding —
guessing wrong here wastes the entire rest of the workflow.

## 0. Check you're not duplicating existing work

Before creating anything, check whether this issue already has a branch or PR:

```bash
git branch -a | grep -i "<issue-number>"
gh pr list --search "<issue-number> in:title,body"
```

If one exists, resume it — check out that branch, read its diff and commits to
see what's already done, and continue from there instead of starting over. If
it looks abandoned or wrong, tell the user what you found and ask how to
proceed rather than silently discarding it.

## 1. Read the issue and its root context

Fetch the issue (`gh issue view <n> --json title,body,labels,comments,url`) and
read it fully, including comments — the real requirement often lives in a
comment thread, not just the original body.

If the issue references something external — an upstream library release, a
linked spec, another PR, a linked issue in a different repo — go read *that*
too before planning. An issue that says "sync with upstream v5.23.0" is
underspecified until you've actually diffed that release; planning to-dos off
the issue title alone leads to missed or wrong changes. This is the single
biggest lever on getting the rest of the workflow right, so don't skip it even
when the issue looks self-contained at a glance.

## 2. Summarize, then plan to-dos

Write a short summary of what the issue is actually asking for, in your own
words. Then break it into concrete, checkable to-dos. Use TaskCreate/TaskUpdate
(or an equivalent todo list) if the issue needs more than a couple of steps —
it keeps you honest about what's actually done versus assumed done, especially
across a long session.

## 3. Create the branch

Only after you understand the issue (steps 1–2), branch from the repository's
default branch:

- `fix/<issue-number>-short-description` for bug fixes
- `feat/<issue-number>-short-description` for new features

Branching early, before you understand the issue, tends to produce a branch
name and initial commit scoped to the wrong thing.

## 4. Work each to-do

For every to-do:

1. **Decide the approach.** If it's genuinely ambiguous and not resolvable
   from the issue, the code, or a sensible default, ask — don't guess on
   something that would be expensive to unwind later.
2. **Check the project's own conventions before assuming a toolchain.** Look
   for `CLAUDE.md`/`AGENTS.md` (nearest directory to the file you're editing
   wins over one at the repo root) for the actual build/test/lint commands
   this project expects. If none exist, infer from manifest files
   (`Cargo.toml`, `package.json`, `pyproject.toml`, etc.) or ask. Don't
   hardcode a command from a different project you've worked in.
3. **Implement it.**
4. **Build/compile/typecheck.** On failure, read the actual error, fix the
   root cause, and retry — don't paper over it.
5. **If you changed a public signature, return type, or behavior, grep the
   whole repo for every call site before calling the to-do done.** A breaking
   change ripples into examples, other tests, and docs that a normal build of
   just the changed package won't touch — you only find them by building
   *everything* (`--workspace`/equivalent) or by grepping for the symbol
   directly. Missing one of these is the most common way this kind of task
   silently ships broken.
6. **Write or extend unit tests for the new behavior, and run them.** Fix
   failures by understanding what broke, not by weakening the assertion.
7. **Write or extend integration tests if the project has that concept, and
   run them for real when you can.** If the sandbox actually has what those
   tests need — real credentials in a `.env`, a running service, network
   access — run them, don't just confirm they compile. Compiling only proves
   syntax; running against the real dependency is what actually catches bugs
   (including bugs in the test itself). If a test mutates external state and
   fails *before* its own cleanup step executes, clean that state up manually
   before moving on — an orphaned record or resource left behind by a failed
   run is a real cost, not a rounding error.
8. **Format/lint only the files you touched this session.** If the linter or
   formatter reports issues elsewhere in the repo, don't fix them as a side
   effect and don't assume you caused them — check whether they already exist
   on a clean checkout of the default branch first. Reformatting or
   relinting an entire repository as a side effect of a scoped change buries
   your real diff in noise and is usually unwanted.
9. **Cross-check** the to-do, the issue summary, and the actual code — do they
   still agree? Requirements sometimes shift mid-implementation; catch that
   here rather than at PR time.
10. **Mark the to-do done and commit.** Committing per to-do (rather than one
    giant commit at the end) makes the eventual PR diff easier to review and
    gives you a rollback point if a later to-do goes sideways.

If the same step fails three times in a row, stop and report the error to the
user instead of continuing to retry or trying increasingly speculative fixes.

## 5. Stay in scope — but don't lose findings

Only change what the issue asked for. If you notice something else that's
broken, stale, or worth improving while you're in there, don't fix it inline —
file it as a separate issue with `gh issue create` and mention it in the PR
description under something like "Additional notes" or "Found while working
on this." This keeps the PR reviewable and makes sure the finding isn't lost
just because it was out of scope this time.

## 6. Open the pull request

Write a concise change summary, then open the PR:

```bash
gh pr create --title "..." --body "..."
```

Look for a PR template first (common locations: `.github/pull_request_template.md`,
`.github/PULL_REQUEST_TEMPLATE/*.md`, `PULL_REQUEST_TEMPLATE.md`) and fill it
in rather than writing free-form if one exists. Link the issue (e.g. "Closes
#123"). You can add extra context after the template's own sections.
