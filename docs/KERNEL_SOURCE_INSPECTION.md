# Inspecting a candidate kernel source tree

Run this read-only helper from the repository root in Linux or WSL:

```bash
bash scripts/inspect-kernel-source.sh /absolute/path/to/linux-source
```

It checks that the directory resembles a Linux ARM64 source tree, reports its
version and available ARM64 defconfigs, and searches a bounded set of source
directories for references to SDM439, PD1930F, vivo 1906, or 1906. The search is
only a discovery aid: vendor trees can use different names, and a text match
does not establish that the tree supports this exact phone.

The helper does not fetch or modify source, select a defconfig, or build or flash
anything. Before using `scripts/build-kernel.sh`, independently verify the
source provenance, exact defconfig, device tree and overlays, kernel modules,
firmware, boot image packaging, AVB/signing, partition layout, and a tested
recovery path. Do not flash a candidate merely because the inspection script
finds a match. A successful kernel compile is not proof of bootability.
