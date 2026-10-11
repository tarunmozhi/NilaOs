#!/usr/bin/env python3
"""Validate the structure and safety metadata of a staged Nila ARM64 rootfs.

This is a structural check only. It cannot prove kernel/device compatibility,
bootability, security enforcement, or readiness to flash a phone.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REQUIRED_DIRECTORIES = (
    "bin", "dev", "etc", "proc", "run", "sys", "tmp", "usr", "var",
)
REQUIRED_FILES = (
    "etc/os-release",
    "etc/hostname",
    "bin/sh",
    "etc/nila/rootfs-manifest.json",
)


class ValidationError(Exception):
    """Raised when the staged rootfs fails a validation requirement."""


def validate_rootfs(root: Path) -> list[str]:
    if root.is_symlink():
        raise ValidationError("rootfs path must not itself be a symbolic link")
    if not root.exists() or not root.is_dir():
        raise ValidationError(f"rootfs directory does not exist: {root}")

    root = root.resolve()
    for relative in REQUIRED_DIRECTORIES:
        item = root / relative
        if not item.is_dir():
            raise ValidationError(f"required directory is missing: /{relative}")

    for relative in REQUIRED_FILES:
        item = root / relative
        if not item.is_file():
            raise ValidationError(f"required file is missing: /{relative}")

    shell = root / "bin/sh"
    if not (shell.stat().st_mode & 0o111):
        raise ValidationError("/bin/sh does not have an executable permission bit")

    manifest_path = root / "etc/nila/rootfs-manifest.json"
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise ValidationError(f"cannot read rootfs manifest: {error}") from error

    if not isinstance(manifest, dict):
        raise ValidationError("rootfs manifest must be a JSON object")

    expected = {
        "schema_version": 1,
        "distribution": "Debian",
        "suite": "bookworm",
        "architecture": "arm64",
        "purpose": "development-rootfs-only",
        "device_ready": False,
        "kernel_included": False,
        "boot_image_created": False,
    }
    for key, value in expected.items():
        if manifest.get(key) != value or (value is False and manifest.get(key) is not False):
            raise ValidationError(
                f"rootfs manifest field {key!r} must be {value!r}"
            )

    return [
        "required directory layout is present",
        "required OS metadata and shell are present",
        "ARM64 development-only manifest is valid",
        "manifest explicitly marks this rootfs as NOT device-ready",
    ]


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Validate a staged generic Debian Bookworm ARM64 rootfs."
    )
    parser.add_argument("rootfs", type=Path, help="path to the staged root filesystem")
    args = parser.parse_args()

    try:
        checks = validate_rootfs(args.rootfs)
    except (ValidationError, OSError) as error:
        print(f"ROOTFS VALIDATION FAILED: {error}", file=sys.stderr)
        return 1

    print(f"Rootfs validation passed: {args.rootfs}")
    for check in checks:
        print(f"  OK: {check}")
    print("This is not a kernel, boot-image, hardware, or flash-readiness test.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
