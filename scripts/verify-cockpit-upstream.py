#!/usr/bin/env python3
"""Verify vendored sources without executing or changing user data."""
import hashlib
import json
from pathlib import Path
root = Path(__file__).resolve().parents[1] / 'src-tauri/crates/cockpit-session-compat'
manifest = json.loads((root / 'UPSTREAM-SHA256.json').read_text())
assert manifest['commit'] == '02498ace91832248f0c6d43f6ea3894a2514aed7'
expected = set(manifest['files'])
actual = {str(p.relative_to(root / 'upstream')) for p in (root / 'upstream').rglob('*.rs')}
assert expected == actual, f'Source set changed: {expected ^ actual}'
for name, digest in manifest['files'].items():
    assert hashlib.sha256((root / 'upstream' / name).read_bytes()).hexdigest() == digest, name
print(f'PASS: {len(expected)} original files match {manifest["commit"]}')
