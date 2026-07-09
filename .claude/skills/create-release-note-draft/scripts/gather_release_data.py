#!/usr/bin/env python3
"""Gather the raw data needed to draft a release note: the previous release
tag, the merged PRs (or commits, if PRs can't be resolved) since then, and a
compare-view URL. Prints a single JSON object to stdout; does no writing.

Usage:
  gather_release_data.py [--since TAG] [--until REF] [--repo OWNER/NAME]

--since  Tag/ref to start from. Defaults to the latest GitHub Release
         (`gh release list`), falling back to the latest reachable git tag.
         Pass "root" to mean "from the beginning of history".
--until  Ref to end at (exclusive of nothing before it). Defaults to HEAD.
--repo   owner/name. Defaults to the current repo via `gh repo view`.
"""

import argparse
import json
import re
import subprocess
import sys


def run(cmd, check=True):
    result = subprocess.run(cmd, capture_output=True, text=True)
    if check and result.returncode != 0:
        raise RuntimeError(
            f"command failed: {' '.join(cmd)}\n{result.stderr.strip()}"
        )
    return result.stdout.strip()


def gh_json(args):
    out = run(["gh", *args])
    return json.loads(out) if out else None


def resolve_repo(explicit):
    if explicit:
        return explicit
    data = gh_json(["repo", "view", "--json", "nameWithOwner"])
    return data["nameWithOwner"]


def resolve_since(explicit, until):
    if explicit and explicit != "root":
        return explicit
    if explicit == "root":
        return None

    try:
        releases = gh_json(
            ["release", "list", "--limit", "1", "--json", "tagName,isDraft,isPrerelease"]
        )
        if releases:
            return releases[0]["tagName"]
    except RuntimeError:
        pass

    try:
        return run(["git", "describe", "--tags", "--abbrev=0", until])
    except RuntimeError:
        return None


PR_PATTERNS = [
    re.compile(r"Merge pull request #(\d+) from"),
    re.compile(r"\(#(\d+)\)\s*$"),
]


def extract_pr_numbers(since, until):
    range_spec = f"{since}..{until}" if since else until
    log = run(["git", "log", range_spec, "--pretty=format:%s"], check=False)
    numbers = []
    seen = set()
    for line in log.splitlines():
        for pattern in PR_PATTERNS:
            match = pattern.search(line)
            if match:
                n = int(match.group(1))
                if n not in seen:
                    seen.add(n)
                    numbers.append(n)
                break
    return numbers


def fetch_pr(number):
    return gh_json(
        [
            "pr",
            "view",
            str(number),
            "--json",
            "number,title,body,labels,author,url,mergedAt,isCrossRepository",
        ]
    )


def since_commit_date(since):
    """ISO 8601 date of the `since` ref, used as the boundary for "new
    contributor" checks. None if there's no boundary (whole history)."""
    if not since:
        return None
    return run(["git", "log", "-1", "--format=%aI", since], check=False) or None


def had_merged_pr_before(repo, login, before_date):
    """True if `login` has a merged PR before `before_date` in this repo.
    Used to tell a genuinely new contributor from someone (e.g. the
    maintainer, a bot) whose first PR *within the fetched range* just
    happens to be the earliest one we saw — that in-range-only comparison
    is not enough on its own."""
    result = gh_json(
        [
            "pr",
            "list",
            "--repo",
            repo,
            "--state",
            "merged",
            "--author",
            login,
            "--search",
            f"merged:<{before_date}",
            "--json",
            "number",
            "--limit",
            "1",
        ]
    )
    return bool(result)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--since")
    parser.add_argument("--until", default="HEAD")
    parser.add_argument("--repo")
    args = parser.parse_args()

    repo = resolve_repo(args.repo)
    since = resolve_since(args.since, args.until)

    pr_numbers = extract_pr_numbers(since, args.until)

    prs = []
    for n in pr_numbers:
        try:
            pr = fetch_pr(n)
        except RuntimeError:
            continue
        if pr is None:
            continue
        prs.append(pr)

    boundary_date = since_commit_date(since)
    checked = {}
    for pr in prs:
        author = pr.get("author") or {}
        login = author.get("login")
        is_bot = author.get("is_bot", False)
        if not login or is_bot or not boundary_date:
            # Bots are never "new contributors" in the celebratory sense, and
            # without a boundary date (whole-history run) the question is
            # meaningless — everyone's PR is trivially "first".
            pr["isLikelyFirstContribution"] = False
            continue
        if login not in checked:
            checked[login] = not had_merged_pr_before(repo, login, boundary_date)
        pr["isLikelyFirstContribution"] = checked[login]

    range_spec = f"{since}..{args.until}" if since else args.until
    raw_commits = run(
        ["git", "log", range_spec, "--pretty=format:%h %s"], check=False
    ).splitlines()

    compare_url = None
    if since:
        compare_url = f"https://github.com/{repo}/compare/{since}...{args.until}"

    print(
        json.dumps(
            {
                "repo": repo,
                "since_tag": since,
                "until_ref": args.until,
                "compare_url": compare_url,
                "pr_count": len(prs),
                "prs": prs,
                "raw_commits_fallback": raw_commits,
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    try:
        main()
    except RuntimeError as e:
        print(f"error: {e}", file=sys.stderr)
        sys.exit(1)
