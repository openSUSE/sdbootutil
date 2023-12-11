#
# spec file for package sdbootutil
#
# Copyright (c) 2023 SUSE LLC
#
# All modifications and additions to the file contributed by third parties
# remain the property of their copyright owners, unless otherwise agreed
# upon. The license for this file, and modifications and additions to the
# file, is the same license as for the pristine package itself (unless the
# license for the pristine package is not an Open Source License, in which
# case the license is the MIT License). An "Open Source License" is a
# license that conforms to the Open Source Definition (Version 1.9)
# published by the Open Source Initiative.

# Please submit bugfixes or comments via https://bugs.opensuse.org/
#


%define nvr %{name}-%{version}-%{release}
%if 0%{?_build_in_place}
%define git_version %(git log '-n1' '--date=format:%Y%m%d' '--no-show-signature' "--pretty=format:+git%cd.%h")
BuildRequires:  git-core
%else
# this is required for obs' source validator. It's
# 20-files-present-and-referenced ignores all conditionals. So the
# definition of git_version actually happens always.
%define git_version %{nil}
%endif
Name:           sdbootutil
Version:        0%{git_version}
Release:        0
Summary:        script to install shim with sd-boot
License:        MIT
URL:            https://en.opensuse.org/openSUSE:Usr_merge
Source:         %{name}-%{version}.tar
Requires:       efibootmgr
Requires:       systemd-boot
Requires:       jq
Requires:       sed
Requires:       pcr-oracle
Supplements:    (systemd-boot and shim)
Requires:       (%{name}-snapper if (snapper and btrfsprogs))
ExclusiveArch:  aarch64 ppc64le riscv64 x86_64

%description
Hook scripts to install shim along with systemd-boot

%package snapper
Summary:        plugin script for snapper
Requires:       %{name} = %{version}
Requires:       btrfsprogs
Requires:       sdbootutil >= %{version}-%{release}
Requires:       snapper

%description snapper
Plugin scripts for snapper to handle BLS config files

%package rpm-scriptlets
Summary:        Scripts to create boot entries on kernel updates
Requires:       sdbootutil >= %{version}-%{release}
# make sure to not replace scriptlets with nops on systems that
# use grub2
Conflicts:      grub2
Conflicts:      suse-kernel-rpm-scriptlets
Provides:       suse-kernel-rpm-scriptlets
Obsoletes:      %{name}-filetriggers < %{version}

%description rpm-scriptlets
Scriptlets that call sdbootutil to create boot entries when
kernels are installed or removed

%prep
%setup -q

%build

%install
install -D -m 755 sdbootutil %{buildroot}%{_bindir}/sdbootutil
%ifarch aarch64
    sed -i -e 's/^image=vmlinuz/image=Image/' %{buildroot}%{_bindir}/sdbootutil
%endif

mkdir -p %{buildroot}%{_prefix}/lib/module-init-tools/kernel-scriptlets
for a in rpm; do
    install -m 755 "$a-script" %{buildroot}%{_prefix}/lib/module-init-tools/kernel-scriptlets
    for b in post posttrans postun pre preun; do
       ln -s "$a-script" %{buildroot}%{_prefix}/lib/module-init-tools/kernel-scriptlets/$a-$b
    done
done
for a in cert inkmp kmp; do
    for b in post posttrans postun pre preun; do
       ln -s /bin/true %{buildroot}%{_prefix}/lib/module-init-tools/kernel-scriptlets/$a-$b
    done
done

# snapper
install -d -m755 %{buildroot}%{_prefix}/lib/snapper/plugins
for i in 10-sdbootutil.snapper; do
  install -m 755 $i %{buildroot}%{_prefix}/lib/snapper/plugins/$i
done

# XXX: temporary for dracut
install -D -m 644 90-dracut-tpm-pcm-signature.conf %{buildroot}/usr/lib/dracut/dracut.conf.d/90-dracut-tpm-pcm-signature.conf

%transfiletriggerin -- /usr/lib/systemd/boot/efi /usr/share/efi/%_build_arch
cat > /dev/null || :
[ "$YAST_IS_RUNNING" != 'instsys' ] || exit 0
[ -e /sys/firmware/efi/efivars ] || exit 0
[ -z "$TRANSACTIONAL_UPDATE" ] || exit 0
[ -z "$VERBOSE_FILETRIGGERS" ] || echo "%{name}-%{version}-%{release}: updating bootloader"
sdbootutil update

%files
%license LICENSE
%{_bindir}/sdbootutil
# XXX: temporary
%dir /usr/lib/dracut
%dir /usr/lib/dracut/dracut.conf.d
/usr/lib/dracut/dracut.conf.d/90-dracut-tpm-pcm-signature.conf

%files rpm-scriptlets
%dir %{_prefix}/lib/module-init-tools
%{_prefix}/lib/module-init-tools/*

%files snapper
%dir %{_prefix}/lib/snapper
%dir %{_prefix}/lib/snapper/plugins
%{_prefix}/lib/snapper/plugins/*

%changelog
