# Staged root filesystem validation

The experimental rootfs bootstrap creates a generic Debian Bookworm ARM64 development userspace. Run the structural validator after the bootstrap completes:

```bash
python3 scripts/validate-rootfs.py /absolute/path/to/rootfs
```

The validator checks required directories, basic OS metadata, an executable shell, and the generated `/etc/nila/rootfs-manifest.json`. The manifest deliberately records that the output is **not device-ready**, contains no kernel, and has no boot image.

The validator is a structural smoke test only. It does not prove that the rootfs can boot, that the target vivo 1906 kernel or drivers match, that security is enforced, or that an image can safely be flashed. It must never be treated as a release or flash approval.
