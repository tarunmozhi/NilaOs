"""Tests for the read-only Qualcomm FDT/DT table inspector."""
import importlib.util
from pathlib import Path
import struct
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "inspect-qcom-dtb.py"
SPEC = importlib.util.spec_from_file_location("inspect_qcom_dtb", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("could not load Qualcomm DT inspector")
inspector = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(inspector)


def _pad4(value: bytes) -> bytes:
    return value + b"\0" * ((-len(value)) & 3)


def make_fdt(root_properties: dict[str, bytes], child_properties: dict[str, bytes] | None = None) -> bytes:
    strings = bytearray()
    offsets = {}
    for name in (*root_properties.keys(), *((child_properties or {}).keys())):
        if name not in offsets:
            offsets[name] = len(strings)
            strings.extend(name.encode("ascii") + b"\0")

    structure = bytearray()
    structure.extend(struct.pack(">I", inspector.FDT_BEGIN_NODE))
    structure.extend(_pad4(b"\0"))
    for name, value in root_properties.items():
        structure.extend(struct.pack(">III", inspector.FDT_PROP, len(value), offsets[name]))
        structure.extend(_pad4(value))
    if child_properties:
        structure.extend(struct.pack(">I", inspector.FDT_BEGIN_NODE))
        structure.extend(_pad4(b"child\0"))
        for name, value in child_properties.items():
            structure.extend(struct.pack(">III", inspector.FDT_PROP, len(value), offsets[name]))
            structure.extend(_pad4(value))
        structure.extend(struct.pack(">I", inspector.FDT_END_NODE))
    structure.extend(struct.pack(">II", inspector.FDT_END_NODE, inspector.FDT_END))

    off_reserve = 40
    off_struct = off_reserve + 16
    off_strings = off_struct + len(structure)
    total_size = off_strings + len(strings)
    header = struct.pack(
        ">10I",
        inspector.FDT_MAGIC,
        total_size,
        off_struct,
        off_strings,
        off_reserve,
        17,
        16,
        0,
        len(strings),
        len(structure),
    )
    return header + b"\0" * 16 + structure + bytes(strings)


def make_dt_table(fdt: bytes) -> bytes:
    header_size = 32
    entry_size = 32
    entry_offset = header_size
    fdt_offset = header_size + entry_size
    total_size = fdt_offset + len(fdt)
    header = struct.pack(
        ">8I",
        inspector.DT_TABLE_MAGIC,
        total_size,
        header_size,
        entry_size,
        1,
        entry_offset,
        2048,
        0,
    )
    entry = struct.pack(">8I", len(fdt), fdt_offset, 0x18, 0x2, 0xA, 0, 0, 0)
    return header + entry + fdt


class QualcommDtInspectorTests(unittest.TestCase):
    def setUp(self):
        self.properties = {
            "qcom,msm-id": struct.pack(">2I", 353, 0),
            "qcom,board-id": struct.pack(">2I", 8, 24),
            "qcom,pmic-id": struct.pack(">4I", 0x010016, 0x25, 0, 0),
        }

    def test_reads_ids_from_raw_fdt_root_node(self):
        fdt = make_fdt(
            self.properties,
            {"qcom,board-id": struct.pack(">2I", 999, 999)},
        )
        parsed = inspector.parse_fdt(fdt)
        self.assertEqual(parsed, self.properties)

    def test_reads_table_metadata_and_ids_from_android_dtbo_v0(self):
        entries = inspector.inspect_image(make_dt_table(make_fdt(self.properties)))
        self.assertEqual(len(entries), 1)
        self.assertEqual(entries[0]["index"], 0)
        self.assertEqual(entries[0]["id"], 0x18)
        self.assertEqual(entries[0]["revision"], 0x2)
        self.assertEqual(entries[0]["custom"], [0xA, 0, 0, 0])
        self.assertEqual(entries[0]["properties"], self.properties)

    def test_rejects_truncated_fdt_and_invalid_table_ranges(self):
        fdt = make_fdt(self.properties)
        with self.assertRaises(inspector.ImageError):
            inspector.parse_fdt(fdt[:-1])

        table = bytearray(make_dt_table(fdt))
        struct.pack_into(">I", table, 36, 0xFFFFFFF0)
        with self.assertRaises(inspector.ImageError):
            inspector.parse_dt_table(bytes(table))

        empty_table = bytearray(make_dt_table(fdt))
        struct.pack_into(">I", empty_table, 16, 0)
        with self.assertRaises(inspector.ImageError):
            inspector.parse_dt_table(bytes(empty_table))

    def test_rejects_non_ascii_property_names(self):
        fdt = bytearray(make_fdt(self.properties))
        strings_offset = struct.unpack_from(">I", fdt, 12)[0]
        fdt[strings_offset] = 0xFF
        with self.assertRaises(inspector.ImageError):
            inspector.parse_fdt(bytes(fdt))

    def test_rejects_unknown_image_format(self):
        with self.assertRaises(inspector.ImageError):
            inspector.inspect_image(b"not a device tree")


if __name__ == "__main__":
    unittest.main()
