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
$ tests/run --push $(tests/run --list | awk '{print $1}')   # everything
```

The guests are separate machines that share nothing, so they run at the
same time; each one's output is collected and printed whole, in the
order the guests were named, so a parallel run reads exactly like a
serial one. Naming several scenarios runs them one after another on each
guest and pays for the connection and the copies once — a full sweep of
eight scenarios over six guests takes about three and a half minutes,
almost all of it dracut.

Scenarios run against a `cp -a` copy of the ESP, handed to bootctl and
sdbootutil through `SYSTEMD_ESP_PATH` and `--esp-path`. The copy is made
again before every scenario, so one cannot inherit what the one before
it did. Nothing a scenario does can stop a guest from booting. State
outside the ESP —
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
| `boot-counter` | a `+N` counter is kept, and does not hide the entry from removal |
| `repair-entry` | `cleanup --repair` restores the file and keeps a hand-edited command line |

### What it reports today

Everything passes, on transactional and non-transactional guests, with
systemd-boot and with grub2-bls, with and without snapshots. Two
warnings are expected and do not fail a run; both are explained under
*Notes on the guests*.

`missing-initrd` skips where the running subvolume's entry already
shares its initrd with another entry, since deleting it would damage
more than the one entry under test.

`fde-state` can also be used on its own, on any machine:

```console
# fde-state capture > before      # sorted key=value, no secrets, safe to attach to a bug
# fde-state check                 # assert I1 and I2
# fde-state diff before after
```

## Secret routing (`tests/unit`)

`run` never writes outside a copy of the ESP, which is what makes it
safe to point at a machine one wants to keep — and what puts enrollment
out of its reach. `enroll` writes LUKS2 headers and the pcrlock
NVIndex, and it cannot even be aimed: `detect_tracked_devices` builds
its list from `lsblk` ∩ `/etc/crypttab`, so a scratch container added to
crypttab is enrolled *alongside* the real root, not instead of it.

`tests/unit` covers the part that does not need any of that. Which
source of a secret wins, what is published to the kernel keyring, and
which warnings are printed are all decided before anything is written,
and a scratch LUKS2 container on a loop device answers `cryptsetup` and
`systemd-cryptenroll` exactly like a root device would. It sources the
library half of `sdbootutil` — everything above the `####### main`
marker, which the completion data already relies on — and calls the
functions one at a time.

```console
# tests/unit
ok   get_device_password takes CURRENT_PW
...
13 cases, all passed
```

It needs root, for `losetup` and the keyring, and it uses the real
keyring names because the names are in the code: it refuses to start
when one of them is already there rather than overwrite a secret left
behind by an installer or an interrupted enrollment, and it purges them
between cases. Every case runs in a subshell with its own environment,
against a freshly formatted header, and can replace a function —
`t_erk_unreachable_pin` fakes a TPM2 slot, because the answer it checks
must not depend on the machine having a TPM2.

What stays out of reach here is the NVIndex itself: authorizing with the
old recovery PIN, the "not needed, so not validated" skip, the dead end
of github issue 250. Those need a real TPM2 and destroy the enrollment
of the machine they run on, so they belong to a destructive guest suite
that does not exist yet, and whose first requirement is a way to put a
guest back.

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
