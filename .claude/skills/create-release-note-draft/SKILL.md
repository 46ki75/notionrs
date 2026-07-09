---
name: create-release-note-draft
description: >
  Draft a user-facing release note / changelog from the commits and merged
  pull requests since the last release, then save it to a file (and
  optionally open it as a draft GitHub release). Use whenever the user asks
  to draft release notes, write a changelog, prepare a GitHub release,
  summarize what shipped since the last version, or asks something like
  "what changed since v1.2.0" or "can you put together release notes for
  this". Fetches the underlying data itself via `gh`/`git` — don't ask the
  user to paste in a PR list or commit log first.
---

# Create Release Note Draft

Produce a release note the way a maintainer writing for their users would:
grouped by what changed, breaking changes impossible to miss, and nothing
that only matters to contributors (CI tweaks, internal refactors) cluttering
it up.

## 1. Gather the data

Don't rely on your own memory of `gh` incantations for this — use the bundled
script, which resolves the previous release tag correctly (via GitHub
Releases, not just the latest git tag, which matters in monorepos or repos
that also tag individual sub-packages) and fetches merged-PR metadata with a
proper new-contributor check:

```bash
python3 <skill_dir>/scripts/gather_release_data.py [--since TAG] [--until REF] [--repo OWNER/NAME]
```

- Omit `--since` to auto-detect the previous release; pass a tag to override,
  or `root` to mean "the whole history."
- Omit `--until` to mean `HEAD`; pass a tag/branch to draft for a specific cut.
- The script prints one JSON object: `repo`, `since_tag`, `until_ref`,
  `compare_url`, `prs` (each with `number`, `title`, `body`, `labels`,
  `author`, `url`, `mergedAt`, `isLikelyFirstContribution`), and
  `raw_commits_fallback` (plain `git log` output, for when a repo doesn't
  merge via PRs, or a PR-extraction regex misses something).

If `prs` comes back empty but `raw_commits_fallback` has entries, the repo
probably doesn't use GitHub's merge-commit or squash-merge conventions this
script parses for — fall back to categorizing directly from the commit
messages instead.

## 2. Categorize each change

For each PR (or commit, in the fallback case), decide its category using this
priority order — each rung is a fallback for when the one above doesn't
apply, not a competing signal to average together:

1. **PR labels first.** Map the repo's actual label names to the categories
   below; labels vary by repo (`feature`/`enhancement`, `bug`/`bugfix`,
   `breaking`/`breaking-change`, etc.) so check what's actually in use rather
   than assuming exact spelling.
2. **Conventional Commit prefix**, if no usable label: `feat:` → Features,
   `fix:` → Bug Fixes, `perf:` → Performance, `docs:` → Documentation,
   `chore:`/`build:`/`ci:` → Maintenance.
3. **Infer from the diff/PR body**, only if neither of the above gives an
   answer.

Categories: **Features**, **Bug Fixes**, **Performance**, **Maintenance /
Chores**, **Documentation**, **Breaking Changes**.

- **Deduplicate**: if multiple PRs address the same logical change (a feature
  PR plus its own follow-up fix), merge them into one bullet rather than
  listing both.
- **Breaking changes always get surfaced**, even if you'd otherwise omit the
  underlying PR as maintenance-only — include a short migration note or link
  to one.
- **Omit CI-only, test-only, and purely internal refactors** unless they
  change build output or public behavior. Routine dependency bumps
  (Dependabot-style PRs) almost always belong here too — don't let two dozen
  patch-bump PRs drown out the three PRs a user actually cares about.

## 3. Write the draft

Use this exact structure, omitting any section with nothing in it:

```markdown
# <Repository Name> v<VERSION> — Release Notes

## Highlights

<1-3 sentence summary of the most impactful changes>

## What's Changed

### Features

- <Short description> (#<PR number>)

### Bug Fixes

- <Short description> (#<PR number>)

### Performance

- <Short description> (#<PR number>)

### Maintenance / Chores

- <Short description> (#<PR number>)

### Documentation

- <Short description> (#<PR number>)

### Breaking Changes

- <Description of the breaking change and migration path> (#<PR number>)

## New Contributors

- @<username> made their first contribution in #<PR number>

## Full Changelog

<compare_url from the gathered data>
```

Rules for the prose itself:

- **Write for end-users and integrators**, not internal developers — avoid
  implementation details unless they affect the public API or the user
  experience.
- **New Contributors** comes from `isLikelyFirstContribution: true` entries
  only — that field is already boundary-checked against PRs merged before
  this release, not just "earliest in this batch," so trust it over your own
  guess from the PR list.
- **Every bullet links to its PR(s) or commit(s).**
- **Tone**: professional, concise, factual — no marketing language.
- **Length**: aim for one line per change; a second line only for breaking
  changes or complex migrations.
- **Language**: English, unless the user asked for another.

## 4. Save it, and offer to publish

Write the draft to a file — `RELEASE_NOTES_DRAFT.md` at the repo root is a
reasonable default, but ask if the user has a preferred location or if one is
implied by the repo's own conventions (e.g. a `CHANGELOG.md` this repo already
maintains by hand).

Don't create an actual GitHub release as a side effect of drafting one — that
action is visible to everyone watching the repo, so offer it and let the user
decide:

```bash
gh release create <tag> --draft --notes-file <path> --title "<title>"
```
