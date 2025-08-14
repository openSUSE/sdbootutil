#!/bin/bash
set -euo pipefail

WHITE="\e[1;37m"
LIGHT_BLUE="\e[1;34m"
END="\e[m"

measure_pcr_crypttab() {
	grep -q "tpm2-measure-pcr=yes" /etc/crypttab
}

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

exit_with_msg() {
	local msg="$1"

	if ! measure_pcr_crypttab; then
		echo "INFO: No PCR 15 validation"

		exit 0
	elif get_measure_pcr_ignore; then
		echo "WARNING: The validation of PCR 15 failed"
		echo "WARNING: $msg"

		exit 0
	else
		echo "ERROR: the validation of PCR 15 failed"

		kill -SIGRTMIN+21 1
		sleep 1
		echo -ne '\n\n\a'
		echo -e "${WHITE}*********************************************************************${END}"
		echo -e "${WHITE}ERROR: $msg${END}"
		echo -e "${WHITE}Use${END} '${LIGHT_BLUE}measure-pcr-validator.ignore=yes${END}' ${WHITE}in cmdline to bypass the check${END}"
		echo -e "${WHITE}*********************************************************************${END}"
		echo
		read -n1 -s -r -t 10 -p $'\e[1;37m*** The system will be halted. Press any key ...\e[0m' || true
		echo
		kill -SIGRTMIN+20 1

		exit 1
	fi
}

[ -f /etc/crypttab ] || exit 0
grep -q 'tpm2-measure-pcr=yes' /etc/crypttab || exit 0
[ -f "/var/lib/sdbootutil/measure-pcr-prediction" ] || exit_with_msg "Missing measure-pcr-prediction file"
[ -f "/var/lib/sdbootutil/measure-pcr-prediction.sha256" ] || exit_with_msg "Missing measure-pcr-prediction.sha256 signature file"
validate_measure_pcr_signature || exit_with_msg "Signature for the prediction file is not valid"
validate_measure_pcr || exit_with_msg "PCR 15 mismatch. Encrypted devices compromised"
