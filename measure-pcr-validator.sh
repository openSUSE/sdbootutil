#!/bin/bash
set -euo pipefail

get_measure_pcr_ignore() {
	(set +eu; . /lib/dracut-lib.sh; getargbool no measure-pcr-validator.ignore)
}

validate_measure_pcr_signature() {
	openssl dgst -sha256 \
		-verify /var/lib/sdbootutil/measure-pcr-public.pem \
		-signature /var/lib/sdbootutil/measure-pcr-prediction.sha256 \
		/var/lib/sdbootutil/measure-pcr-prediction &> /dev/null
}

validate_measure_pcr() {
	if [ -f "/var/lib/sdbootutil/measure-pcr-prediction.sha256" ] && \
		   [ -f "/var/lib/sdbootutil/measure-pcr-public.pem" ]; then
		if ! validate_measure_pcr_signature; then
			echo "Error: the signature for the prediction file is not valid"
			return 1
		fi
	else
		echo "Warning: the signature for the prediction file is missing"
	fi

	if [ ! -e "/sys/class/tpm/tpm0" ]; then
		echo "Error: TPM2 not found in /sys/class/tpm/tpm0"
		return 1
	fi

	local res=1
	for sha in sha1 sha256 sha384 sha512; do
		[ -e "/sys/class/tpm/tpm0/pcr-$sha/15" ] || continue
		read -r expected_pcr_15 < "/sys/class/tpm/tpm0/pcr-$sha/15"
		grep -Fixq "$expected_pcr_15" /var/lib/sdbootutil/measure-pcr-prediction; res="$?"
		break
	done

	return "$res"
}

# The measure-pcr-prediction file contain a list of hashes (sha1,
# sha256, ...)
if [ -f "/var/lib/sdbootutil/measure-pcr-prediction" ] && ! validate_measure_pcr; then
	if get_measure_pcr_ignore; then
		echo "Warning: the validation of PCR 15 failed. Continuing the boot process"
	else
		echo "Error: the validation of PCR 15 failed"
		exit 1
	fi
fi
