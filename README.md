# sdbootutil

sdbootutil manages the boot loader of openSUSE systems that use the
Boot Loader Specification: **systemd-boot** or **grub2-bls**.  It is
made for systems with btrfs and snapper, but it also works without
snapshots.

Can also manage the operations required for full disk encryption
(FDE), like enrolling using a TPM2, a recovery password or a FIDO2
key.

It is mostly called by other tools: the kernel packages, snapper,
transactional-update and the installer.  You can also call it
yourself to do the enrollment or to diagnose issues with FDE.

## What it does

- **Installs and updates the boot loader** in the EFI System Partition
  (ESP), with shim when Secure Boot is used.
- **Adds a boot entry for each kernel and each snapshot.**  You can
  boot an older snapshot from the boot menu.
- **Saves space in the ESP.**  Kernels and initrds are named by their
  checksum, so snapshots that use the same file share one copy.
- **Keeps free space in the ESP.**  Old entries are removed when a new
  kernel does not fit.
- **Sets up full disk encryption (FDE)** for the LUKS2 devices in
  `/etc/crypttab`.  A device can be opened with:
  - the TPM2, with or without a PIN,
  - a FIDO2 key,
  - a password,
  - a recovery key.
- **Keeps the TPM2 working after updates.**  When the kernel, the
  initrd or the boot loader changes, it predicts the new TPM2
  measurements (PCRs) with `systemd-pcrlock`, so the disk still opens
  without a password.
- **Opens the disks only during boot.**  The TPM2 slots are bound to
  PCR 15, which changes when the system leaves the initrd.  After that
  point the TPM2 cannot open the disks again.
- **Works with firmware updates.**  fwupd can tell `systemd-pcrlock`
  to ignore the firmware measurements before an update, and sdbootutil
  publishes that policy and measures the firmware again after the
  reboot.

## Examples

See the boot status, and if the TPM2 will open the disk:

    sdbootutil status

Install systemd-boot with shim:

    sdbootutil install --secure-boot

Update the boot loader when the package has a new version:

    sdbootutil update

Add the entries of all kernels of the current snapshot:

    sdbootutil add-all-kernels

List the boot entries:

    sdbootutil list-entries

Enroll the TPM2 for all encrypted devices, and choose the recovery PIN
that you will need if the TPM2 stops opening the disks:

    sdbootutil enroll --method tpm2 --ask-recovery-pin

Update the TPM2 predictions by hand:

    sdbootutil update-predictions

## Troubleshooting

If the system asks for the password, the TPM2 measurements of this boot
did not match the policy.  Look at the status:

    sdbootutil status

To make a new policy for this boot, use your recovery PIN, and reboot:

    sdbootutil --ask-recovery-pin update-predictions

The section TROUBLESHOOTING of `man 8 sdbootutil` explains the usual
reasons, how to enroll again, and how to collect logs for a bug report.
Report bugs at https://bugzilla.opensuse.org or
https://github.com/openSUSE/sdbootutil/issues.

## More information

- `man 8 sdbootutil`: all the commands, options and files.
- `sdbootutil --help`: the short version.
- `ARCHITECTURE.md`: how boot entries and snapshots fit together.
