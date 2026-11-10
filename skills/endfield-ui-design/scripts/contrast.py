#!/usr/bin/env python3
"""Offline WCAG sRGB contrast for opaque RGB colors; no network or app access."""

import argparse
import json
import math
import re
from pathlib import Path


def luminance(color):
    if not isinstance(color, str) or not re.fullmatch(r"#[0-9a-fA-F]{6}", color):
        raise ValueError("Use an opaque six-digit RGB color, for example #20231f")
    channels = [int(color[i:i + 2], 16) / 255 for i in (1, 3, 5)]
    linear = [v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4 for v in channels]
    return sum(v * weight for v, weight in zip(linear, (0.2126, 0.7152, 0.0722)))


def contrast(foreground, background):
    a, b = sorted((luminance(foreground), luminance(background)))
    return (b + 0.05) / (a + 0.05)


def check_palette(path):
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    results = []
    for theme, tokens in data["themes"].items():
        for pair in data["checks"]:
            ratio = contrast(tokens[pair["foreground"]], tokens[pair["background"]])
            results.append({"theme": theme, "name": pair["name"], "ratio": round(ratio, 3), "minimum": pair["minimum"], "passed": ratio >= pair["minimum"]})
    return results


def self_test():
    assert math.isclose(contrast("#000000", "#FFFFFF"), 21.0)
    assert math.isclose(contrast("#123456", "#123456"), 1.0)
    assert math.isclose(contrast("#777777", "#FFFFFF"), 4.478089453577214)
    for invalid in ("red", "#fff", "#ffffff80", "#zzzzzz", None):
        try:
            luminance(invalid)
        except ValueError:
            continue
        raise AssertionError("Invalid input accepted")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--palette", type=Path)
    parser.add_argument("--foreground")
    parser.add_argument("--background")
    parser.add_argument("--minimum", type=float, default=4.5)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("Contrast self-test passed")
        return 0
    try:
        if args.palette:
            if args.foreground or args.background:
                parser.error("Use either --palette or a foreground/background pair")
            results = check_palette(args.palette)
        else:
            if not args.foreground or not args.background:
                parser.error("Provide --palette or both --foreground and --background")
            if not math.isfinite(args.minimum) or not 1 <= args.minimum <= 21:
                parser.error("--minimum must be between 1 and 21")
            ratio = contrast(args.foreground, args.background)
            results = [{"ratio": round(ratio, 3), "minimum": args.minimum, "passed": ratio >= args.minimum}]
    except (ValueError, KeyError, OSError, TypeError) as error:
        parser.error(str(error))
    print(json.dumps(results, ensure_ascii=False, indent=2))
    return 0 if results and all(r["passed"] for r in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
