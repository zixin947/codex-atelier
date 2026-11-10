#!/usr/bin/env python3
"""Check portable skill links, evidence hashes, palette and CSS consistency."""

import argparse
import hashlib
import json
import re
from pathlib import Path
from urllib.parse import unquote, urlsplit

from contrast import check_palette


def validate(root):
    root = Path(root).resolve()
    errors = []
    required = ["SKILL.md", "agents/openai.yaml", "references/index.md", "references/source-registry.md", "references/evidence/manifest.json", "assets/preview.html", "assets/endfield-tokens.css", "assets/endfield-components.css", "assets/palette.json"]
    for name in required:
        if not (root / name).is_file():
            errors.append("Missing: " + name)
    if errors:
        return errors
    for file in root.rglob("*"):
        if file.is_symlink():
            errors.append("Symlink is not portable: " + str(file.relative_to(root)))
    skill = (root / "SKILL.md").read_text(encoding="utf-8")
    if not skill.startswith("---\n") or "name: endfield-ui-design" not in skill.split("---", 2)[1]:
        errors.append("Missing expected frontmatter")
    if "[TODO:" in skill:
        errors.append("Unfinished scaffold")
    graph = {}
    for file in root.rglob("*.md"):
        graph[file] = []
        for href in re.findall(r"!?\[[^\]]*\]\(([^\s)]+)\)", file.read_text(encoding="utf-8")):
            parsed = urlsplit(href)
            if parsed.scheme or not parsed.path:
                continue
            target = (file.parent / unquote(parsed.path)).resolve()
            if not target.is_relative_to(root):
                errors.append("Link leaves skill: " + href)
            elif not target.is_file():
                errors.append(str(file.relative_to(root)) + " broken link: " + href)
            elif target.suffix == ".md":
                graph[file].append(target)
    reachable = set()
    queue = [root / "SKILL.md"]
    while queue:
        current = queue.pop()
        if current not in reachable:
            reachable.add(current)
            queue.extend(graph.get(current, []))
    for file in (root / "references").rglob("*.md"):
        if file not in reachable:
            errors.append("Reference not routed: " + str(file.relative_to(root)))
    manifest = json.loads((root / "references/evidence/manifest.json").read_text(encoding="utf-8"))
    registered = set()
    for record in manifest["screenshots"]:
        image = (root / record["path"]).resolve()
        registered.add(image)
        if not image.is_relative_to(root) or not image.is_file():
            errors.append("Missing/invalid evidence image: " + record["path"])
            continue
        raw = image.read_bytes()
        if not raw.startswith(b"\x89PNG\r\n\x1a\n") or hashlib.sha256(raw).hexdigest() != record["sha256"]:
            errors.append("Evidence content mismatch: " + record["path"])
        if not record.get("source_url") or not record.get("scope"):
            errors.append("Missing evidence context: " + record["path"])
    for image in (root / "references/screenshots").glob("*.png"):
        if image.resolve() not in registered:
            errors.append("Unregistered evidence: " + image.name)
    data = json.loads((root / "assets/palette.json").read_text(encoding="utf-8"))
    css = (root / "assets/endfield-tokens.css").read_text(encoding="utf-8")
    for theme, tokens in data["themes"].items():
        block = re.search(r"""\[data-ef-theme=['"]""" + re.escape(theme) + r"""['"]\]\s*\{([^}]+)\}""", css)
        if not block:
            errors.append("Missing CSS theme: " + theme)
            continue
        for name, value in tokens.items():
            if not re.search(r"--ef-" + re.escape(name) + r"\s*:\s*" + re.escape(value) + r"\s*;", block[1], re.I):
                errors.append("CSS/palette drift: " + theme + "." + name)
    for result in check_palette(root / "assets/palette.json"):
        if not result["passed"]:
            errors.append("Contrast failed: " + result["theme"] + "." + result["name"])
    return errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("skill", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        errors = validate(args.skill)
    except (KeyError, ValueError, OSError, TypeError) as error:
        errors = [str(error)]
    if errors:
        print("\n".join(errors))
        return 1
    print("Skill links, routed references, evidence hashes, CSS tokens and palette checks passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
