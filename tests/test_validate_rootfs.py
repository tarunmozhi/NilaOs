import importlib.util
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "validate-rootfs.py"
SPEC = importlib.util.spec_from_file_location("nila_rootfs_validator", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("could not load rootfs validator")
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)
ValidationError = MODULE.ValidationError
validate_rootfs = MODULE.validate_rootfs


class RootfsValidatorTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "rootfs"
        for name in ("bin", "dev", "etc", "proc", "run", "sys", "tmp", "usr", "var"):
            (self.root / name).mkdir(parents=True, exist_ok=True)

        (self.root / "etc/os-release").write_text(
            'PRETTY_NAME="Debian GNU/Linux 12 (bookworm)"\n', encoding="utf-8"
        )
        (self.root / "etc/hostname").write_text("nila\n", encoding="utf-8")
        shell = self.root / "bin/sh"
        shell.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        shell.chmod(0o755)
        manifest_path = self.root / "etc/nila/rootfs-manifest.json"
        manifest_path.parent.mkdir(parents=True)
        manifest_path.write_text(json.dumps({
            "schema_version": 1,
            "distribution": "Debian",
            "suite": "bookworm",
            "architecture": "arm64",
            "purpose": "development-rootfs-only",
            "device_ready": False,
            "kernel_included": False,
            "boot_image_created": False,
        }), encoding="utf-8")

    def test_valid_development_rootfs_passes(self):
        checks = validate_rootfs(self.root)
        self.assertEqual(len(checks), 4)

    def test_missing_required_directory_fails(self):
        (self.root / "proc").rmdir()
        with self.assertRaisesRegex(ValidationError, "required directory"):
            validate_rootfs(self.root)

    def test_missing_os_metadata_fails(self):
        (self.root / "etc/os-release").unlink()
        with self.assertRaisesRegex(ValidationError, "required file"):
            validate_rootfs(self.root)

    def test_non_executable_shell_fails(self):
        (self.root / "bin/sh").chmod(0o644)
        with self.assertRaisesRegex(ValidationError, "executable"):
            validate_rootfs(self.root)

    def test_corrupt_manifest_fails(self):
        (self.root / "etc/nila/rootfs-manifest.json").write_text("{bad", encoding="utf-8")
        with self.assertRaisesRegex(ValidationError, "manifest"):
            validate_rootfs(self.root)

    def test_device_ready_manifest_is_rejected(self):
        path = self.root / "etc/nila/rootfs-manifest.json"
        manifest = json.loads(path.read_text(encoding="utf-8"))
        manifest["device_ready"] = True
        path.write_text(json.dumps(manifest), encoding="utf-8")
        with self.assertRaisesRegex(ValidationError, "device_ready"):
            validate_rootfs(self.root)

    def test_missing_rootfs_fails(self):
        with self.assertRaisesRegex(ValidationError, "does not exist"):
            validate_rootfs(self.root / "missing")

    @unittest.skipUnless(hasattr(os, "symlink"), "symbolic links unavailable")
    def test_symlink_rootfs_is_rejected(self):
        alias = Path(self.temp.name) / "rootfs-alias"
        alias.symlink_to(self.root, target_is_directory=True)
        with self.assertRaisesRegex(ValidationError, "symbolic link"):
            validate_rootfs(alias)


if __name__ == "__main__":
    unittest.main()
