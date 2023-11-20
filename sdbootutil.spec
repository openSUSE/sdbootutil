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
Requires:       jq
Requires:       sed
Requires:       systemd-boot
Supplements:    (systemd-boot and shim)
Requires:       (%{name}-snapper if (snapper and btrfsprogs))

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

%package systemd-boot-update
Summary:        Update the EFI binaries when there is a new version
Requires:       sdbootutil >= %{version}-%{release}
BuildRequires:  systemd-rpm-macros

%description systemd-boot-update
Update systemd-bootx64.efi, BOOTX64.EFI and grub.efi if there is a
new systemd-boot version

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

# systemd-boot-update
install -D -m 644 systemd-boot-update-grub.service %{buildroot}%{_unitdir}/systemd-boot-update-grub.service
install -d -m755 %{buildroot}%{_unitdir}/multi-user.target.wants
ln -sr %{buildroot}%{_unitdir}/systemd-boot-update-grub.service %{buildroot}%{_unitdir}/multi-user.target.wants
# Create an empty file before doing the link
touch %{buildroot}%{_unitdir}/systemd-boot-update.service
ln -sr %{buildroot}%{_unitdir}/systemd-boot-update.service %{buildroot}%{_unitdir}/multi-user.target.wants

%pre systemd-boot-update
%service_add_pre systemd-boot-update-grub.service

%post systemd-boot-update
%service_add_post systemd-boot-update-grub.service

%preun systemd-boot-update
%service_del_preun systemd-boot-update-grub.service

%postun systemd-boot-update
%service_del_postun systemd-boot-update-grub.service

%files
%license LICENSE
%{_bindir}/sdbootutil

%files rpm-scriptlets
%dir %{_prefix}/lib/module-init-tools
%{_prefix}/lib/module-init-tools/*

%files snapper
%dir %{_prefix}/lib/snapper
%dir %{_prefix}/lib/snapper/plugins
%{_prefix}/lib/snapper/plugins/*

%files systemd-boot-update
%dir %{_unitdir}/multi-user.target.wants
%{_unitdir}/multi-user.target.wants/systemd-boot-update.service
%{_unitdir}/multi-user.target.wants/systemd-boot-update-grub.service
%ghost %{_unitdir}/systemd-boot-update.service
%{_unitdir}/systemd-boot-update-grub.service

%changelog
