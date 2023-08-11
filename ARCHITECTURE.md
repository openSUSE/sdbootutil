systemd implementation of the bootloader spec
---------------------------------------------

The kernel-install script shipped with systemd can install kernels in the EFI
partition according to the [bootloader
specification](https://uapi-group.org/specifications/specs/boot_loader_specification/).

Assuming two installed kernels, 1.2.3-1-default and 4.5.6-1-default
one would call

    kernel-install add /lib/modules/1.2.3-1-default/vmlinuz
    kernel-install add /lib/modules/3.4.5-1-default/vmlinuz

Scripts in `/usr/lib/kernel/install.d/` would copy the kernel into
the ESP, generate an initrd and create a config to boot the
specified kernel.

With an entry token (eg machine-id) of 2ceda9f (shortened for
readability) the ESP would look like this:

    ├── 2ceda9f
    │   ├── 1.2.3-1-default
    │   │   ├── initrd
    │   │   └── linux
    │   └── 4.5.6-1-default
    │       ├── initrd
    │       └── linux
    ├── EFI
    │   ├── BOOT
    │   │   └── BOOTX64.EFI
    │   └── systemd
    │       └── systemd-bootx64.efi
    └── loader
        └── entries
            ├── 2ceda9f-1.2.3-1-default.conf
            └── 2ceda9f-4.5.6-1-default.conf

Therefore an entry file might look like this:

    title      openSUSE Tumbleweed
    version    1.2.3-1-default
    machine-id 2ceda9f
    sort-key   opensuse-tumbleweed
    options    root=UUID=abc...
    linux      /2ceda9f/1.2.3-1-default/linux
    initrd     /2ceda9f/1.2.3-1-default/initrd


This scheme is basically designed so a specific instance of an
operating system (in the example 2ceda9f) can be booted with
different kernel versions. It is assumed that a kernel with a given
version is unique and each kernel has one specific initrd. Removing
a kernel (ie kernel-install remove 1.2.3-1-default.conf) would
remove the kernel, initrd and entry file. The root filesystem can
be specified as option or discovered automatically according to the
[discoverable partition specification](https://uapi-group.org/specifications/specs/discoverable_partitions_specification/).


Introducing snapshots
---------------------

With btrfs snapshots, the same root filesystem holds multiple
generations of the operating system. It's necessary to add a
`rootflags=` parameter to the options to tell the kernel which
subvolume to mount. Absence of the parameter would boot the default
subvolume.

So when creating a snapshot, snapper hook scripts could copy the
existing entry files, adding parameters to boot the new snapshot. So
for example if snapshot 15 was created the files would look like
this:

    └── loader
        └── entries
            ├── 2ceda9f-1.2.3-1-default.conf
            ├── 2ceda9f-4.5.6-1-default.conf
            ├── 2ceda9f-1.2.3-1-default-15.conf
            └── 2ceda9f-4.5.6-1-default-15.conf

Since the ESP is FAT, a separator like @ can't be used to add the
snapshot number to the file name.

An entry file might now look like this:

    title      openSUSE Tumbleweed
    version    15@1.2.3-1-default
    machine-id 2ceda9f
    sort-key   opensuse-tumbleweed
    options    root=UUID=abc... rootflags=subvol=@/.snapshots/15/snapshot
    linux      /2ceda9f/1.2.3-1-default/linux
    initrd     /2ceda9f/1.2.3-1-default/initrd

In this example the snapshot number is prepended to the version.
This seems to be the best way right now to get the sorting right.

At this point the entry refers to the same kernel and initrd as used
in the running system. The system may install a different kernel
with same uname or recreate the initrd any time though. That would
potentially break booting the snapshot. So a snapshot entry must
also point to a matching kernel and initrd. FAT has no copy-on-write
features, so somehow those files need to be made unique in a
different way. It would be possible to encode the snapshot number
into the file name but then every snapshot would have it's own copy
of kernel and initrd, even when they could be shared. So a better
option is to use the file's checksum instead.

So that makes an entry look like this

    title      openSUSE Tumbleweed
    version    15@1.2.3-1-default
    machine-id 2ceda9f
    sort-key   opensuse-tumbleweed
    options    root=UUID=abc... rootflags=subvol=@/.snapshots/15/snapshot
    linux      /2ceda9f/1.2.3-1-default/linux-b021b508eb42b2afd06de8f0242b9727aa7dc494
    initrd     /2ceda9f/1.2.3-1-default/initrd-7b200fad3d005285ca914069a4740a5b6874c0ae

To avoid wasting any space, the default entries should also use
checkums and never store plain "linux" and "initrd" files.

With this scheme `kernel-install remove` would no longer work
though. It must especially not delete the kernel version directory
(eg 1.2.3-1-default) as it may contain files needed by other snapshots.
Therefore a `bootctl unlink` and `bootctl cleanup` features were
imlemented (https://github.com/systemd/systemd/pull/26103) to use
reference counting when deleting entries.

Default boot entry
------------------
So far the btrfs default subvolume flag is used to select which
subvolume to boot in absence of boot options. Now this means each
time the default subvolume is changed (ie rollback), the
systemd-boot entries that do not refer to a subvolume need to be
changed to ones that select the correct kernel and initrd combo.

Systemd-boot itself has a mechanism to set a default entry. So
setting the btrfs default subvolume alone actually has no effect.
Therefore snapper also needs to set the systemd-boot default when
changing the default subvolume.

Now creating a snapshot would create an entry with snapshot number
embedded anyway. Therefore the extra step to also create an entry
without snapshot number is not necessary. Instead all entries could
just have the subvolume setting. Selecting the default would only be
done based on the entry, not on the default subvolume. This would
also be the more natural way for MicroOS where no read-write
subvolume exists.

So the ESP would look like this:

    ├── 2ceda9f
    │   ├── 1.2.3-1-default
    │   │   ├── initrd-6272c61615fc18d5158aadda22fb0955c98382c8
    │   │   └── linux-dcebae96db3cab7ffeb8883e78423a99d31d2bfd
    │   └── 4.5.6-1-default
    │       ├── initrd-7b200fad3d005285ca914069a4740a5b6874c0ae
    │       ├── initrd-1b6b609e33104d68bd543a11a0b8a4d354478f46
    │       └── linux-b021b508eb42b2afd06de8f0242b9727aa7dc494
    ├── EFI
    │   ├── BOOT
    │   │   └── BOOTX64.EFI
    │   └── systemd
    │       └── systemd-bootx64.efi
    └── loader
        └── entries
            ├── 2ceda9f-1.2.3-1-default-1.conf
            ├── 2ceda9f-4.5.6-1-default-1.conf
            ├── 2ceda9f-1.2.3-1-default-15.conf
            └── 2ceda9f-4.5.6-1-default-15.conf

Distribution integration
------------------------

The scheme described in this document is implemented by
[sdbootutil](https://github.com/lnussel/sdbootutil). Similar to
`kernel-install` it allows to add and remove kernels to/from the
ESP but is fully snapshot aware. To speed up snapshot creation,
`sdbootutil` also tries to be smart about the initrd and reuses
existing ones from the parent snapshot if possible.
That also means initrds are not updated based on user space
changes at this point. Only kernel changes trigger initrd creation.

`sdbootutil` is called from a [snapper
plugin](https://github.com/lnussel/sdbootutil/blob/main/10-sdbootutil.snapper)
to manage entires on snapshot creation and removal, as well as when
the default subvolume is set.

Kernel package installation and removal also trigger calls to
`sdbootutil` via [file
triggers](https://github.com/lnussel/sdbootutil/blob/main/kernelhooks.lua).
Unfortunately the file trigger method seems to be unreliable with zypper.
Moreover, file triggers can't make an rpm transaction fail
(https://github.com/rpm-software-management/rpm/issues/2581).

Secure boot support
------------------------

Upstream `bootctl` has no support for shim yet
(https://github.com/systemd/systemd/issues/27234). That's required
for secure boot support though. Therefore `sdbootutil` also
implements an `install` feature.

If `/usr/share/efi/$(uname -m)` exists, shim and related files get
installed into the ESP

    └── EFI
        ├── boot
        │   ├── BOOTX64.EFI    <-- shim
        │   ├── MokManager.efi
        │   └── fallback.efi
        └─── systemd
             ├── MokManager.efi
             ├── boot.csv
             ├── grub.efi      <-- actually systemd-boot
             └── shim.efi

Future plans
------------

In general sdbootutil should not exist. It's features should be
implemented in `bootctl`, `kernel-install` or `snapper`.

To simplify handling of the initrd, it would be desirable to switch
to e.g. UKIs that are built on server side. That way no guessing
would be needed as to whether an initrd needs to be updated.
