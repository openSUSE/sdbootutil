#!/bin/bash

# Prerequisite check(s) for module.
check() {
    # Return 255 to only include the module, if another module
    # requires it.
    if [ -n "$hostonly" ]; then
        # The module does two jobs, and /etc/crypttab decides if
        # either is needed.  The generator serializes the unlocks that
        # share a FIDO2 key, which has nothing to do with the TPM2,
        # and the validator has nothing to check without a device that
        # measures PCR 15
        if ! grep -qs "fido2-device=\|tpm2-measure-pcr=yes" /etc/crypttab; then
            return 255
        fi
    fi

    return 0
}

install() {
    inst_multiple grep openssl
    inst_script "$moddir/measure-pcr-generator.sh" "/usr/lib/systemd/system-generators/measure-pcr-generator"
    inst_script "$moddir/measure-pcr-validator.sh" "/usr/bin/measure-pcr-validator"
    inst_simple "$moddir/measure-pcr-validator.service" "$systemdsystemunitdir/measure-pcr-validator.service"
    [ -f "/var/lib/sdbootutil/measure-pcr-public.pem" ] && inst_simple "/var/lib/sdbootutil/measure-pcr-public.pem"
    $SYSTEMCTL -q --root "$initdir" enable measure-pcr-validator.service
}
