#!/usr/bin/env python3
"""Read-only inspection of Qualcomm IDs in raw FDT and Android DTBO v0 images.

This reports metadata found in an image. It does not identify the currently
installed phone, select a safe overlay, or validate boot compatibility.
"""
from __future__ import annotations

import argparse
import struct
import sys
from pathlib import Path
from typing import Any

FDT_MAGIC = 0xD00DFEED
DT_TABLE_MAGIC = 0xD7B7AB1E
FDT_BEGIN_NODE = 1
FDT_END_NODE = 2
FDT_PROP = 3
FDT_NOP = 4
FDT_END = 9
MAX_IMAGE_SIZE = 256 * 1024 * 1024
QCOM_PROPERTIES = ("qcom,msm-id", "qcom,board-id", "qcom,pmic-id")


class ImageError(ValueError):
    """Raised when an FDT or Android DT table image is malformed."""


def _need(data: bytes, offset: int, length: int, label: str) -> None:
    if offset < 0 or length < 0 or offset > len(data) or length > len(data) - offset:
        raise ImageError(f"{label} is outside the image")


def _align4(value: int) -> int:
    return (value + 3) & ~3


def _read_c_string(data: bytes, offset: int, limit: int, label: str) -> tuple[bytes, int]:
    if offset < 0 or offset >= limit or limit > len(data):
        raise ImageError(f"{label} starts outside its section")
    end = data.find(b"\0", offset, limit)
    if end < 0:
        raise ImageError(f"{label} is not NUL-terminated")
    return data[offset:end], end + 1


def parse_fdt(blob: bytes) -> dict[str, bytes]:
    """Parse root-node properties from a standalone Flattened Device Tree."""
    if len(blob) < 36:
        raise ImageError("FDT is shorter than its header")
    if struct.unpack_from(">I", blob, 0)[0] != FDT_MAGIC:
        raise ImageError("input does not start with an FDT magic value")

    magic, total_size, off_struct, off_strings, off_reserve, version, last_compatible, boot_cpu, size_strings = struct.unpack_from(
        ">9I", blob, 0
    )
    del magic, last_compatible, boot_cpu
    if total_size < 36 or total_size > len(blob):
        raise ImageError("FDT total size is invalid")
    if version < 16:
        raise ImageError(f"unsupported FDT version {version}; version 16 or newer is required")

    if version >= 17:
        if len(blob) < 40:
            raise ImageError("FDT version 17 header is truncated")
        size_struct = struct.unpack_from(">I", blob, 36)[0]
    else:
        if off_strings <= off_struct:
            raise ImageError("cannot infer the version 16 FDT structure size")
        size_struct = off_strings - off_struct

    if off_struct < 36 or off_strings < 36 or off_reserve < 36:
        raise ImageError("FDT section offset overlaps the header")
    if off_struct > total_size or size_struct > total_size - off_struct:
        raise ImageError("FDT structure section is outside total_size")
    if off_strings > total_size or size_strings > total_size - off_strings:
        raise ImageError("FDT strings section is outside total_size")
    if off_reserve > total_size - 16:
        raise ImageError("FDT reserve map is outside total_size")

    image = blob[:total_size]
    structure_end = off_struct + size_struct
    strings_end = off_strings + size_strings
    cursor = off_struct
    depth = 0
    root_seen = False
    end_seen = False
    properties: dict[str, bytes] = {}

    while cursor < structure_end:
        _need(image, cursor, 4, "FDT token")
        token = struct.unpack_from(">I", image, cursor)[0]
        cursor += 4

        if token == FDT_BEGIN_NODE:
            _, cursor = _read_c_string(image, cursor, structure_end, "FDT node name")
            cursor = _align4(cursor)
            if cursor > structure_end:
                raise ImageError("FDT node name exceeds the structure section")
            depth += 1
            if depth > 64:
                raise ImageError("FDT nesting exceeds the safety limit")
            if depth == 1:
                if root_seen:
                    raise ImageError("FDT contains multiple root nodes")
                root_seen = True
        elif token == FDT_END_NODE:
            if depth == 0:
                raise ImageError("FDT node nesting is unbalanced")
            depth -= 1
        elif token == FDT_PROP:
            if depth == 0:
                raise ImageError("FDT property appears outside a node")
            _need(image, cursor, 8, "FDT property header")
            length, name_offset = struct.unpack_from(">II", image, cursor)
            cursor += 8
            _need(image, cursor, length, "FDT property value")
            value = image[cursor:cursor + length]
            cursor = _align4(cursor + length)
            if cursor > structure_end:
                raise ImageError("FDT property exceeds the structure section")
            if name_offset >= size_strings:
                raise ImageError("FDT property name offset is invalid")
            raw_name, _ = _read_c_string(image, off_strings + name_offset, strings_end, "FDT property name")
            name = raw_name.decode("utf-8", errors="strict")
            if depth == 1 and name in QCOM_PROPERTIES:
                properties[name] = value
        elif token == FDT_NOP:
            continue
        elif token == FDT_END:
            if depth != 0 or not root_seen:
                raise ImageError("FDT ended before its root node was closed")
            end_seen = True
            break
        else:
            raise ImageError(f"unknown FDT structure token {token}")

    if not end_seen:
        raise ImageError("FDT structure has no FDT_END token")
    return properties


def parse_dt_table(blob: bytes) -> list[dict[str, Any]]:
    """Parse Android DT table version 0 and its contained FDT entries."""
    if len(blob) < 32:
        raise ImageError("Android DT table header is truncated")
    values = struct.unpack_from(">8I", blob, 0)
    magic, total_size, header_size, entry_size, entry_count, entries_offset, page_size, version = values
    del page_size
    if magic != DT_TABLE_MAGIC:
        raise ImageError("input does not have the Android DT table magic value")
    if version != 0:
        raise ImageError(f"unsupported Android DT table version {version}; version 0 is supported")
    if header_size != 32 or entry_size < 32:
        raise ImageError("Android DT table header or entry size is invalid")
    if total_size < header_size or total_size > len(blob):
        raise ImageError("Android DT table total_size is invalid")
    if entries_offset < header_size or entries_offset > total_size:
        raise ImageError("Android DT entry offset is invalid")
    if entry_count > (total_size - entries_offset) // entry_size:
        raise ImageError("Android DT entry array exceeds total_size")

    entries = []
    data = blob[:total_size]
    for index in range(entry_count):
        entry_offset = entries_offset + index * entry_size
        _need(data, entry_offset, 32, f"DT table entry {index}")
        dt_size, dt_offset, entry_id, revision, *custom = struct.unpack_from(">8I", data, entry_offset)
        entry_array_end = entries_offset + entry_count * entry_size
        if dt_size == 0 or dt_offset < entry_array_end or dt_offset > total_size or dt_size > total_size - dt_offset:
            raise ImageError(f"DT table entry {index} has an invalid FDT range")
        fdt = data[dt_offset:dt_offset + dt_size]
        entries.append({
            "index": index,
            "id": entry_id,
            "revision": revision,
            "custom": custom,
            "properties": parse_fdt(fdt),
        })
    return entries


def inspect_image(blob: bytes) -> list[dict[str, Any]]:
    if len(blob) > MAX_IMAGE_SIZE:
        raise ImageError(f"image exceeds the {MAX_IMAGE_SIZE}-byte safety limit")
    if len(blob) < 4:
        raise ImageError("input is too short to contain an FDT or DT table")
    magic = struct.unpack_from(">I", blob, 0)[0]
    if magic == FDT_MAGIC:
        return [{"index": None, "id": None, "revision": None, "custom": [], "properties": parse_fdt(blob)}]
    if magic == DT_TABLE_MAGIC:
        return parse_dt_table(blob)
    raise ImageError("input is neither a standalone FDT nor an Android DT table image")


def _format_value(value: bytes) -> str:
    if len(value) % 4 == 0:
        cells = struct.unpack(f">{len(value) // 4}I", value)
        return " ".join(f"{cell} (0x{cell:08x})" for cell in cells)
    return value.hex()


def render_report(path: Path, entries: list[dict[str, Any]]) -> str:
    lines = [f"Image: {path}"]
    for entry in entries:
        if entry["index"] is not None:
            lines.extend((
                f"DT table entry {entry['index']}:",
                f"  id: 0x{entry['id']:08x}",
                f"  rev: 0x{entry['revision']:08x}",
                "  custom: " + " ".join(f"0x{value:08x}" for value in entry["custom"]),
            ))
        properties = entry["properties"]
        for name in QCOM_PROPERTIES:
            value = properties.get(name)
            lines.append(f"  {name}: {_format_value(value) if value is not None else 'absent'}")
    lines.append("Image metadata only; this does not prove which entry is used by the phone.")
    lines.append("Do not select a kernel overlay from these values without matching stock-device metadata.")
    return "\n".join(lines)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Read Qualcomm IDs from a raw FDT or Android DT table version 0 image."
    )
    parser.add_argument("image", type=Path, help="raw .dtb/.dtbo or Android dtb.img/dtbo.img")
    args = parser.parse_args(argv)
    try:
        size = args.image.stat().st_size
        if size > MAX_IMAGE_SIZE:
            raise ImageError(f"image exceeds the {MAX_IMAGE_SIZE}-byte safety limit")
        blob = args.image.read_bytes()
        entries = inspect_image(blob)
    except (OSError, ImageError) as error:
        print(f"DT inspection failed: {error}", file=sys.stderr)
        return 1

    print(render_report(args.image, entries))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
