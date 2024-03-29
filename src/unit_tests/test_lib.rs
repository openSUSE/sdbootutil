use super::super::*;
use std::env::consts::ARCH;
use std::fs;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_command_kernels() {
    let result = command_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_snapshots() {
    let result = command_snapshots().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_entries() {
    let result = command_entries().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_bootloader_only_systemd_boot_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let sdboot_efi_path = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    fs::create_dir_all(sdboot_efi_path.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    File::create(&sdboot_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert!(command_bootloader(Some(0), "x64", Some(temp_dir.path())).unwrap());
}

#[test]
fn test_command_bootloader_systemd_boot_and_grub2_exist() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let sdboot_efi_path = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    let grub2_efi_path = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));

    fs::create_dir_all(sdboot_efi_path.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    fs::create_dir_all(grub2_efi_path.parent().unwrap())
        .expect("Failed to create directory for GRUB2 EFI file");

    File::create(&sdboot_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();
    File::create(&grub2_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert!(!command_bootloader(Some(0), "x64", Some(temp_dir.path())).unwrap());
}

#[test]
fn test_command_bootloader_only_grub2_exist() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2_efi_path = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));

    fs::create_dir_all(grub2_efi_path.parent().unwrap())
        .expect("Failed to create directory for GRUB2 EFI file");

    File::create(&grub2_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert!(!command_bootloader(Some(0), "x64", Some(temp_dir.path())).unwrap());
}

#[test]
fn test_bootloader_name_neither_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    assert!(command_bootloader(Some(0), "x64", Some(temp_dir.path())).is_err());
}

#[test]
fn test_command_add_kernel() {
    let result = command_add_kernel("5.8.0-53-generic").unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_add_all_kernels() {
    let result = command_add_all_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_mkinitrd() {
    let result = command_mkinitrd().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_remove_kernel() {
    let result = command_remove_kernel("5.8.0-53-generic").unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_remove_all_kernels() {
    let result = command_remove_all_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_list_kernels() {
    let result = command_list_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_list_entries() {
    let result = command_list_entries().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_list_snapshots() {
    let result = command_list_snapshots().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_set_default_snapshot() {
    let result = command_set_default_snapshot().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_is_bootable() {
    let result = command_is_bootable().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_is_installed_true() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let systemd_boot_path = snapshot_dir.join("EFI").join("systemd");
    let is_installed_file_path = temp_dir.path().join("installed_by_sdbootutil");
    fs::create_dir_all(&is_installed_file_path.parent().unwrap())
        .expect("Failed to create installed path");
    fs::create_dir_all(&systemd_boot_path).expect("Failed to create systemd-boot path");
    let systemd_boot_test_file = systemd_boot_path.join("systemd-bootx64.efi");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &systemd_boot_test_file,
    )
    .expect("Failed to copy systemd-boot test file");
    File::create(&is_installed_file_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    let is_installed = command_is_installed(
        Some(0),
        "",
        "",
        "",
        "",
        Some(systemd_boot_test_file),
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(is_installed, true, "Expected is_installed to return true")
}

#[test]
fn test_command_is_installed_false_bootloader() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let systemd_boot_path = snapshot_dir.join("EFI").join("systemd");
    let is_installed_file_path = temp_dir.path().join("installed_by_sdbootutil");
    fs::create_dir_all(&is_installed_file_path.parent().unwrap())
        .expect("Failed to create installed path");
    fs::create_dir_all(&systemd_boot_path).expect("Failed to create systemd-boot path");
    let systemd_boot_test_file = systemd_boot_path.join("systemd-bootx64.efi");
    File::create(&systemd_boot_test_file)
        .unwrap()
        .write_all(b"")
        .unwrap();
    File::create(&is_installed_file_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    let is_installed = command_is_installed(
        Some(0),
        "",
        "",
        "",
        "",
        Some(systemd_boot_test_file),
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(is_installed, false, "Expected is_installed to return false")
}

#[test]
fn test_command_is_installed_false_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let systemd_boot_path = snapshot_dir.join("EFI").join("systemd");
    let is_installed_file_path = temp_dir.path().join("installed_by_sdbootutil");
    fs::create_dir_all(&is_installed_file_path.parent().unwrap())
        .expect("Failed to create installed path");
    fs::create_dir_all(&systemd_boot_path).expect("Failed to create systemd-boot path");
    let systemd_boot_test_file = systemd_boot_path.join("systemd-bootx64.efi");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &systemd_boot_test_file,
    )
    .expect("Failed to copy systemd-boot test file");

    let is_installed = command_is_installed(
        Some(0),
        "",
        "",
        "",
        "",
        Some(systemd_boot_test_file),
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(is_installed, false, "Expected is_installed to return false")
}

#[test]
fn test_command_install() {
    let result = command_install().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_needs_update_no_systemd_boot() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let systemd_boot_test_file = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    let systemd_boot_efi_file = temp_dir
        .path()
        .join("boot/efi/EFI/systemd/systemd-bootx64.efi");
    fs::create_dir_all(systemd_boot_test_file.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    fs::create_dir_all(systemd_boot_efi_file.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &systemd_boot_test_file,
    )
    .expect("Failed to copy systemd-boot test file");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &systemd_boot_efi_file,
    )
    .expect("Failed to copy systemd-boot efi file");

    let needs_update = command_needs_update(
        Some(0),
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        Some(temp_dir.path()),
    )
    .unwrap();
    assert!(!needs_update);
}

#[test]
fn test_command_needs_update_no_grub2() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2 = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));
    let grub2_efi_file = temp_dir.path().join("boot/efi/EFI/systemd/grub.efi");
    fs::create_dir_all(grub2.parent().unwrap())
        .expect("Failed to create directory for grub2 EFI file");
    fs::create_dir_all(grub2_efi_file.parent().unwrap())
        .expect("Failed to create directory for grub2 EFI file");
    fs::copy(PathBuf::from("src/unit_tests/test_files/grub2.efi"), &grub2)
        .expect("Failed to copy grub2 test file");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/grub2.efi"),
        &grub2_efi_file,
    )
    .expect("Failed to copy grub2 efi file");

    let needs_update = command_needs_update(
        Some(0),
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        Some(temp_dir.path()),
    )
    .unwrap();
    assert!(!needs_update);
}

#[test]
fn test_command_needs_update_shim_systemd_boot() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let systemd_boot_test_file = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    let systemd_boot_efi_file = temp_dir
        .path()
        .join("boot/efi/EFI/systemd/systemd-bootx64.efi");
    fs::create_dir_all(systemd_boot_test_file.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    fs::create_dir_all(systemd_boot_efi_file.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &systemd_boot_test_file,
    )
    .expect("Failed to copy systemd-boot test file");
    let shim_test_file = temp_dir.path().join("usr/share/efi/x86_64/shim.efi");
    let shim_efi_file = temp_dir.path().join("boot/efi/EFI/systemd/grub.efi");
    fs::create_dir_all(shim_test_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    fs::create_dir_all(shim_efi_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    File::create(&shim_test_file)
        .unwrap()
        .write_all(b"")
        .unwrap();
    File::create(&shim_efi_file)
        .unwrap()
        .write_all(b"#### LoaderInfo: systemd-boot 253.4+suse.17.gbe772961ad ####")
        .unwrap();

    let needs_update = command_needs_update(
        Some(0),
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        Some(temp_dir.path()),
    )
    .unwrap();
    assert!(needs_update);
}

#[test]
fn test_command_needs_update_no_shim_systemd_boot() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let systemd_boot_test_file = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    let systemd_boot_efi_file = temp_dir
        .path()
        .join("boot/efi/EFI/systemd/systemd-bootx64.efi");
    fs::create_dir_all(systemd_boot_test_file.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    fs::create_dir_all(systemd_boot_efi_file.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &systemd_boot_test_file,
    )
    .expect("Failed to copy systemd-boot test file");
    let shim_test_file = temp_dir.path().join("usr/share/efi/x86_64/shim.efi");
    let shim_efi_file = temp_dir.path().join("boot/efi/EFI/systemd/grub.efi");
    fs::create_dir_all(shim_test_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    fs::create_dir_all(shim_efi_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    File::create(&shim_test_file)
        .unwrap()
        .write_all(b"")
        .unwrap();
    File::create(&shim_efi_file)
        .unwrap()
        .write_all(b"#### LoaderInfo: systemd-boot 256.4+suse.17.gbe772961ad ####")
        .unwrap();

    let needs_update = command_needs_update(
        Some(0),
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        Some(temp_dir.path()),
    )
    .unwrap();
    assert!(!needs_update);
}

#[test]
fn test_command_needs_update_shim_grub2() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2 = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));
    let grub2_efi_file = temp_dir.path().join("boot/efi/EFI/systemd/grub.efi");
    fs::create_dir_all(grub2.parent().unwrap())
        .expect("Failed to create directory for grub2 EFI file");
    fs::create_dir_all(grub2_efi_file.parent().unwrap())
        .expect("Failed to create directory for grub2 EFI file");
    fs::copy(PathBuf::from("src/unit_tests/test_files/grub2.efi"), &grub2)
        .expect("Failed to copy grub2 test file");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/grub2.efi"),
        &grub2_efi_file,
    )
    .expect("Failed to copy grub2 efi file");
    let shim_test_file = temp_dir.path().join("usr/share/efi/x86_64/shim.efi");
    let shim_efi_file = temp_dir.path().join("boot/efi/EFI/systemd/grub.efi");
    fs::create_dir_all(shim_test_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    fs::create_dir_all(shim_efi_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    File::create(&shim_test_file)
        .unwrap()
        .write_all(b"")
        .unwrap();
    File::create(&shim_efi_file)
        .unwrap()
        .write_all(b"GNU GRUB  version %s\x002.10\x00prefixESC at any time exits.")
        .unwrap();

    let needs_update = command_needs_update(
        Some(0),
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        Some(temp_dir.path()),
    )
    .unwrap();
    assert!(needs_update);
}

#[test]
fn test_command_needs_update_no_shim_grub2() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2 = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));
    let grub2_efi_file = temp_dir.path().join("boot/efi/EFI/systemd/grub.efi");
    fs::create_dir_all(grub2.parent().unwrap())
        .expect("Failed to create directory for grub2 EFI file");
    fs::create_dir_all(grub2_efi_file.parent().unwrap())
        .expect("Failed to create directory for grub2 EFI file");
    fs::copy(PathBuf::from("src/unit_tests/test_files/grub2.efi"), &grub2)
        .expect("Failed to copy grub2 test file");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/grub2.efi"),
        &grub2_efi_file,
    )
    .expect("Failed to copy grub2 efi file");
    let shim_test_file = temp_dir.path().join("usr/share/efi/x86_64/shim.efi");
    let shim_efi_file = temp_dir.path().join("boot/efi/EFI/systemd/grub.efi");
    fs::create_dir_all(shim_test_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    fs::create_dir_all(shim_efi_file.parent().unwrap())
        .expect("Failed to create directory for shim EFI file");
    File::create(&shim_test_file)
        .unwrap()
        .write_all(b"")
        .unwrap();
    File::create(&shim_efi_file)
        .unwrap()
        .write_all(b"GNU GRUB  version %s\x002.13\x00prefixESC at any time exits.")
        .unwrap();

    let needs_update = command_needs_update(
        Some(0),
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        Some(temp_dir.path()),
    )
    .unwrap();
    assert!(!needs_update);
}

#[test]
fn test_command_update() {
    let result = command_update().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_force_update() {
    let result = command_force_update().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_update_predictions() {
    let result = command_update_predictions().unwrap();
    assert_eq!(result, true);
}
