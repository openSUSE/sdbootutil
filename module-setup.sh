#!/bin/bash

# Prerequisite check(s) for module.
check() {
    # Return 255 to only include the module, if another module requires it.
    return 0
}

depends() {
    return 0
}

install() {
    inst_script "$moddir/measure-pcr-generator.sh" "/usr/lib/systemd/system-generators/measure-pcr-generator"
}
