#!/bin/bash

# Prerequisite check(s) for module.
check() {
    # Return 255 to only include the module, if another module
    # requires it.
    if [ -n "$hostonly" ]; then
        # The module does two jobs, and /etc/crypttab decides if
        # either is needed.  The generator serializes the unlocks that
        # share a FIDO2 key, which has nothing to do with the TPM2,
        # and the end of the initrd is only measured for a TPM2 policy
        # that includes PCR 15
        if ! grep -qs "fido2-device=\|tpm2-measure-pcr=yes" /etc/crypttab; then
            return 255
        fi
    fi

    return 0
}

depends() {
    # Measures the volume keys (since systemd 262) and the end of
    # the initrd
    echo systemd-pcrextend
    return 0
}

install() {
    inst_script "$moddir/measure-pcr-generator.sh" "/usr/lib/systemd/system-generators/measure-pcr-generator"
    inst_simple "$moddir/measure-pcr-pcrextend.conf" "$systemdsystemunitdir/systemd-pcrextend.socket.d/50-measure-pcr.conf"
    inst_simple "$moddir/measure-pcr-boundary.service" "$systemdsystemunitdir/measure-pcr-boundary.service"
    $SYSTEMCTL -q --root "$initdir" enable measure-pcr-boundary.service
}
