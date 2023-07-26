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
Requires:       systemd-boot
Requires:       jq
Requires:       sed
Supplements:    (systemd-boot and shim)

%description
Hook scripts to install shim along with systemd-boot

%package snapper
Summary:        plugin script for snapper
Requires:       %{name} = %{version}
Requires:       btrfsprogs
Requires:       sdbootutil >= %{version}-%{release}
Requires:       snapper
Supplements:    (snapper and btrfsprogs)

%description snapper
Plugin scripts for snapper to handle BLS config files

%package rpm-scriptlets
Summary:        dummy scriptlets for the kernel
# make sure to not replace scriptlets with nops on systems that
# use grub2
Conflicts:      grub2
Conflicts:      suse-kernel-rpm-scriptlets
Provides:       suse-kernel-rpm-scriptlets
Obsoletes:      %{name}-filetriggers < %{version}

%description rpm-scriptlets
Empty scriptlets to satisfy kernel dependencies

%prep
%setup -q

%build

%install
install -D -m 755 sdbootutil %{buildroot}%{_bindir}/sdbootutil

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

%changelog
