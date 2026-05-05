# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`sdbootutil` is a bootctl wrapper for BLS (Boot Loader Specification) boot loaders (systemd-boot and grub2-bls) designed for btrfs-based, snapper-managed systems. It manages the full lifecycle of bootloader installations with Full Disk Encryption (FDE) support based on systemd.

**Key Capabilities:**
- Installs and updates systemd-boot with shim for secure boot
- Manages kernel entries in ESP (EFI System Partition) with snapshot awareness
- Handles btrfs snapshots via snapper integration
- Implements full disk encryption enrollment (TPM2, FIDO2, password)
- Uses checksums for kernel/initrd deduplication across snapshots
- Manages PCR (Platform Configuration Register) predictions for TPM2

## Architecture

### Core Components

1. **`sdbootutil` (main bash script)** - 4400+ lines
   - Primary interface for all bootloader operations
   - Handles kernel/entry management, snapshot integration, FDE enrollment
   - Entry point for snapper hooks and RPM triggers

2. **`uhmac/` (Rust utility)** - HMAC calculation utility
   - Used for cryptographic operations in FDE scenarios
   - Built with cargo, linked as `%{_libexecdir}/sdbootutil/uhmac`

3. **Integration Scripts:**
   - `10-sdbootutil.snapper` - Snapper plugin for snapshot lifecycle hooks
   - `kernelhooks.lua` - RPM file trigger for kernel package install/remove
   - `10-sdbootutil.tukit` - Tukit plugin for transactional systems
   - `50-sdbootutil.install` - kernel-install plugin script

4. **FDE/TPM Components:**
   - `measure-pcr-generator.sh` - Generates PCR 15 predictions
   - `measure-pcr-validator.sh` - Validates PCR measurements at boot
   - `sdbootutil-enroll` - Enrollment service wrapper
   - `jeos-firstboot-enroll` - JEOS integration for first-boot enrollment

### How Snapshots Work

Unlike standard systemd-boot which assumes one OS instance per kernel version, sdbootutil extends this for btrfs snapshots where multiple subvolumes share kernels:

- **Entry naming:** `{machine-id}-{version}-{snapshot}.conf` (e.g., `2ceda9f-6.2.1-1-default-15.conf`)
- **Kernel/initrd deduplication:** Uses checksums instead of snapshot numbers in filenames
  - Example: `linux-b021b508eb42b2afd06de8f0242b9727aa7dc494`
  - Allows multiple snapshots to share same kernel/initrd files
- **Reference counting:** Uses `bootctl unlink` and `bootctl cleanup` to safely remove entries
- **Initrd reuse:** Intelligently reuses initrds from parent snapshots when possible

### System Integration Points

**Snapper Hooks** (`10-sdbootutil.snapper`):
- `create-snapshot-post`: Updates bootloader, adds entries (Tumbleweed) or defers to set-default (transactional)
- `delete-snapshot-pre`: Removes entries for deleted snapshot
- `set-default-snapshot-post`: Sets bootloader default, adds entries (transactional systems only)

**RPM Triggers** (`kernelhooks.lua`):
- Monitors `/usr/lib/modules/{version}/vmlinuz` installations
- Filters out legacy `/boot/vmlinuz-*` locations
- Calls `sdbootutil add-kernel` / `remove-kernel` automatically
- Note: File triggers can be unreliable with zypper

**Transactional vs Non-Transactional:**
- **Transactional (MicroOS):** Kernel entries added in `set-default-snapshot-post` after transaction completes
- **Non-Transactional (Tumbleweed):** Kernel entries added immediately in `create-snapshot-post`

## Common Commands

### Build

```bash
# Build uhmac utility (Rust)
cd uhmac
cargo build --release

# For RPM build, see sdbootutil.spec (%build and %install sections)
```

### Testing sdbootutil

```bash
# Enable trace mode (outputs to /var/log/sdbootutil.log)
sudo ./sdbootutil --start-trace-code <command>
sudo ./sdbootutil --stop-trace-code

# Verbose output
sudo ./sdbootutil -v <command>

# Check bootloader status
sudo ./sdbootutil is-installed
sudo ./sdbootutil bootloader

# List entries/kernels for snapshot
sudo ./sdbootutil list-entries [snapshot]
sudo ./sdbootutil list-kernels [snapshot]
sudo ./sdbootutil list-snapshots

# Test kernel entry management
sudo ./sdbootutil add-kernel <version> [snapshot]
sudo ./sdbootutil remove-kernel <version> [snapshot]
sudo ./sdbootutil cleanup [snapshot]
```

### Full Disk Encryption Operations

```bash
# Enroll TPM2 with PIN
sudo ./sdbootutil enroll --method tpm2+pin

# Enroll FIDO2 key
sudo ./sdbootutil enroll --method fido2

# Update PCR predictions after kernel/bootloader changes
sudo ./sdbootutil update-predictions

# List tracked encrypted devices
sudo ./sdbootutil list-devices
```

### Bootloader Management

```bash
# Install bootloader with shim (secure boot)
sudo ./sdbootutil install --secure-boot

# Check if bootloader needs update
sudo ./sdbootutil needs-update

# Update bootloader (only if newer version available)
sudo ./sdbootutil update

# Force update bootloader to match system version
sudo ./sdbootutil update --sync
```

## Configuration

### Config File Hierarchy
1. `/etc/sdbootutil.conf` - User configuration (loaded if exists)
2. Auto-generated from defaults if bootloader installed but no config exists
3. CLI arguments override config file values

**Key Config Variables:**
- `ENTRY_TOKEN` - Entry identifier (default: machine-id)
- `ESP_PATH` - ESP mount point
- `BOOTLOADER_TYPE` - "systemd-boot" or "grub2-bls"
- `UPDATE_NVRAM` - Whether to update EFI variables
- `REUSE_INITRD` - Whether to reuse initrds from parent snapshots

### ESP Directory Structure

```
ESP/
├── {entry-token}/              # e.g., 2ceda9f/
│   └── {kernel-version}/       # e.g., 6.2.1-1-default/
│       ├── linux-{checksum}    # Deduplicated kernel
│       └── initrd-{checksum}   # Deduplicated initrd
├── EFI/
│   ├── BOOT/
│   │   └── BOOTX64.EFI         # Shim for removable media
│   └── systemd/
│       ├── systemd-bootx64.efi
│       ├── shim.efi
│       └── grub.efi            # Actually systemd-boot when using shim
└── loader/
    ├── entries/
    │   └── {entry-token}-{version}-{snapshot}.conf
    └── loader.conf             # Default entry, timeout
```

## Important Implementation Details

### Entry Configuration Format

Entries include snapshot-specific `rootflags=subvol=` parameter:

```
title      openSUSE Tumbleweed
version    15@6.2.1-1-default
machine-id 2ceda9f
sort-key   opensuse-tumbleweed
options    root=UUID=... rootflags=subvol=@/.snapshots/15/snapshot
linux      /2ceda9f/6.2.1-1-default/linux-{checksum}
initrd     /2ceda9f/6.2.1-1-default/initrd-{checksum}
```

### Checksum-Based Deduplication

Function `install_kernel()` uses SHA-256 checksums to:
1. Check if identical kernel/initrd already exists in ESP
2. Reuse existing files instead of copying duplicates
3. Maintain reference counts for safe cleanup

### Initrd Reuse Logic

`reuse_initrd()` function (lines 901+):
- Compares kernel versions between snapshots
- If kernel version matches, reuses parent snapshot's initrd
- Checks both vmlinuz and modules to ensure compatibility
- Can be disabled with `--no-reuse-initrd`

### PCR Prediction System

Full disk encryption relies on PCR (Platform Configuration Register) predictions:

1. **Generation** (`measure-pcr-generator.sh`):
   - Calculates expected PCR 15 values for initrd measurements
   - Signs predictions with private key
   - Stores in `/var/lib/sdbootutil/measure-pcr-prediction`

2. **Validation** (`measure-pcr-validator.sh`, `measure-pcr-validator.service`):
   - Runs at boot to verify PCR 15 matches prediction
   - Checks signature validity
   - Prevents boot if validation fails (unless `measure-pcr-validator.ignore` set)

### State File for Transactional Systems

`/var/lib/misc/transactional-update.state` stores state across reboots for read-only root systems.

### Debug Tracing

When `--start-trace-code` is used:
- Creates `/var/log/sdbootutil.log`
- Enables `set -x` with BASH_XTRACEFD=3
- Custom PS4 with timestamps and source locations
- WARNING: May contain secrets (encryption keys, passwords)

## Common Development Patterns

### Adding New Commands

1. Add command handler function (e.g., `my_command()`)
2. Add to help text in `helpandquit()`
3. Add case in main command switch (line 4353+)
4. Update bash completion in `completions/bash_sdbootutil`

### Working with Snapshots

Always use `${snapshot:-$root_snapshot}` pattern for optional snapshot argument, where `$root_snapshot` is the current/default snapshot.

### Error Handling

- Use `err()` for fatal errors (exits with status 1)
- Use `warn()` for non-fatal warnings
- Use `info()` for user-facing messages
- Use `dbg()` / `dbg_var()` / `dbg_cat()` for debug output (only when verbose mode enabled)

### Rollback Support

Functions can use `install_with_rollback()` to backup files before modification. On cleanup (error or exit), `rollback[@]` array is processed to restore backups.

## Dependencies

**Runtime:**
- systemd (for bootctl, systemd-pcrlock)
- snapper (for snapshot integration)
- dracut (for initrd generation)
- btrfs-progs (for btrfs operations)
- tpm2-tools, keyutils (for FDE)
- efibootmgr (for NVRAM management)
- jq, sed, openssl, qrencode (utilities)

**Build:**
- cargo, cargo-packaging (for uhmac)
- libopenssl-devel (for uhmac)
- systemd-rpm-macros (for packaging)

## Testing Considerations

- Changes to bootloader logic should be tested in VMs with btrfs + snapper
- FDE enrollment requires TPM2 device or FIDO2 hardware
- Snapshot operations require active snapper configuration
- Secure boot testing requires shim + signed binaries in `/usr/share/efi/$(uname -m)`
- Always test both transactional and non-transactional paths

## Related Documentation

- **ARCHITECTURE.md** - Detailed explanation of bootloader spec implementation with snapshots
- **sdbootutil.spec** - RPM packaging, subpackages, dependencies
- See upstream: [Boot Loader Specification](https://uapi-group.org/specifications/specs/boot_loader_specification/)
