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
Version:        1%{git_version}
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
Requires:       dialog
# While systemd-pcrlock is in experimental
Requires:       systemd-experimental
Requires:       dracut-pcr-signature
Supplements:    (systemd-boot and shim)
Requires:       (%{name}-snapper if (snapper and btrfsprogs))
Requires:       (%{name}-tukit if read-only-root-fs)
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

%package tukit
Summary:        plugin script for tukit
Requires:       %{name} = %{version}
Requires:       sdbootutil >= %{version}-%{release}
Requires:       tukit

%description tukit
Plugin scripts for tukit to handle BLS config files

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

%package kernel-install
Summary:        Hook script for kernel-install
Requires:       sdbootutil >= %{version}-%{release}
Requires:       /usr/bin/kernel-install

%description kernel-install
Plugin script for kernel-install. Note: installation of this
package may disable other plugin scripts that are incompatible.

%prep
%setup -q

%build

%install
install -D -m 755 sdbootutil %{buildroot}%{_bindir}/sdbootutil

# services
for i in sdbootutil-update-predictions.service; do
	install -D -m 644 "$i" %{buildroot}%{_unitdir}/"$i"
done

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

# tukit
install -d -m755 %{buildroot}%{_prefix}/lib/tukit/plugins
for i in 10-sdbootutil.tukit; do
  install -m 755 $i %{buildroot}%{_prefix}/lib/tukit/plugins/$i
done

# kernel-install
install -d -m755 %{buildroot}%{_prefix}/lib/kernel/install.d
for i in 50-sdbootutil.install; do
  install -m 755 $i %{buildroot}%{_prefix}/lib/kernel/install.d/$i
done

# tmpfiles
install -d -m755 %{buildroot}%{_prefix}/lib/tmpfiles.d
for i in kernel-install-sdbootutil.conf; do
  install -m 755 $i %{buildroot}%{_prefix}/lib/tmpfiles.d/$i
done
mkdir -p %{buildroot}/etc/kernel/install.d

%transfiletriggerin -- /usr/lib/systemd/boot/efi /usr/share/efi/%_build_arch
cat > /dev/null || :
[ "$YAST_IS_RUNNING" != 'instsys' ] || exit 0
[ -e /sys/firmware/efi/efivars ] || exit 0
[ -z "$TRANSACTIONAL_UPDATE" ] || exit 0
[ -z "$VERBOSE_FILETRIGGERS" ] || echo "%{name}-%{version}-%{release}: updating bootloader"
sdbootutil update

%posttrans kernel-install
%tmpfiles_create kernel-install-sdbootutil.conf

%files
%license LICENSE
%{_bindir}/sdbootutil
%{_unitdir}/sdbootutil-update-predictions.service

%files rpm-scriptlets
%dir %{_prefix}/lib/module-init-tools
%{_prefix}/lib/module-init-tools/*

%files snapper
%dir %{_prefix}/lib/snapper
%dir %{_prefix}/lib/snapper/plugins
%{_prefix}/lib/snapper/plugins/*

%files tukit
%dir %{_prefix}/lib/tukit
%dir %{_prefix}/lib/tukit/plugins
%{_prefix}/lib/tukit/plugins/*

%files kernel-install
%dir %{_prefix}/lib/kernel
%dir %{_prefix}/lib/kernel/install.d
%{_prefix}/lib/kernel/install.d/*
%{_prefix}/lib/tmpfiles.d/kernel-install-sdbootutil.conf
%dir /etc/kernel
%dir /etc/kernel/install.d
%ghost %config(noreplace,missingok) /etc/kernel/install.d/50-depmod.install
%ghost %config(noreplace,missingok) /etc/kernel/install.d/50-dracut.install
%ghost %config(noreplace,missingok) /etc/kernel/install.d/51-dracut-rescue.install
%ghost %config(noreplace,missingok) /etc/kernel/install.d/90-loaderentry.install

%changelog
