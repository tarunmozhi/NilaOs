#!/usr/bin/env python3
"""Validate the Nila OS device profile without third-party dependencies."""
from pathlib import Path
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
PROFILE = ROOT / "device/vivo/1906/device.toml"


def main() -> int:
    try:
        data = tomllib.loads(PROFILE.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as exc:
        print(f"ERROR: cannot parse {PROFILE.relative_to(ROOT)}: {exc}", file=sys.stderr)
        return 1

    required = {
        ("device", "name"): "vivo Y11 / vivo 1906",
        ("device", "codename"): "vivo1906",
        ("device", "model"): "PD1930F",
        ("hardware", "abi"): "arm64",
        ("compatibility", "deployment"): "mobile-only",
    }
    errors = []
    for (section, key), expected in required.items():
        actual = data.get(section, {}).get(key)
        if actual != expected:
            errors.append(f"{section}.{key}: expected {expected!r}, got {actual!r}")

    hardware = data.get("hardware", {})
    if not isinstance(hardware.get("ram_mb"), int) or hardware["ram_mb"] < 1:
        errors.append("hardware.ram_mb must be a positive integer")
    if not isinstance(hardware.get("storage_gb"), int) or hardware["storage_gb"] < 1:
        errors.append("hardware.storage_gb must be a positive integer")

    if errors:
        print("Device profile validation failed:", file=sys.stderr)
        for error in errors:
            print(f" - {error}", file=sys.stderr)
        return 1

    print("Device profile OK: vivo Y11 / PD1930F (ARM64, mobile-only)")
    print("Note: profile validation does not verify hardware support or bootability.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
