#!/bin/bash

# Prerequisite check(s) for module.
check() {
    # Return 255 to only include the module, if another module
    # requires it.
    return 0
}

depends() {
    return 0
}

installkernel() {
    inst_multiple grep openssl
}

install() {
    inst_script "$moddir/measure-pcr-generator.sh" "/usr/lib/systemd/system-generators/measure-pcr-generator"
    inst_script "$moddir/measure-pcr-validator.sh" "/usr/bin/measure-pcr-validator"
    inst_simple "$moddir/measure-pcr-validator.service" "$systemdsystemunitdir/measure-pcr-validator.service"
    [ -f "/var/lib/sdbootutil/measure-pcr-public.pem" ] && inst_simple "/var/lib/sdbootutil/measure-pcr-public.pem"
    $SYSTEMCTL -q --root "$initdir" enable measure-pcr-validator.service
}
