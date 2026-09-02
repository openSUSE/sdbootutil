# Boot and FDE state regression suite

Two properties decide whether a machine still comes up and still opens
its disks. Neither is visible from a single command, and both break
silently — the report arrives at the firmware prompt, one reboot too
late:

- **I1 — ESP consistency.** Every registered entry points at a kernel
  and an initrd that are present, every snapshot that must boot still
  has an entry, no two entries describe the same system, nothing is
  left in the ESP that no entry refers to, and the free space reserve
  is respected.
- **I2 — Unlock continuity.** Every device the TPM2 is supposed to open
  is still covered by a policy that matches the PCRs as they are now,
  and the PCR 15 prediction still describes the current `/etc/crypttab`.

`fde-state` asserts them, and never calls sdbootutil: a harness written
in terms of the tool under test cannot notice that tool regressing,
because whatever the tool believes about the system it would report
back unchanged. The state is read from the system itself — `bootctl`,
`cryptsetup`, `snapper`, the on-disk pcrlock policy, sysfs and
`/etc/crypttab`.

## Usage

```console
$ tests/run --list                      # available scenarios
$ tests/run noop                        # baseline every reachable guest
$ tests/run --push add-all-kernels      # drive the working tree, not the installed rpm
$ tests/run -g <address> orphan-initrd
```

Scenarios run against a `cp -a` copy of the ESP, handed to bootctl and
sdbootutil through `SYSTEMD_ESP_PATH` and `--esp-path`. Nothing a
scenario does can stop a guest from booting. State outside the ESP —
crypttab, the LUKS headers, the pcrlock policy — is read from the real
system and never written; `--disable-predictions` keeps sdbootutil off
the TPM2 NVIndex and `--no-variables` keeps it out of the EFI variables,
so the grubenv it writes instead lands in the copy with everything else.

The scenarios, roughly in order of how much of the tool they touch:

| scenario | what it holds sdbootutil to |
|---|---|
| `noop` | the capture is deterministic and the guest is healthy |
| `orphan-initrd` | negative control: the harness is not blind |
| `add-all-kernels` | the ordinary kernel-install path is idempotent |
| `missing-initrd` | a deleted initrd is regenerated and the entry made whole |
| `remove-readd-kernel` | reference counting frees exactly what it should, and the round trip returns |
| `update-all-entries` | editing an entry in place is not a silent no-op, and is deterministic |

### What it reports today

Everything passes except on a guest without snapshots, which fails
`I1.5` on the three scenarios that create an entry —
`add-all-kernels`, `missing-initrd` and `remove-readd-kernel`:

```
FAIL I1.5 more than one entry for the same kernel and subvolume: 7.0.12-1-default@- 7.1.2-1-default@-
```

`entry_conf_file()` adds the grub2 ordering prefix there too, because
`subvol_is_ro()` returns false when there are no snapshots to ask
about, so every kernel ends up with a prefixed entry next to the
unprefixed one `kernel-install` wrote.
This is a real defect and the harness is doing its job by failing;
`update-all-entries` passes on the same guest because it only edits
entries and never creates one.

`fde-state` can also be used on its own, on any machine:

```console
# fde-state capture > before      # sorted key=value, no secrets, safe to attach to a bug
# fde-state check                 # assert I1 and I2
# fde-state diff before after
```

## Adding a scenario

A scenario is a bash script in `scenarios/`, run on the guest with
`$ESP` and `$SDBOOTUTIL` set. Line 3 is the description shown by
`--list`. Setting `EXPECT=fail` marks a negative control, which passes
only when the invariants break. Exiting 77 says the scenario does not
apply to this guest and is reported as a skip.

Scope a scenario to what the command under test actually owns.
`add-all-kernels` installs the kernels of one snapshot, so a scenario
that damages an entry belonging to a different snapshot is measuring a
gap rather than the path it names, which is why `missing-initrd`
selects its target through the running subvolume.

Keep at least one negative control passing. `orphan-initrd` exists
because a harness that never fails is not evidence of anything.

## Notes on the guests

- The addresses are not in the repository: they are throwaway VMs whose
  leases change, and an address is not a fact about the code. Put them
  in `tests/guests` (one per line, untracked) or in `$FDE_TEST_GUESTS`,
  or name one with `-g`.
- The fleet does not run the same sdbootutil, and the rpm version
  string does not say which. `run` refuses to drive a build that has no
  `--disable-predictions`, because without it a scenario would rewrite
  the real TPM2 NVIndex even though the ESP is redirected. Use
  `--push`.
- `WARN I2.3` (a requested PCR is not bound) is reported but does not
  fail. Dropping a PCR that cannot be predicted right now is designed
  behaviour and the boot service adds it back. The regression that
  matters — a PCR that *was* bound and is not any more — shows up in
  `diff` as a removed `pcrlock/policy/pcr/N/...` line.
- `WARN I1.6` (the default entry is written without the `.conf` suffix)
  is the same grub2-bls conformance gap as `LoaderEntrySelected`. The
  entry exists and the machine boots it, so it is reported rather than
  failed until grub2-bls is fixed.
- Keys under `vol/` (PCR contents, free space, `LoaderEntrySelected`)
  change on their own and are excluded from `diff`.
