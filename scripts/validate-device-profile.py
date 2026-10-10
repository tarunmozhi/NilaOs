#!/usr/bin/env python3
"""Validate the Nila OS device profile without third-party dependencies."""
from pathlib import Path
import sys
import tomllib
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PROFILE = ROOT / "device/vivo/1906/device.toml"

REQUIRED_VALUES = {
    ("device", "name"): "vivo Y11 / vivo 1906",
    ("device", "codename"): "vivo1906",
    ("device", "model"): "PD1930F",
    ("hardware", "abi"): "arm64",
    ("compatibility", "deployment"): "mobile-only",
    ("software", "android_reference"): "Android 11 / Funtouch OS 10.5",
    ("software", "security_patch"): "2022-03-01",
    ("software", "kernel"): "Linux 4.9.227-perf+",
    ("software", "build"): "PD1930CF_EX_A_6.71.15",
}
REQUIRED_TABLES = ("device", "hardware", "compatibility", "software", "features")


def validate_profile(data: Any) -> list[str]:
    """Return schema errors instead of crashing on malformed TOML structures."""
    errors: list[str] = []
    if not isinstance(data, dict):
        return ["profile root must be a TOML table"]

    for section in REQUIRED_TABLES:
        if not isinstance(data.get(section), dict):
            errors.append(f"{section} must be a TOML table")

    for (section, key), expected in REQUIRED_VALUES.items():
        values = data.get(section)
        if not isinstance(values, dict):
            continue
        actual = values.get(key)
        if actual != expected:
            errors.append(f"{section}.{key}: expected {expected!r}, got {actual!r}")

    hardware = data.get("hardware")
    if isinstance(hardware, dict):
        for key in ("ram_mb", "storage_gb"):
            value = hardware.get(key)
            if type(value) is not int or value < 1:
                errors.append(f"hardware.{key} must be a positive integer")

        for key in ("soc", "cpu", "display", "storage_type"):
            if not isinstance(hardware.get(key), str) or not hardware[key].strip():
                errors.append(f"hardware.{key} must be a non-empty string")

    features = data.get("features")
    if isinstance(features, dict):
        for key in ("dual_sim", "fingerprint", "wifi", "bluetooth", "gnss", "usb_otg"):
            if type(features.get(key)) is not bool:
                errors.append(f"features.{key} must be a boolean")

    compatibility = data.get("compatibility")
    if isinstance(compatibility, dict):
        if compatibility.get("development_host") != "PC/Linux/VM":
            errors.append(
                "compatibility.development_host must be 'PC/Linux/VM' "
                "(development only; deployment remains mobile-only)"
            )

    return errors


def main() -> int:
    try:
        data = tomllib.loads(PROFILE.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as exc:
        print(f"ERROR: cannot parse {PROFILE.relative_to(ROOT)}: {exc}", file=sys.stderr)
        return 1

    errors = validate_profile(data)
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
