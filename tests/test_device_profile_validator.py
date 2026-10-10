"""Regression tests for the vivo 1906 profile validator."""
from copy import deepcopy
from pathlib import Path
import sys
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import validate_device_profile as validator  # noqa: E402


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
            any("hardware.ram_mb must be a positive integer" == error
                for error in validator.validate_profile(data))
        )

    def test_rejects_missing_feature_flags(self):
        data = deepcopy(self.profile)
        del data["features"]["wifi"]
        self.assertTrue(
            any("features.wifi must be a boolean" == error
                for error in validator.validate_profile(data))
        )

    def test_deployment_must_remain_mobile_only(self):
        data = deepcopy(self.profile)
        data["compatibility"]["deployment"] = "desktop"
        self.assertTrue(
            any("compatibility.deployment" in error
                for error in validator.validate_profile(data))
        )


if __name__ == "__main__":
    unittest.main()
