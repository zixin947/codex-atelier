#!/usr/bin/env python3
"""Create a portable .skill ZIP without installing or modifying the source."""

import argparse
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile

from check_skill import validate


def package(root, output):
    root = Path(root).resolve()
    output = Path(output).resolve()
    if output.is_relative_to(root):
        raise ValueError("Output must be outside the source skill directory")
    if output.exists():
        raise FileExistsError("Output already exists: " + str(output))
    errors = validate(root)
    if errors:
        raise ValueError("Invalid skill: " + "; ".join(errors))
    files = sorted(p for p in root.rglob("*") if p.is_file() and not any(part in {"__pycache__", ".DS_Store", ".git"} for part in p.relative_to(root).parts) and p.suffix != ".pyc")
    output.parent.mkdir(parents=True, exist_ok=True)
    with ZipFile(output, "x", compression=ZIP_DEFLATED) as archive:
        for file in files:
            archive.write(file, (Path(root.name) / file.relative_to(root)).as_posix())
    with ZipFile(output) as archive:
        if archive.testzip():
            raise ValueError("Archive integrity check failed")
    return len(files)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("skill", type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    try:
        count = package(args.skill, args.output)
    except (ValueError, OSError) as error:
        parser.error(str(error))
    print(f"Packaged {count} files: {args.output}")


if __name__ == "__main__":
    raise SystemExit(main())
