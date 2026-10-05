# Raw disk image openSUSE Full Disk Encryption

This document explains how `disk-encryption-tool`, `jeos-firstboot`,
`sdbootutil`, and (optionally) `combustion` cooperate to deliver Full
Disk Encryption (FDE) with different unlocking mechanisms like
password, recovery key, TPM2 or FIDO2 on openSUSE Tumbleweed and
MicroOS, using `systemd-cryptenroll`, `systemd-pcrlock` and
`cryptsetup`.

---

## 1. Disk enrollment

When a MicroOS or a Tumbleweed image is booted by the first time, the
new image will be encrypted and enrolled in order to enable FDE.
Those two steps are worked by two different components:

1. **Converting the unencrypted root filesystem into LUKS2** - always
   done by `disk-encryption-tool` (specifically
   `disk-encryption-tool-dracut`, run from the first-boot initrd).
   This step always generates one *transient, random* recovery
   passphrase and enrolls it as a throwaway LUKS2 keyslot called the
   **enrollment-key**.
2. **Enrolling the "real" unlock factors** (TPM2, TPM2+PIN, FIDO2,
   password, recovery-key) and wiring up `systemd-pcrlock` boot-chain
   attestation - always done by `sdbootutil enroll`, either:
   - **unattended**, driven by systemd credentials and run once by
     `sdbootutil-enroll.service`, or
   - **interactively**, driven by dialogs in the
     `jeos-firstboot-enroll` module and run as part of
     `jeos-firstboot.service`.

`combustion` (and Ignition, cloud-init, or any other first-boot
provisioning mechanism) is **one way to deliver the systemd
credentials, kernel keys, or environment variables** that select the
unattended path.  `jeos-firstboot` is the human-facing alternative
when no such credentials are present.  `disk-encryption-tool` always
does the LUKS2 conversion; `combustion` and `jeos-firstboot` are two
alternative ways to complete the TPM2/FIDO2 enrollment step
afterwards.  Agama / YaST2 can in the same way deliver the credentials
or kernel keys and call `sdbootutil` to complete the enrollment.

```
                         ┌───────────────────────────────────────────┐
                         │              IMAGE BUILD TIME              │
                         │  kiwi image contains plain-text btrfs root │
                         │  + disk-encryption-tool dracut module      │
                         │  (95disk-encryption-tool, always present)  │
                         └───────────────────────────────────────────┘
                                            │
                                            ▼
        ┌──────────────────────────────────────────────────────────────────┐
        │  FIRST BOOT - initrd stage, all still before switch_root         │
        │                                                                  │
        │  firstboot-detect.service → firstboot.target (only if this       │
        │  really is a first boot: no/uninitialized machine-id, or         │
        │  combustion.firstboot/ignition.firstboot on the cmdline)         │
        │              │                                                   │
        │              ▼                                                   │
        │  combustion-prepare.service  (script "--prepare", waits for the  │
        │     config-drive device, may set up networking)                 │
        │     → e.g. writes /run/credstore/disk-encryption-tool-dracut.encrypt │
        │       = "force" so encryption isn't skipped/aborted              │
        │              │                                                   │
        │              ▼                                                   │
        │  combustion.service  (script normal phase, run via               │
        │     `transactional-update shell` / chroot into the *still        │
        │     plain-text* /sysroot - not yet switch_root'ed)               │
        │     → e.g. writes /etc/credstore.encrypted/sdbootutil-enroll.*   │
        │       (systemd-creds encrypt) into the plain-text filesystem -   │
        │       these files survive the in-place LUKS2 conversion below    │
        │       unchanged, and are read again after the *next* boot        │
        │              │  (After=ignition-complete.target too, if used)    │
        │              ▼                                                   │
        │  disk-encryption-tool-dracut.service  (RequiredBy=firstboot.target,│
        │     After=combustion.service)                                    │
        │     → disk-encryption-tool-dracut → disk-encryption-tool         │
        │     → shrinks btrfs, cryptsetup reencrypt --encrypt in place,    │
        │       grows back, writes /etc/crypttab, enrolls a *transient*    │
        │       random "enrollment-key" LUKS2 slot into the session keyring│
        │       ( %user:cryptenroll ), regenerates initrd                  │
        └──────────────────────────────────────────────────────────────────┘
                                            │  switch_root into the now-encrypted rootfs
                                            ▼
        ┌──────────────────────────────────────────────────────────────────┐
        │  FIRST BOOT - real root, enrollment stage                        │
        │  (KeyringMode=shared everywhere so the session keyring with the  │
        │   transient enrollment-key survives the switch_root)             │
        │                                                                  │
        │   PATH A: unattended                 PATH B: interactive         │
        │   (sdbootutil-enroll.* credentials    (no sdbootutil-enroll.*    │
        │    already present in                 credentials present)      │
        │    /etc/credstore.encrypted,                                    │
        │    written by combustion above)                                 │
        │        │                                    │                   │
        │        ▼                                    ▼                   │
        │  sdbootutil-enroll.service            jeos-firstboot.service    │
        │  (After=jeos-firstboot.service)          → enroll module        │
        │        │                                    │                   │
        │        └───────────────┬────────────────────┘                   │
        │                        ▼                                        │
        │           sdbootutil enroll --method=<recovery-key|password|    │
        │                                        tpm2[+pin]|fido2>        │
        │           → systemd-cryptenroll (+ systemd-pcrlock make-policy  │
        │             for tpm2/tpm2+pin)                                  │
        │           → on full success: wipe the transient enrollment-key  │
        │             keyslot                                             │
        └──────────────────────────────────────────────────────────────────┘
                                            │
                                            ▼
        ┌──────────────────────────────────────────────────────────────────┐
        │  EVERY SUBSEQUENT BOOT                                           │
        │   systemd-cryptsetup unseals the LUKS2 volume key via TPM2       │
        │   (policy = /var/lib/systemd/pcrlock.json) or FIDO2 token        │
        │   measure-pcr-validator.service (initrd) independently verifies  │
        │   PCR 15 against a signed prediction, for tpm2-measure-pcr=yes   │
        │   devices only                                                   │
        └──────────────────────────────────────────────────────────────────┘
                                            │
                                            ▼
        ┌──────────────────────────────────────────────────────────────────┐
        │  ONGOING SYSTEM LIFECYCLE                                        │
        │  kernel update / snapshot create-default / bootloader update     │
        │   → sdbootutil (via 50-sdbootutil.install, 10-sdbootutil.snapper,│
        │      10-sdbootutil.tukit) sets update_predictions=1              │
        │   → sdbootutil update-predictions (or sdbootutil-update-         │
        │      predictions.service) regenerates systemd-pcrlock policy and │
        │      measure-pcr-* prediction so the new boot chain still unlocks│
        └──────────────────────────────────────────────────────────────────┘
```

---

## 2. The components

### 2.1 `disk-encryption-tool`

A dracut module (`/usr/lib/dracut/modules.d/95disk-encryption-tool`)
plus a standalone script.  Its job is **turn a plain btrfs (or swap)
volume into LUKS2**.

- `disk-encryption-tool` - the actual worker.  Given a mountpoint or
  block device, it:
  1. Refuses if already LUKS or if the target volume name already exists.
  2. For btrfs: mounts (or remounts rw), shrinks to `btrfs
     inspect-internal min-dev-size`, shrinks the partition table entry
     with `sfdisk`, unmounts.  For swap: computes a minimal size and
     shrinks similarly.
  3. Picks a passphrase: `--key VALUE` (explicit), `--keyring NAME`
     (recover/store in kernel keyring `%user:NAME`), or - the normal
     first-boot case - generates a fresh random one (`dd
     if=/dev/urandom bs=8 count=1 | base64`).
  4. Runs `cryptsetup reencrypt --encrypt --reduce-device-size=32m
     --progress-frequency=1 <extra options from /etc/encrypt_options>`
     on the raw partition, in place.
  5. Grows the partition and filesystem back to the full device size,
     remounts.
  6. Tags the generated key as an `enrollment-key` LUKS2 token
     (`cryptsetup token import` with
     `{"type":"enrollment-key","keyslots":["0"]}`) - this is the
     marker that `sdbootutil-enroll` / `jeos-firstboot-enroll` later
     look for and wipe once real enrollment succeeds.
  7. Appends an `/etc/crypttab` entry for the volume.

- `disk-encryption-tool-dracut` - the initrd glue invoked by the
  systemd unit below.  It decides *whether* to encrypt at all (systemd
  credential + kernel cmdline + a "system already configured?"
  heuristic based on `/sysroot/var/lib/YaST2/reconfig_system`), gives
  the console a 10-second Esc-to-abort window unless forced, discovers
  `root_device` via `findmnt`, optionally encrypts extra partitions
  listed in a credential, then encrypts `/sysroot` by calling
  `disk-encryption-tool` itself.

- `module-setup.sh` - the dracut module descriptor: depends on
  dracut's `crypt` module, pulls in `dmi_sysfs` (so systemd
  credentials delivered via QEMU `fw_cfg`/SMBIOS reach the initrd),
  installs `cryptsetup`, `cryptsetup-reencrypt`, `btrfs`, `sfdisk`,
  etc., ships both scripts to `/usr/bin/`, installs and **enables**
  `disk-encryption-tool-dracut.service` inside the built initrd, and
  optionally bakes a build-host `/etc/encrypt_options` into the image.

The package `Requires: combustion cryptsetup keyutils` - Combustion is
a hard runtime dependency because
`disk-encryption-tool-dracut.service` is explicitly ordered
`After=combustion.service` (and `After=ignition-complete.target`), so
that a Combustion/Ignition script gets to run first and set
credentials such as `disk-encryption-tool-dracut.encrypt=force` before
the encryption decision is made.  Since `combustion.service` itself
already `Requires=`/`After=combustion-prepare.service` (§2.4),
depending on it alone is enough to guarantee *both* of combustion's
script invocations have finished.

### 2.2 `jeos-firstboot`

A generic first-boot text wizard.  The FDE support is delivered via a
plugin (described later).

- `jeos-firstboot.service` runs once, gated by
  `ConditionPathExists=/var/lib/YaST2/reconfig_system` (set by
  image-build tooling), masks the stock `systemd-firstboot.service`
  (jeos-firstboot calls the `systemd-firstboot` *binary* itself once
  locale/keymap/timezone dialogs finish), and imports
  `ImportCredential=passwd.plaintext-password.root` +
  `ImportCredential=firstboot.*`.
- It discovers modules from `/etc/jeos-firstboot/modules/*` (wins) and
  `/usr/share/jeos-firstboot/modules/*` (fallback), skips any module
  that is a symlink to `/dev/null`, sorts by a `<module>_priority`
  property (default `50`), and calls, in that order:
  `<module>_systemd_firstboot` (before/around the real
  `systemd-firstboot` call), then `<module>_post` (after), then
  `<module>_cleanup` (on exit).
- `jeos-config` reuses the same module machinery to offer a standalone
  reconfiguration menu (`<module>_jeos_config` hook).
- `jeos-firstboot-snapshot.service` runs after
  `jeos-firstboot.service`, deletes the `reconfig_system` flag (the
  actual "only runs once" mechanism - both units share the same
  `ConditionPathExists`, so once the flag is gone, neither runs
  again), and takes a snapper snapshot of the freshly configured
  system.

FDE support is added *externally*, by `sdbootutil` shipping a module
named `enroll` (`jeos-firstboot-enroll`, described in §2.3) into
`/usr/share/jeos-firstboot/modules/`.

### 2.3 `sdbootutil` - enrollment & PCR-lock subsystem

`sdbootutil` is primarily a bootloader manager (see its own
`ARCHITECTURE.md`), but it owns *all* FDE enrollment and
PCR-prediction logic.  The FDE-relevant pieces:

| File | Role |
|---|---|
| `sdbootutil` (main script) | `enroll`, `unenroll-device`, `list-devices`, `update-predictions` subcommands; all `pcrlock*`/`measure_pcr*`/`enroll_*` functions |
| `sdbootutil-enroll` | Standalone orchestrator: reads `sdbootutil-enroll.*` credentials/keyring and drives unattended enrollment |
| `sdbootutil-enroll.service` | Runs `sdbootutil-enroll` once at first boot, after `jeos-firstboot.service` |
| `jeos-firstboot-enroll` | jeos-firstboot module: interactive TPM2/FIDO2/password/recovery-key dialogs |
| `jeos-firstboot-enroll-override.conf` | Drop-in on `jeos-firstboot.service` adding `KeyringMode=shared` |
| `measure-pcr-generator.sh` | systemd generator: uses `SYSTEMD_FORCE_MEASURE` to enable volume key measurement in PCR15 and order devices |
| `measure-pcr-validator.sh` / `.service` | Initrd-time validation of the signed PCR15 prediction |
| `sdbootutil-update-predictions.service` | Regenerates `systemd-pcrlock` policy prediction after system changes         |
| `kernel-install-sdbootutil.conf` | tmpfiles drop-in disabling stock `kernel-install` scripts that would otherwise bypass sdbootutil's prediction updates |
| `snapper-override.conf`, `10-sdbootutil.tukit(.conf)` | Ensure snapshot/transactional-update operations can regenerate predictions correctly |

### 2.4 `combustion`

A dracut module (depends on `bash firstboot network systemd url-lib`)
that runs a single user-provided script, once, on first boot -
entirely from *within the initrd*, before `switch_root`.  It has no
built-in knowledge of encryption, credentials, or `sdbootutil`; it is
purely a generic "run this script at boot, twice, at two different
points" mechanism that other tools (here, `disk-encryption-tool` and,
indirectly, `sdbootutil`) are wired up to via ordering
(`After=combustion.service`) and via the script's own contents.

- **Gating**: the separate `30firstboot` dracut module's
  `firstboot-detect.service` decides whether this is actually a first
  boot (missing/`uninitialized` `/etc/machine-id`, or
  `combustion.firstboot=`/ `ignition.firstboot=` on the kernel
  cmdline) and only then activates `firstboot.target`, which pulls in
  `combustion.service` (`RequiredBy=firstboot.target`).  On a normal
  (non-first) boot, none of this runs.
- **Config discovery**: in priority order - `combustion.url=` kernel
  cmdline parameter (fetched with `curl`), a QEMU `fw_cfg` blob
  (`opt/org.opensuse.combustion/script`), a VMware
  `guestinfo.combustion.script` key (base64+gzip), or a block device
  with `LABEL=combustion`/`COMBUSTION` (falling back to
  `ignition`/`IGNITION`/`install`/`INSTALL` labels for
  co-installability with Ignition or a KIWI self-install ISO).
  `combustion.rules` (udev) aliases whichever source is found to a
  synthetic `/dev/combustion/config` device, with a 10s (30s on
  aarch64) detection timeout.
- **Two invocations, both pre-switch_root**:
  1. `combustion-prepare.service` runs `/usr/bin/combustion
     --prepare`, which execs the script as `./script --prepare` -
     still in the initrd, before networking/the initqueue is even
     brought up.  A script opts into this by including a line matching
     `^# combustion:(.*)\bprepare\b` anywhere in the file (commonly,
     but not strictly required to be, the first line); the
     `network`/`prepare` flags can be combined on one such line,
     e.g. `# combustion: network prepare`.
  2. `combustion.service` runs `/usr/bin/combustion --complete`,
     which - if `transactional-update` is available - execs the plain
     script (no arguments) inside a `transactional-update shell`, i.e.
     chrooted into `/sysroot`, which at this point is **still the
     plain-text filesystem** (`disk-encryption-tool-dracut.service`
     hasn't run yet).  If the script exits non-zero here, the whole
     transaction is rolled back and `combustion.service` fails, which
     fails the boot - a script that must write a credential should
     treat a failed write as fatal, not swallow the error.
- **No built-in credential support**: combustion has no dedicated verb
  or helper for systemd credentials, `/run/credstore`, or
  `/etc/credstore.encrypted` - a script has to call
  `systemd-creds`/write files itself, exactly as shown in §3.1's
  examples.  A missing config source/script is only a warning, not a
  boot failure.

Because the "normal phase" chroot happens *before*
`disk-encryption-tool-dracut.service` converts `/sysroot` to LUKS2,
anything a combustion script writes into the filesystem at that point
(e.g.  `/etc/credstore.encrypted/sdbootutil-enroll.pw`) is preserved
as-is by the in-place `cryptsetup reencrypt` and is still there,
readable after `switch_root`, on every later boot - which is exactly
why the *persistent* `sdbootutil-enroll.*` credentials use
`/etc/credstore.encrypted` while the *this-boot-only*
`disk-encryption-tool-dracut.encrypt` decision uses the transient,
initrd-local `/run/credstore` instead (only
`combustion-prepare.service`'s earlier, still-in-initrd invocation can
usefully set that one, since `disk-encryption-tool-dracut.service`
also runs before `switch_root`).

---

## 3. The three ways FDE gets enabled

### 3.1 Via `combustion` (fully unattended)

Both of combustion's script invocations happen in the initrd, before
`switch_root` (see §2.4) - there is no separate combustion stage after
the real root has taken over:

1. **`--prepare` phase (`combustion-prepare.service`), before
   `disk-encryption-tool-dracut.service` even starts.** Typical use:
   force encryption without the interactive abort countdown, via the
   *transient* `/run/credstore`:
   ```sh
   #!/bin/bash
   # combustion: prepare
   if [ "$1" = "--prepare" ]; then
       mkdir -p /run/credstore
       echo "force" > /run/credstore/disk-encryption-tool-dracut.encrypt
       exit 0
   fi
   ```
   (The `# combustion: prepare` marker just has to appear on some line
   starting with `# combustion:` - the actual match is `grep -qE '^#
   combustion:(.*)\bprepare\b'` against the whole file, not strictly
   the first line - but putting it first, as shown, is the documented
   convention.)
2. **Normal phase (`combustion.service`), also still
   pre-`switch_root`**, executed via `transactional-update shell`
   chrooted into `/sysroot` - which at this point is **still the
   plain-text filesystem**, since
   `disk-encryption-tool-dracut.service` (ordered
   `After=combustion.service`) hasn't run yet.  Typical use: seed the
   *persistent* `sdbootutil-enroll.*` credentials so enrollment is
   fully unattended once the system reaches its real root, on a later
   boot:
   ```sh
   mkdir -p /etc/credstore.encrypted
   echo "linux" | systemd-creds encrypt --name=sdbootutil-enroll.pw - \
                  /etc/credstore.encrypted/sdbootutil-enroll.pw
   # ...similarly for sdbootutil-enroll.tpm2, .tpm2+pin, .fido2, .rk, .recovery-pin
   ```
   Because this file lands in the plain-text root *before* it is
   converted to LUKS2, it survives the in-place `cryptsetup reencrypt`
   unchanged and is still present (now on the encrypted volume) after
   `switch_root`.  `sdbootutil-enroll.service`'s
   `ImportCredential=sdbootutil-enroll.*` picks it up automatically
   the next time it runs - no interactive dialog is shown at all, and
   `jeos-firstboot-enroll`'s dialog is effectively skipped in practice
   because `sdbootutil-enroll` (or the module itself, depending on
   which credentials exist) has already enrolled the requested
   method(s) and wiped the transient key.

   Nothing stops other first-boot tools (Ignition, cloud-init) from
   writing the very same credential files - Combustion is the
   mechanism actually exercised in this repo's own test suite
   (`disk-encryption-tool/test/testscript`).  Note also that a
   combustion script's normal phase failing (non-zero exit) triggers a
   `transactional-update rollback` and fails `combustion.service`,
   which fails the boot - so a script that must write one of these
   credentials should treat a failed write as fatal.

### 3.2 Via `disk-encryption-tool` (manual/offline, or as the always-present base layer)

`disk-encryption-tool` performs the actual LUKS2 conversion,
regardless of which enrollment path is used afterwards (at least for
image based installations, as Agama / YaST2 can do the same for
traditional installations).

### 3.3 Via `jeos-firstboot` (interactive, local installs)

If no `sdbootutil-enroll.*` credential is present, the `enroll` module
(`jeos-firstboot-enroll`) in `jeos-firstboot.service` presents a
text-mode menu once the transient enrollment key is confirmed present
in the session keyring (`keyctl id` lookup for `%user:cryptenroll`):

- Detects available factors: FIDO2 (`systemd-cryptenroll
  --fido2-device=list`), TPM2 (`/sys/class/tpm/tpm0`).
- Lets the admin choose, repeatedly, from: recovery-key, FIDO2, TPM2
  (with an optional, interactive, double-entry PIN sub-flow), reuse
  the root password, or set an extra password - until they select
  "done".
- On the module's `_post` hook (after `systemd-firstboot` itself has
  run), calls `sdbootutil enroll --method=...` once per selected
  method, in the fixed order recovery-key → password → tpm2[+pin] →
  fido2, exactly mirroring `sdbootutil-enroll`'s order (the two share
  an identical `wipe_enrollment_key()` implementation, deliberately
  kept in sync per a code comment).
- Displays the generated recovery key / PIN via `/run/issue.d/*.issue`
  (so it survives to the login banner) before the transient
  enrollment-key keyslot is wiped.

---

## 4. End-to-end boot sequence

**Stage 0 - image build.** A kiwi image ships a plain-text btrfs root
plus the `95disk-encryption-tool` dracut module baked in
(`disk-encryption-tool.spec` installs it purely via `module-setup.sh`
at `dracut`-build time; there is no RPM `%post` scriptlet).
`/var/lib/YaST2/reconfig_system` is present so `jeos-firstboot` will
run.

**Stage 1 - first-boot initrd, entirely before `switch_root`.**
`firstboot-detect.service` (from the `30firstboot` dracut module)
first decides whether this is actually a first boot at all; if so it
activates `firstboot.target`, which pulls in (in order):
1. `combustion-prepare.service` - runs the combustion script's
   `--prepare` invocation (§2.4/§3.1), e.g.  to force encryption via
   the transient `/run/credstore/disk-encryption-tool-dracut.encrypt`.
2. `combustion.service` - runs the script's normal invocation,
   chrooted into the *still plain-text* `/sysroot` via
   `transactional-update shell`, e.g. to seed persistent
   `/etc/credstore.encrypted/sdbootutil-enroll.*` credentials.
3. `disk-encryption-tool-dracut.service` (`Type=oneshot`,
   `DefaultDependencies=false`, `RequiredBy=firstboot.target`,
   `After=initrd-root-device.target combustion.service
   ignition-complete.target`, `Before=initrd-parse-etc.service`) runs
   `disk-encryption-tool-dracut`, which:
   - Reads credential `disk-encryption-tool-dracut.encrypt` (`no` ⇒
     skip entirely; `force` ⇒ skip the "already configured" heuristic
     and the abort countdown).
   - Reads credential `disk-encryption-tool-dracut.partitions` for any
     extra (non-root) volumes to encrypt.
   - Encrypts `/sysroot` via `disk-encryption-tool` **in place**,
     preserving whatever combustion wrote to the filesystem in step 2
     (including the `sdbootutil-enroll.*` credential files), writes
     `/etc/crypttab`, and leaves a transient `enrollment-key` LUKS2
     token and its passphrase in the shared session keyring
     (`KeyringMode=shared` on the unit).

**Stage 2 - switch_root, enrollment.** After `switch_root` into the
now-encrypted root, `KeyringMode=shared` on every downstream unit
keeps the transient key visible:
- **Unattended path**: `sdbootutil-enroll.service`
  (`After=jeos-firstboot.service`,
  `ImportCredential=sdbootutil-enroll.*`) runs `sdbootutil-enroll`,
  which enrolls every method for which a credential (or matching
  keyring entry) exists, then wipes the transient keyslot on full
  success.
- **Interactive path**: `jeos-firstboot.service` (with the
  `jeos-firstboot-enroll-override.conf` drop-in adding
  `KeyringMode=shared`) runs the `enroll` module's dialogs, then calls
  the same `sdbootutil enroll --method=...` sequence from its `_post`
  hook.

Either way, `sdbootutil enroll` for `tpm2`/`tpm2+pin`:
1. Generates (if not already present) an RSA-4096 keypair for PCR15
   measurement signing (`create_measure_pcr_keys`, only if
   `--no-measure-pcr` is not given).
2. Adds `tpm2-device=auto` (+ `tpm2-measure-pcr=yes` unless disabled)
   to the device's `/etc/crypttab` line.
3. Regenerates the initrd (forced, so the new public key and crypttab
   option land in it).
4. Builds the `systemd-pcrlock` policy for the current boot chain
   (`generate_tpm2_predictions_pcrlock` → `systemd-pcrlock
   ... make-policy`, writing `/var/lib/systemd/pcrlock.json`).
5. Runs `systemd-cryptenroll --wipe-slot=tpm2 --tpm2-device=auto
   --tpm2-public-key= --tpm2-pcrlock=/var/lib/systemd/pcrlock.json
   [--tpm2-with-pin=1] "$dev"`.
6. Enables `sdbootutil-update-predictions.service` for future changes.

For `fido2`, it is simply `systemd-cryptenroll --wipe-slot=fido2
--fido2-device=auto "$dev"` - no PCR involvement at all.

**Stage 3 - every subsequent boot.** `systemd-cryptsetup` (generated
per `/etc/crypttab` line) unseals the LUKS2 volume key using the TPM2
device and the `pcrlock.json` policy (or the FIDO2 token, or a
password/recovery-key prompt).  Independently, for any device with
`tpm2-measure-pcr=yes`, `measure-pcr-validator.service` (initrd,
`Before=initrd-root-device.target`,
`FailureAction=poweroff-immediate`) verifies the signed PCR15
prediction - see §6.

**Stage 4 - ongoing lifecycle.** Kernel installs
(`50-sdbootutil.install` via `kernel-install`), snapshot
create/delete/set-default (`10-sdbootutil.snapper`), and
transactional-update transactions (`10-sdbootutil.tukit`) all funnel
back into `sdbootutil`, which sets an internal `update_predictions=1`
flag whenever the boot chain changes.  At the end of the run - or
explicitly via `sdbootutil update-predictions` /
`sdbootutil-update-predictions.service` - the `systemd-pcrlock` policy
and the PCR15 prediction are regenerated so the new
kernel/initrd/entry still unlocks.

---

## 5. systemd units involved

| Unit | Type | Where it runs | Purpose |
|---|---|---|---|
| `combustion-prepare.service` | oneshot | first-boot initrd, pre-network | Runs the user script's `--prepare` invocation |
| `combustion.service` | oneshot | first-boot initrd, chrooted into plain-text `/sysroot` | Runs the user script's normal invocation |
| `disk-encryption-tool-dracut.service` | oneshot | first-boot initrd | Runs the LUKS2 conversion (§2.1) |
| `jeos-firstboot.service` | oneshot | real root, first boot | Interactive wizard; runs the `enroll` module's dialogs |
| `jeos-firstboot-snapshot.service` | oneshot | real root, first boot | Snapper snapshot after configuration; clears the `reconfig_system` flag |
| `sdbootutil-enroll.service` | oneshot | real root, first boot | Unattended enrollment driven by `sdbootutil-enroll.*` credentials |
| `measure-pcr-validator.service` | oneshot | every boot, initrd | Verifies signed PCR15 prediction before `initrd-root-device.target` |
| `sdbootutil-update-predictions.service` | exec | on demand / triggered | Regenerates `systemd-pcrlock` policy + PCR15 prediction, with backup/restore of `/var/lib/pcrlock.d` on failure |
| `measure-pcr-generator.sh` | dracut generator (not a service) | initrd generation time | Adds `SYSTEMD_FORCE_MEASURE=yes` to relevant `systemd-cryptsetup@.service` units |

Full unit files (verbatim from the repos):

**`combustion-prepare.service`**
```ini
[Unit]
Description=Combustion (preparations)
DefaultDependencies=false
Wants=dev-combustion-config.device
After=dev-combustion-config.device
After=ignition-setup-user.service
Before=ignition-enable-network.service
Before=dracut-initqueue.service nm-initrd.service NetworkManager-initrd.service
Conflicts=initrd-switch-root.target umount.target
Conflicts=dracut-emergency.service emergency.service emergency.target

[Service]
Type=oneshot
StandardOutput=journal+console
ExecStart=/usr/bin/combustion --prepare
```
No `[Install]` section - it is never enabled on its own, only pulled
in via `combustion.service`'s `Requires=`.  It blocks (up to a 10s/30s
timeout drop-in on `dev-combustion-config.device`) until a config
source is detected by `combustion.rules`, and is explicitly ordered
before networking comes up (`Before=dracut-initqueue.service ...`) so
a `--prepare` script gets to configure things (including writing
credentials) before network services start.

**`combustion.service`**
```ini
[Unit]
Description=Combustion
DefaultDependencies=false
Requires=initrd-root-device.target
After=initrd-root-device.target
Requires=combustion-prepare.service
After=combustion-prepare.service
After=network-online.target
Wants=network-online.target
After=ignition-complete.target
Before=initrd-parse-etc.service
Before=firstboot.target
Conflicts=initrd-switch-root.target umount.target
Conflicts=dracut-emergency.service emergency.service emergency.target

[Service]
Type=oneshot
ExecStart=/usr/bin/combustion --complete

[Install]
RequiredBy=firstboot.target
```
Runs the script's normal invocation inside `transactional-update
shell` (chrooted into `/sysroot`, still plain-text at this point).
`Requires=`/`After=combustion-prepare.service` means anything ordered
`After=combustion.service` (like
`disk-encryption-tool-dracut.service`) is guaranteed to run after
*both* combustion invocations.  Like `combustion-prepare.service`, it
is entirely an initrd unit (`Conflicts=initrd-switch-root.target`,
`Before=initrd-parse-etc.service`) - it never runs post-`switch_root`.

**`disk-encryption-tool-dracut.service`**
```ini
[Unit]
Description=Encrypt root disk
DefaultDependencies=false
Requires=initrd-root-device.target
After=initrd-root-device.target
After=combustion.service
After=ignition-complete.target
Before=initrd-parse-etc.service
Conflicts=initrd-switch-root.target umount.target
Conflicts=dracut-emergency.service emergency.service emergency.target
OnFailure=emergency.target
OnFailureJobMode=isolate

[Service]
Type=oneshot
KeyringMode=shared
ExecStart=/usr/bin/disk-encryption-tool-dracut
ImportCredential=disk-encryption-tool-dracut.*

[Install]
RequiredBy=firstboot.target
```

**`jeos-firstboot.service`**
```ini
[Unit]
Description=SUSE JeOS First Boot Wizard
After=apparmor.service local-fs.target plymouth-start.service YaST2-Second-Stage.service
Conflicts=plymouth-start.service
Before=getty@tty1.service serial-getty@hvc0.service serial-getty@ttyS0.service serial-getty@ttyS1.service serial-getty@ttyS2.service serial-getty@ttyAMA0.service
Before=display-manager.service
ConditionPathExists=/var/lib/YaST2/reconfig_system
OnFailure=poweroff.target
Before=wicked.service systemd-user-sessions.service
After=NetworkManager.service
After=cloud-init-local.service
Wants=jeos-firstboot-snapshot.service

[Service]
Type=oneshot
Environment=TERM=linux
RemainAfterExit=yes
ExecStartPre=/bin/sh -c "/usr/bin/plymouth quit 2>/dev/null || :"
ExecStart=/usr/sbin/jeos-firstboot
StandardOutput=tty
StandardInput=tty
KeyringMode=shared
ImportCredential=passwd.plaintext-password.root
ImportCredential=firstboot.*

[Install]
WantedBy=default.target
```
(`KeyringMode=shared` here is actually applied via the
`jeos-firstboot-enroll-override.conf` drop-in shipped by `sdbootutil`,
not the base `jeos-firstboot` package - without it the module can't
see the transient enrollment key created back in the initrd.)

**`jeos-firstboot-enroll-override.conf`** (drop-in for `jeos-firstboot.service`)
```ini
[Service]
KeyringMode=shared
```

**`sdbootutil-enroll.service`**
```ini
[Unit]
Description=Enroll encrypted root disk
DefaultDependencies=false
After=jeos-firstboot.service

[Service]
Type=oneshot
RemainAfterExit=yes
KeyringMode=shared
ExecStart=/usr/bin/sdbootutil-enroll
ImportCredential=sdbootutil-enroll.*

[Install]
WantedBy=default.target
```

**`measure-pcr-validator.service`**
```ini
[Unit]
Description=Validate LUKS2 devices
DefaultDependencies=false
FailureAction=poweroff-immediate
ConditionPathExists=/etc/crypttab
Wants=cryptsetup.target
After=cryptsetup.target
Before=initrd-root-device.target

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/bin/measure-pcr-validator
Environment=TERM=linux
ExecStopPost=/bin/sh -c "/usr/bin/plymouth quit 2>/dev/null || :"
StandardOutput=journal+console
StandardInput=null

[Install]
WantedBy=initrd-root-device.target
```

**`sdbootutil-update-predictions.service`**
```ini
[Unit]
Description=Update TPM predictions
ConditionSecurity=tpm2

[Service]
Type=exec
KeyringMode=shared
PrivateTmp=yes
ExecCondition=/usr/bin/sh -c '[ -e /etc/crypttab ] && grep -q tpm2-device /etc/crypttab'
ExecStartPre=/usr/bin/sh -c 'mkdir -p "/tmp/$INVOCATION_ID"; [ ! -d /var/lib/pcrlock.d ] || cp -a /var/lib/pcrlock.d "/tmp/$INVOCATION_ID/pcrlock.d"'
ExecStart=/usr/bin/sh -c '\
    systemctl --quiet is-active sdbootutil-update-predictions-shutdown.service && systemctl stop sdbootutil-update-predictions-shutdown.service; \
    while true; do \
        rm -f /run/sdbootutil/update-predictions; \
        /usr/bin/sdbootutil -v update-predictions > /dev/null || exit $?; \
        [ -f /run/sdbootutil/update-predictions ] || break; \
    done'
ExecStopPost=/usr/bin/sh -c '\
    if [ "$SERVICE_RESULT" != "success" ] && [ "$SERVICE_RESULT" != "exec-condition" ]; then \
        if [ -d "/tmp/$INVOCATION_ID/pcrlock.d" ]; then \
            rm -rf /var/lib/pcrlock.d/*; \
            cp -a "/tmp/$INVOCATION_ID/pcrlock.d/." /var/lib/pcrlock.d/; \
        fi; \
    fi; \
    rm -rf "/tmp/$INVOCATION_ID"'
ImportCredential=sdbootutil-update-predictions.*
KillSignal=SIGCONT
TimeoutStopSec=120

[Install]
WantedBy=default.target
```
Enabled automatically by `enroll_tpm2()` the first time TPM2
enrollment succeeds.  Note the backup/restore dance around
`/var/lib/pcrlock.d`: if regeneration fails partway, the previous,
known-good pcrlock component set is restored so the machine doesn't
end up unable to unseal its own disk.

---

## 6. The two independent PCR mechanisms

It's easy to conflate these; they are **separate, complementary**
checks operating on different data:

### 6.1 `systemd-pcrlock` - boot-chain attestation for the TPM2 *keyslot itself*

This is what actually gates whether the TPM2 will release the sealed
secret.  `sdbootutil` builds a per-machine "pcrlock" policy (a set of
authorized/expected event-log measurements - firmware, Secure Boot
state/authority, GPT, bootloader PE binaries, kernel command line,
initrd, boot entry) via `systemd-pcrlock lock-*` subcommands, wrapped
by `pcrlock()`/`pcrlock_lock()` in the main `sdbootutil` script, then
folded together with `systemd-pcrlock make-policy` into
`/var/lib/systemd/pcrlock.json`.  This is handed to
`systemd-cryptenroll` as
`--tpm2-pcrlock=/var/lib/systemd/pcrlock.json` at TPM2 enrollment
time, and to `systemd-cryptsetup` implicitly at unlock time via the
LUKS2 token metadata.  Default PCR set: `0,2,4,7,9` for systemd-boot,
`0,2,4,7,8,9` for grub2-bls (PCR 0/2 - firmware - are dropped under
virtualization).  This mechanism supersedes the older, now-removed
`pcr-oracle` static-signed-policy approach referenced in the 2023
MicroOS blog post; the current code explicitly passes
`--tpm2-public-key=` (empty) to disable that legacy path.

Whenever the boot chain changes (new kernel, new snapshot default,
bootloader update, Secure Boot authority change), this policy must be
regenerated - that's what `sdbootutil
update-predictions`/`sdbootutil-update-predictions.service` does.

### 6.2 `measure-pcr-*` - PCR15 device/volume-key binding check

This is an **sdbootutil-specific, additional** integrity check,
layered on top of (2), that protects against the LUKS device/keyslot
mapping being swapped underneath an otherwise-valid pcrlock policy
(e.g.  an attacker presenting a different, TPM2-pcrlock-sealed volume
as `root`).  It only applies to crypttab entries with
`tpm2-measure-pcr=yes` (set automatically by `sdbootutil enroll
--method=tpm2[+pin]` unless `--no-measure-pcr` is passed):

- `measure-pcr-generator.sh` is a **dracut generator**, run
  automatically during initrd assembly (not a runtime service).  For
  every `systemd-cryptsetup@<name>.service` matching a
  `tpm2-measure-pcr=yes` crypttab line, it drops
  `Environment=SYSTEMD_FORCE_MEASURE=yes` plus ordering between
  successive cryptsetup units, so PCR15 gets extended, at unlock time,
  with an HMAC of each device's real LUKS volume key (via the Rust
  `uhmac` helper for sha1/sha256/sha384/sha512), in a deterministic
  order.
- After enrollment (or any crypttab change), `sdbootutil`
  (`generate_tpm2_predictions_pcr_15`) computes the expected final
  PCR15 value offline the same way, and **signs it** with a dedicated
  RSA-4096 keypair (`create_measure_pcr_keys`, `openssl genrsa
  ... 4096`, stored at
  `/var/lib/sdbootutil/measure-pcr-{private,public}.pem`; only the
  public half ships in the initrd), writing
  `/var/lib/sdbootutil/measure-pcr-prediction` + `.sha256` and copying
  both to the ESP.
- At boot, `measure-pcr-validator.sh`/`.service` verifies the RSA
  signature over the prediction, reads the live PCR15 value from
  `/sys/class/tpm/tpm0/pcr-<bank>/15` (strongest bank first), and
  compares.
  - Match → boot continues.
  - Mismatch → `measure-pcr-validator.ignore=yes|1|true` on the kernel
    command line (`getargbool no measure-pcr-validator.ignore`)
    downgrades this to a printed warning; otherwise the unit exits
    non-zero, `FailureAction=poweroff-immediate` fires, and the
    machine halts rather than booting with a validated-but-unexpected
    key mapping.
  - Absent PCR15/no `tpm2-measure-pcr=yes` entries → the check is
    silently skipped (`exit 0`), so this mechanism is opt-in and
    non-fatal by default for systems that don't use it.

In short: **pcrlock answers "did the correct, expected boot chain
run?"**; **measure-pcr-15 answers "is the volume I just unlocked
actually the one the signed prediction says it should be?"**.  Both
are regenerated together by `generate_tpm2_predictions()`/`sdbootutil
update-predictions`, but they write to, and are checked against,
entirely separate state (`/var/lib/systemd/pcrlock.json` vs.
`/var/lib/sdbootutil/measure-pcr-*`).

---

## 7. systemd credentials reference

All credentials are read either from `$CREDENTIALS_DIRECTORY`
(systemd's tmpfs credential store, populated from
`ImportCredential=`/`LoadCredential=`/`SetCredential=`,
`systemd.set_credential=` on the kernel cmdline, an SMBIOS/`fw_cfg`
OEM string on QEMU, or a file under `/etc/credstore(.encrypted)/`) or,
for the interactive path, the kernel session keyring (works because
`KeyringMode=shared` is set on every relevant unit so the
initrd-created keyring survives `switch_root`).

| Credential / keyring key | Consumer | Format / meaning |
|---|---|---|
| `disk-encryption-tool-dracut.encrypt` | `disk-encryption-tool-dracut` | `no` = skip encryption entirely; `force` = skip the "already configured" heuristic and the 10 s abort countdown; unset = default heuristic-driven behavior |
| `disk-encryption-tool-dracut.partitions` | `disk-encryption-tool-dracut` | Multi-line list, one `volume_name device options` triplet per line; extra (non-root) volumes to encrypt, or an override for the root volume's name/options |
| `passwd.plaintext-password.root` | `jeos-firstboot.service` | Root password; if present, the interactive root-password dialog is skipped |
| `firstboot.locale` | `jeos-firstboot` | Skips the locale dialog |
| `firstboot.keymap` | `jeos-firstboot` | Skips the keyboard layout dialog |
| `firstboot.license-agreed` | `jeos-firstboot` | Skips the EULA dialog |
| `firstboot.timezone` | `jeos-firstboot` | Skips the timezone dialog |
| `sdbootutil-enroll.current-pw` | `sdbootutil-enroll` | Existing password/passphrase needed to authorize enrolling a *new* factor on an already-enrolled device |
| `sdbootutil-enroll.pw` | `sdbootutil-enroll`, `jeos-firstboot-enroll` | Enroll a password keyslot |
| `sdbootutil-enroll.rk` | `sdbootutil-enroll` | Enroll a recovery-key keyslot |
| `sdbootutil-enroll.tpm2` | `sdbootutil-enroll`, `jeos-firstboot-enroll` | Enroll a TPM2 keyslot (no PIN) |
| `sdbootutil-enroll.tpm2+pin` | `sdbootutil-enroll`, `jeos-firstboot-enroll` | Enroll a TPM2 keyslot; value is the PIN |
| `sdbootutil-enroll.fido2` | `sdbootutil-enroll`, `jeos-firstboot-enroll` | Enroll a FIDO2 keyslot |
| `sdbootutil-enroll.recovery-pin` | `sdbootutil-enroll` | Recovery PIN for the `systemd-pcrlock` NV-index (lets a recovery-key holder re-authorize/re-lock the pcrlock policy later, e.g. after a key compromise) |
| `sdbootutil-update-predictions.*` | `sdbootutil-update-predictions.service` | Same import-glob pattern, e.g. to supply a recovery PIN during unattended, scheduled prediction refreshes |
| `%user:cryptenroll` (kernel keyring, not a systemd credential) | `disk-encryption-tool` → `sdbootutil enroll`, `jeos-firstboot-enroll` | The transient random passphrase generated during Stage 1; consumed by `require_unlock()` to authorize enrolling the first real factor, then the corresponding LUKS2 slot is wiped |
| `%user:sdbootutil-pw`, `%user:sdbootutil-tpm2-pin`, `%user:sdbootutil-recovery-pin` | `sdbootutil enroll` | Interactive-path equivalents of the `sdbootutil-enroll.*` credentials, set by `jeos-firstboot-enroll`'s dialogs instead of by an external provisioning tool |
| `cryptenroll.tpm2-pin` | `systemd-cryptenroll` (not set anywhere by these repos) | Standard systemd credential for an unattended TPM2-PIN unlock; mentioned in a `sdbootutil` comment only to explain why a PIN-protected TPM2 slot can't self-authorize a *new* enrollment unattended |

---

## 8. Key on-disk state

| Path | Written by | Meaning |
|---|---|---|
| `/etc/crypttab` | `disk-encryption-tool` (initial), `sdbootutil enroll` (adds `tpm2-device=`/`fido2-device=`/`tpm2-measure-pcr=` options) | Standard crypttab; drives both `systemd-cryptsetup@.service` generation and `measure-pcr-generator.sh` |
| `/etc/encrypt_options` | Admin / build host | Extra `cryptsetup reencrypt` arguments, one per non-`#`-containing line (e.g. `--type=luks1`, `--iter-time=2000`) |
| `/etc/credstore.encrypted/*` | Combustion's normal-phase script (or Ignition/cloud-init equivalent), via `systemd-creds encrypt`, written into the still plain-text root before LUKS2 conversion | Persistent, encrypted systemd credentials - mainly `sdbootutil-enroll.*` |
| `/run/credstore/*` | Combustion's `--prepare`-phase script, in the initrd | Transient (this-boot-only) credentials - mainly `disk-encryption-tool-dracut.encrypt` |
| `/var/lib/systemd/pcrlock.json` | `systemd-pcrlock make-policy` (via `sdbootutil`) | The TPM2 boot-chain policy handed to `systemd-cryptenroll --tpm2-pcrlock=` |
| `/var/lib/pcrlock.d/*.pcrlock.d/*.pcrlock` | `sdbootutil` `pcrlock_*` functions | Individual pcrlock components (per bootloader entry / kernel / cmdline variation) fed into `make-policy` |
| `/var/lib/sdbootutil/measure-pcr-{private,public}.pem` | `sdbootutil create_measure_pcr_keys` | RSA-4096 keypair signing the PCR15 prediction; only the public key is copied into the initrd/ESP |
| `/var/lib/sdbootutil/measure-pcr-prediction[.sha256]` | `sdbootutil generate_tpm2_predictions_pcr_15` | Signed, expected PCR15 value; validated at boot by `measure-pcr-validator` |
| `/var/lib/sdbootutil/crypttab.sha1` | `sdbootutil` | Tracks whether `/etc/crypttab` changed since the last prediction run |
| `/var/lib/misc/transactional-update.state` | `sdbootutil` | State carried across reboots on transactional (read-only root) systems |

---

## 9. Source repositories

- `disk-encryption-tool` -
  https://github.com/openSUSE/disk-encryption-tool
- `jeos-firstboot` - https://github.com/openSUSE/jeos-firstboot
- `sdbootutil` - https://github.com/openSUSE/sdbootutil (see also its
  own `ARCHITECTURE.md` for the bootloader-entry/snapshot side, not
  duplicated here)
- `combustion` - https://github.com/openSUSE/combustion
- Background/narrative (predates the `systemd-pcrlock` migration
  described here, which replaced the earlier `pcr-oracle`-based
  signed-policy approach):
  https://microos.opensuse.org/blog/2023-12-20-sdboot-fde/
