#!/usr/bin/env python3
"""Update all commit SHAs and line numbers in README.md based on rosu_pp_js.d.ts."""

import json
import re
import urllib.request
from pathlib import Path

REPO = "MaxOhn/rosu-pp-js"
BRANCH = "main"
README = Path("README.md")
DTS = Path("rosu_pp_js.d.ts")


def get_latest_sha() -> str:
    url = f"https://api.github.com/repos/{REPO}/commits/{BRANCH}"
    req = urllib.request.Request(url, headers={"User-Agent": "readme-updater"})
    with urllib.request.urlopen(req) as resp:
        return json.load(resp)["sha"]


def get_line_ranges(dts_path: Path) -> dict[str, tuple[int, int]]:
    """
    Parse the .d.ts file and return {TypeName: (start_line, end_line)}.
    Handles top-level `class`, `interface`, `enum`, and `type` declarations.
    """
    lines = dts_path.read_text().splitlines()
    total = len(lines)

    # Match the start of any top-level declaration (no leading whitespace)
    decl_re = re.compile(
        r"^(?:export\s+)?(?:(?:abstract|declare)\s+)?"
        r"(class|interface|enum|type)\s+(\w+)"
    )

    # First pass: collect all declaration start lines
    decls: list[tuple[int, str]] = []  # (0-based line index, name)
    for i, line in enumerate(lines):
        m = decl_re.match(line)
        if m:
            decls.append((i, m.group(2)))

    # Second pass: determine end of each declaration
    ranges: dict[str, tuple[int, int]] = {}
    for idx, (start_idx, name) in enumerate(decls):
        # End is one line before the next declaration, or EOF
        if idx + 1 < len(decls):
            end_idx = decls[idx + 1][0] - 1
        else:
            end_idx = total - 1

        # Trim trailing blank lines
        while end_idx > start_idx and not lines[end_idx].strip():
            end_idx -= 1

        ranges[name] = (start_idx + 1, end_idx + 1)  # convert to 1-based

    return ranges


def update_readme(sha: str, ranges: dict[str, tuple[int, int]]) -> None:
    content = README.read_text()

    md_link = re.compile(
        r"\[`?(\w+)`?\]\("
        rf"(https://github\.com/{re.escape(REPO)}/blob/[0-9a-f]{{7,40}}/[^)]+)"
        r"\)"
    )

    def replace(m: re.Match) -> str:
        name = m.group(1)
        old_url = m.group(2)
        new_url = re.sub(
            rf"(github\.com/{re.escape(REPO)}/blob/)[0-9a-f]{{7,40}}",
            rf"\g<1>{sha}",
            old_url,
        )
        if name in ranges:
            start, end = ranges[name]
            new_url = re.sub(r"#L\d+(?:-L\d+)?$", f"#L{start}-L{end}", new_url)
        return f"[`{name}`]({new_url})"

    updated = md_link.sub(replace, content)

    if updated == content:
        print("Nothing to update.")
        return

    README.write_text(updated)
    print(f"README updated (SHA: {sha[:7]}).")
    for m in md_link.finditer(content):
        name = m.group(1)
        if name in ranges:
            print(f"  {name}: L{ranges[name][0]}-L{ranges[name][1]}")


if __name__ == "__main__":
    sha = get_latest_sha()
    ranges = get_line_ranges(DTS)
    update_readme(sha, ranges)
