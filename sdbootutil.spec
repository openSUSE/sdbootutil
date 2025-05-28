#
# spec file for package sdbootutil
#
# Copyright (c) 2025 SUSE LLC
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


%global rustflags '-Clink-arg=-Wl,-z,relro,-z,now'
Name:           sdbootutil
Version:        0
Release:        0
Summary:        bootctl wrapper for BLS boot loaders
License:        MIT
URL:            https://github.com/openSUSE/sdbootutil
Source:         %{name}-%{version}.tar
Source1:        vendor.tar.zst
Source2:        config.toml
BuildRequires:  cargo
BuildRequires:  cargo-packaging
BuildRequires:  libopenssl-devel
BuildRequires:  systemd-rpm-macros
Requires:       %{name}-dracut-measure-pcr
Requires:       dialog
Requires:       dracut-pcr-signature
Requires:       efibootmgr
Requires:       jq
Requires:       keyutils
Requires:       openssl
Requires:       pcr-oracle
Requires:       qrencode
Requires:       sed
Requires:       (%{name}-snapper if (snapper and btrfsprogs))
Requires:       (%{name}-tukit if read-only-root-fs)
# While systemd-pcrlock is in experimental
Requires:       systemd-experimental
# something needs to require it. Can be us.
Requires:       tpm2.0-tools
# While bootctl is in udev
Requires:       udev
Supplements:    (grub2-x86_64-efi-bls and shim)
Supplements:    (systemd-boot and shim)
BuildArch:      noarch
ExclusiveArch:  aarch64 riscv64 x86_64
%{?systemd_requires}

%description
bootctl wrapper for BLS boot loaders, like systemd-boot and grub2-bls.
Implements also the life cycle of a full disk encryption installation,
based on systemd.

%package snapper
Summary:        plugin script for snapper
Requires:       %{name} = %{version}
Requires:       btrfsprogs
Requires:       snapper
BuildArch:      noarch

%description snapper
Plugin scripts for snapper to handle BLS config files

%package tukit
Summary:        plugin script for tukit
Requires:       %{name} = %{version}
Requires:       tukit
BuildArch:      noarch

%description tukit
Plugin scripts for tukit to handle BLS config files

%package kernel-install
Summary:        Hook script for kernel-install
Requires:       %{name} = %{version}
# While kernel-install is in udev
Requires:       udev
BuildArch:      noarch

%description kernel-install
Plugin script for kernel-install. Note: installation of this
package may disable other plugin scripts that are incompatible.

%package enroll
Summary:        Full disk encryption enrollment
Requires:       %{name} = %{version}
BuildArch:      noarch

%description enroll
Systemd service and script for full disk encryption enrollment.

%package jeos-firstboot-enroll
Summary:        JEOS module for full disk encryption enrollment
Requires:       %{name} = %{version}
Requires:       %{name}-enroll = %{version}
Requires:       jeos-firstboot
BuildArch:      noarch

%description jeos-firstboot-enroll
JEOS module for full disk encryption enrollment. The module
present the different options and delegate into sdbootutil-enroll
service the effective enrollment.

%package bash-completion
Summary:        Bash completions for sdbootutil
Requires:       %{name} = %{version}
Requires:       bash
Requires:       bash-completion
BuildArch:      noarch

%description bash-completion
Bash completions script for sdbootutil.
Allows the user to press TAB to see available commands,
options and parameters.

%package dracut-measure-pcr
Summary:        Dracut module to measure PCR 15
BuildRequires:  pkgconfig
BuildRequires:  rpm-config-SUSE
BuildRequires:  pkgconfig(dracut)
BuildArch:      noarch

%description dracut-measure-pcr
Dracut module from sdbootutil to measure PCR 15 in non-UKIs systems

%prep
%autosetup -a1 -p1
mv vendor uhmac
cd uhmac
mkdir .cargo
install -D -m 644 %{SOURCE2} .cargo/config.toml

%build
cd uhmac
sed -i 's/edition = "2024"/edition = "2018"/' Cargo.toml
%{cargo_build}

%install
install -D -m 755 %{name} %{buildroot}%{_bindir}/%{name}
install -D -m 755 uhmac/target/release/uhmac %{buildroot}%{_libexecdir}/%{name}/uhmac

# Update prediction service
install -D -m 644 %{name}-update-predictions.service \
	%{buildroot}%{_unitdir}/%{name}-update-predictions.service

# Enrollment service
install -m 755 %{name}-enroll %{buildroot}%{_bindir}/%{name}-enroll
install -D -m 644 %{name}-enroll.service %{buildroot}/%{_unitdir}/%{name}-enroll.service

# Jeos module
install -D -m 644 jeos-firstboot-enroll-override.conf \
	%{buildroot}%{_prefix}/lib/systemd/system/jeos-firstboot.service.d/jeos-firstboot-enroll-override.conf
install -D -m 644 jeos-firstboot-enroll %{buildroot}%{_datadir}/jeos-firstboot/modules/enroll

# Snapper
install -D -m 755 10-%{name}.snapper %{buildroot}%{_prefix}/lib/snapper/plugins/10-%{name}.snapper

# Tukit
install -D -m 755 10-%{name}.tukit %{buildroot}%{_prefix}/lib/tukit/plugins/10-%{name}.tukit

# kernel-install
install -D -m 755 50-%{name}.install %{buildroot}%{_prefix}/lib/kernel/install.d/50-%{name}.install

# Bash completions
install -D -m 755 completions/bash_sdbootutil %{buildroot}%{_datadir}/bash-completion/completions/sdbootutil

# Dracut module
install -D -m 755 module-setup.sh %{buildroot}%{_prefix}/lib/dracut/modules.d/50measure-pcr/module-setup.sh
install -D -m 755 measure-pcr-generator.sh %{buildroot}%{_prefix}/lib/dracut/modules.d/50measure-pcr/measure-pcr-generator.sh
install -D -m 755 measure-pcr-validator.sh %{buildroot}%{_prefix}/lib/dracut/modules.d/50measure-pcr/measure-pcr-validator.sh
install -D -m 644 measure-pcr-validator.service %{buildroot}/%{_prefix}/lib/dracut/modules.d/50measure-pcr/measure-pcr-validator.service

install -d -m 700 %{buildroot}%{_sharedstatedir}/%{name}

# tmpfiles
install -D -m 755 kernel-install-%{name}.conf \
	%{buildroot}%{_prefix}/lib/tmpfiles.d/kernel-install-%{name}.conf

%transfiletriggerin -- %{_prefix}/lib/systemd/boot/efi %{_datadir}/grub2/%{_build_arch}-efi %{_datadir}/efi/%{_build_arch}
cat > /dev/null || :
[ "$YAST_IS_RUNNING" != 'instsys' ] || exit 0
[ -e /sys/firmware/efi/efivars ] || exit 0
[ -z "$TRANSACTIONAL_UPDATE" ] || exit 0
[ -z "$VERBOSE_FILETRIGGERS" ] || echo "%{name}-%{version}-%{release}: updating bootloader"
if [ -e /etc/sysconfig/bootloader ]; then
	. /etc/sysconfig/bootloader &> /dev/null
	if [ "$LOADER_TYPE" = "grub2-bls" ] || [ "$LOADER_TYPE" = "systemd-boot" ]; then
		sdbootutil update
	fi
else
	sdbootutil update
fi

%preun
%service_del_preun %{name}-update-predictions.service

%postun
%service_del_postun %{name}-update-predictions.service

%pre
%service_add_pre %{name}-update-predictions.service

%post
%service_add_post %{name}-update-predictions.service

%preun enroll
%service_del_preun %{name}-enroll.service

%postun enroll
%service_del_postun %{name}-enroll.service

%pre enroll
%service_add_pre %{name}-enroll.service

%post enroll
%service_add_post %{name}-enroll.service

%posttrans kernel-install
%tmpfiles_create kernel-install-%{name}.conf

%post dracut-measure-pcr
%{?regenerate_initrd_post}

%posttrans dracut-measure-pcr
%{?regenerate_initrd_posttrans}

%postun dracut-measure-pcr
%{?regenerate_initrd_post}

%files
%license LICENSE
%dir %{_sharedstatedir}/%{name}
%{_bindir}/%{name}
%{_unitdir}/%{name}-update-predictions.service
%dir %{_libexecdir}/%{name}
%{_libexecdir}/%{name}/uhmac

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
%{_prefix}/lib/tmpfiles.d/kernel-install-%{name}.conf

%files enroll
%{_bindir}/%{name}-enroll
%{_unitdir}/%{name}-enroll.service

%files jeos-firstboot-enroll
%dir %{_datadir}/jeos-firstboot
%dir %{_datadir}/jeos-firstboot/modules
%{_datadir}/jeos-firstboot/modules/enroll
%dir %{_unitdir}/jeos-firstboot.service.d
%{_unitdir}/jeos-firstboot.service.d/jeos-firstboot-enroll-override.conf

%files bash-completion
%dir %{_datadir}/bash-completion
%dir %{_datadir}/bash-completion/completions
%{_datadir}/bash-completion/completions/sdbootutil

%files dracut-measure-pcr
%dir %{_prefix}/lib/dracut
%dir %{_prefix}/lib/dracut/modules.d
%{_prefix}/lib/dracut/modules.d/50measure-pcr

%changelog
