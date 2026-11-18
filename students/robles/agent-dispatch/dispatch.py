#!/usr/bin/env python3
"""Copy `agent-ready` issues from cppalliance/runvil-gateway into the fork and
trigger a Cursor cloud agent there by commenting `@cursor ...` as the logged-in
`gh` user.

Dry run is the default. `--apply` performs the writes. The upstream repository
is only ever read. Issue text is data: it is passed to `gh` as an argument or
on stdin and never executed, evaluated or interpolated into a shell.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

UPSTREAM = "cppalliance/runvil-gateway"
FORK = "sykosyber/runvil-gateway"
BRANCH = "master"
LABEL = "agent-ready"
TRUSTED_ROLES = {"triage", "write", "maintain", "admin"}
HERE = Path(__file__).resolve().parent


# ---------------------------------------------------------------- pure functions

def first_line(text: str | None) -> str:
    return (text or "").lstrip("﻿").split("\n", 1)[0].strip()


def marker_line(upstream_url: str) -> str:
    return (f"Copy of {upstream_url} for a Cursor cloud agent. "
            "Fix it here, in this fork.")


def format_fork_body(upstream_url: str, upstream_body: str | None) -> str:
    """First line marker, a blank line, then the upstream body unchanged."""
    return marker_line(upstream_url) + "\n\n" + (upstream_body or "")


def url_in_first_line(url: str, body: str | None) -> bool:
    """True if `url` appears in the first line as a whole URL.

    `.../issues/1` must not match `.../issues/10`, so the URL must not be
    followed by a digit or another path character.
    """
    pattern = re.escape(url) + r"(?![0-9A-Za-z_/-])"
    return re.search(pattern, first_line(body)) is not None


def norm_title(t: str | None) -> str:
    """Compare titles ignoring backticks and spacing (a hand copy lost them)."""
    return " ".join((t or "").replace("`", "").split())


def find_fork_issue(upstream: dict, fork_issues: list[dict]) -> tuple[dict, str] | None:
    """Return (fork issue, rule) if this upstream issue was already copied.

    Rule 1 (the real one): the fork issue's first body line contains the
    upstream URL.
    Rule 2 (legacy, errs toward not spending money): a fork issue whose first
    line is not a "Copy of http..." marker and whose title equals the upstream
    title. This covers copies made by hand before this tool existed.
    """
    url = upstream["url"]
    for fi in fork_issues:
        if url_in_first_line(url, fi.get("body")):
            return fi, "marker"
    for fi in fork_issues:
        is_marked = first_line(fi.get("body")).startswith("Copy of http")
        if not is_marked and norm_title(fi.get("title")) == norm_title(upstream.get("title")):
            return fi, "legacy title match"
    return None


def role_is_trusted(perm: dict | None) -> bool:
    if not perm:
        return False
    role = (perm.get("role_name") or "").lower()
    if role:
        return role in TRUSTED_ROLES
    return (perm.get("permission") or "").lower() in {"write", "admin"}


def latest_labeler(events: list[dict], label: str) -> str | None:
    login = None
    for ev in events:
        if ev.get("event") == "labeled" and (ev.get("label") or {}).get("name") == label:
            login = (ev.get("actor") or {}).get("login")
    return login


def read_template(path: Path = HERE / "trigger-comment.md") -> str:
    return path.read_text(encoding="utf-8").strip()


# ---------------------------------------------------------------- gh wrappers

def gh(args: list[str], stdin: str | None = None) -> str:
    r = subprocess.run(["gh", *args], input=stdin, capture_output=True,
                       text=True, encoding="utf-8")
    if r.returncode != 0:
        raise RuntimeError(f"gh {' '.join(args[:3])} failed: {r.stderr.strip()}")
    return r.stdout


def gh_json(args: list[str]):
    return json.loads(gh(args) or "null")


def labeler_info(number: int) -> tuple[str | None, bool, str]:
    """(login, trusted, note). Fails closed: any lookup problem is untrusted."""
    try:
        pages = gh_json(["api", "--paginate", "--slurp",
                         f"repos/{UPSTREAM}/issues/{number}/timeline"])
        flat = [e for page in pages for e in page]
        login = latest_labeler(flat, LABEL)
        if not login:
            return None, False, "labeler not found in timeline"
        perm = gh_json(["api", f"repos/{UPSTREAM}/collaborators/{login}/permission"])
        role = perm.get("role_name") or perm.get("permission")
        return login, role_is_trusted(perm), f"role={role}"
    except Exception as exc:  # noqa: BLE001 - fail closed, report why
        return None, False, f"lookup failed: {exc}"


# ---------------------------------------------------------------- main

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--apply", action="store_true",
                    help="perform the writes (default is a dry run)")
    ap.add_argument("--max", type=int, default=1,
                    help="most dispatches per run (default 1; each is billed to the seat)")
    args = ap.parse_args()
    mode = "APPLY" if args.apply else "DRY RUN"
    print(f"[{mode}] upstream={UPSTREAM} fork={FORK} label={LABEL} max={args.max}")

    template = read_template()
    upstream_issues = gh_json(["issue", "list", "--repo", UPSTREAM, "--label", LABEL,
                               "--state", "open", "--limit", "100",
                               "--json", "number,title,body,url,labels"])
    fork_issues = gh_json(["issue", "list", "--repo", FORK, "--state", "all",
                           "--limit", "1000", "--json", "number,title,body,url"])
    print(f"{len(upstream_issues)} open upstream issue(s) labelled {LABEL}; "
          f"{len(fork_issues)} fork issue(s) checked")

    pending = []
    for up in sorted(upstream_issues, key=lambda i: i["number"]):
        hit = find_fork_issue(up, fork_issues)
        if hit:
            fi, rule = hit
            print(f"#{up['number']}: skipped, already dispatched ({rule}) "
                  f"-> fork #{fi['number']} {fi['url']}")
            continue
        login, trusted, note = labeler_info(up["number"])
        who = login or "unknown"
        if not trusted:
            print(f"#{up['number']}: skipped, label added by {who} "
                  f"without confirmed Triage access ({note})")
            continue
        print(f"#{up['number']}: label added by {who} ({note})")
        pending.append(up)

    cap = max(args.max, 0)
    batch, over = pending[:cap], pending[cap:]
    for up in over:
        print(f"#{up['number']}: skipped, --max {args.max} reached this run")
    if not batch:
        print("Nothing to dispatch.")
        return 0

    print(f"{'Syncing' if args.apply else 'Would sync'} {FORK} {BRANCH} "
          "from upstream (fast-forward only)")
    if args.apply:
        try:
            gh(["repo", "sync", FORK, "--branch", BRANCH])  # no --force, ever
        except RuntimeError as exc:
            print(f"STOP: fork sync is not a clean fast-forward or failed: {exc}")
            return 2

    for up in batch:
        title, body = up["title"], format_fork_body(up["url"], up.get("body"))
        if not args.apply:
            print(f"#{up['number']}: would create fork issue titled {title!r} with body:")
            print("    " + body.replace("\n", "\n    ")[:600] + ("..." if len(body) > 600 else ""))
            print(f"#{up['number']}: would comment on it: {template}")
            continue
        out = gh(["issue", "create", "--repo", FORK, "--title", title,
                  "--body-file", "-"], stdin=body).strip()
        fork_url = out.splitlines()[-1]
        gh(["issue", "comment", fork_url, "--body-file", "-"], stdin=template)
        print(f"#{up['number']}: dispatched -> {fork_url}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
