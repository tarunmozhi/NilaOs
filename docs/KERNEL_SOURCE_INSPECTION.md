# Inspecting a candidate kernel source tree

Run this read-only helper from the repository root in Linux or WSL:

```bash
bash scripts/inspect-kernel-source.sh /absolute/path/to/linux-source
```

It checks that the directory resembles a Linux ARM64 source tree, reports its
version and available ARM64 defconfigs, and searches a bounded set of source
directories for references to SDM439, PD1930CF, PD1930BF, PD1930F, vivo 1906, or 1906. The search is
only a discovery aid: vendor trees can use different names, and a text match
does not establish that the tree supports this exact phone.

## Read stock DTB/DTBO identifiers

Use the read-only inspector with a raw compiled FDT blob or an Android DT table
version 0 image:

```bash
python3 scripts/inspect-qcom-dtb.py /path/to/stock/dtbo.img
python3 scripts/inspect-qcom-dtb.py /path/to/extracted/device-tree.dtb
```

It reports the Android table entry `id`, `rev`, and `custom[]` fields, when
present, plus Qualcomm root-node `qcom,msm-id`, `qcom,board-id`, and
`qcom,pmic-id` properties. These are distinct identifiers: the tool does not
treat a table entry ID as a Qualcomm board ID, and it does not choose an
overlay or claim that an image matches the phone.

The tool accepts raw DTB/DTBO FDT blobs and Android DT table version 0 images.
Extract a DTB/DTBO from a boot container before inspection; it does not parse
`boot.img`, `vendor_boot.img`, compressed kernel images, or vendor-specific
containers. It reads files only and does not alter or flash them. A binary match
is evidence to compare against actual device metadata, not proof of bootability.
See the [AOSP DTB and DTBO partition format](https://source.android.com/docs/core/architecture/dto/partitions).

The source-tree helper does not fetch or modify source, select a defconfig, or
build or flash anything. Before using `scripts/build-kernel.sh`, independently
verify source provenance, exact defconfig, device tree and overlays, kernel
modules, firmware, boot-image packaging, AVB/signing, partition layout, and a
tested recovery path. Do not flash a candidate merely because the inspection
script finds a match. A successful kernel compile is not proof of bootability.
