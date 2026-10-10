"""Regression tests for the vivo 1906 profile validator."""
from copy import deepcopy
import importlib.util
from pathlib import Path
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[1]
VALIDATOR_PATH = ROOT / "scripts" / "validate-device-profile.py"
SPEC = importlib.util.spec_from_file_location("validate_device_profile", VALIDATOR_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("could not load device profile validator")
validator = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(validator)


class DeviceProfileValidatorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = tomllib.loads(validator.PROFILE.read_text(encoding="utf-8"))

    def test_repository_profile_is_valid(self):
        self.assertEqual(validator.validate_profile(self.profile), [])

    def test_rejects_non_table_root(self):
        self.assertIn("profile root must be a TOML table", validator.validate_profile([]))

    def test_rejects_non_table_sections_without_crashing(self):
        data = deepcopy(self.profile)
        data["hardware"] = "not a table"
        errors = validator.validate_profile(data)
        self.assertTrue(any(error.startswith("hardware must be a TOML table") for error in errors))

    def test_rejects_wrong_device_model(self):
        data = deepcopy(self.profile)
        data["device"]["model"] = "UNKNOWN"
        self.assertTrue(
            any("device.model" in error for error in validator.validate_profile(data))
        )

    def test_rejects_boolean_ram_value(self):
        data = deepcopy(self.profile)
        data["hardware"]["ram_mb"] = True
        self.assertTrue(
            any(
                error == "hardware.ram_mb must be a positive integer"
                for error in validator.validate_profile(data)
            )
        )

    def test_rejects_missing_feature_flags(self):
        data = deepcopy(self.profile)
        del data["features"]["wifi"]
        self.assertTrue(
            any(
                error == "features.wifi must be a boolean"
                for error in validator.validate_profile(data)
            )
        )

    def test_deployment_must_remain_mobile_only(self):
        data = deepcopy(self.profile)
        data["compatibility"]["deployment"] = "desktop"
        self.assertTrue(
            any("compatibility.deployment" in error for error in validator.validate_profile(data))
        )

    def test_requires_confirmed_android_reference(self):
        data = deepcopy(self.profile)
        data["software"]["android_reference"] = "Android 9 / Funtouch OS 9.1"
        self.assertTrue(
            any("software.android_reference" in error for error in validator.validate_profile(data))
        )

    def test_requires_confirmed_build_and_kernel(self):
        data = deepcopy(self.profile)
        data["software"]["build"] = "unknown"
        data["software"]["kernel"] = ""
        errors = validator.validate_profile(data)
        self.assertTrue(any("software.build" in error for error in errors))
        self.assertTrue(any("software.kernel" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
