use super::super::fs::*;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use tempfile::TempDir;

// Helper function to create a backup file
fn create_backup_file(temp_dir_path: &PathBuf, filename: &str) -> std::io::Result<PathBuf> {
    let backup_path = temp_dir_path.join(format!("{}.bak", filename));
    let mut backup_file = File::create(&backup_path)?;
    writeln!(backup_file, "Backup content")?;
    Ok(backup_path)
}

// Helper function to create an original file (that could be deleted during cleanup)
fn create_original_file(temp_dir_path: &PathBuf, filename: &str) -> std::io::Result<PathBuf> {
    let original_path = temp_dir_path.join(filename);
    let mut original_file = File::create(&original_path)?;
    writeln!(original_file, "Original content")?;
    Ok(original_path)
}

#[test]
fn test_restore_from_backup() {
    let (_temp_dir, temp_dir_path) = create_temp_dir();

    // Use the helper function to create the original file and its backup
    let original_file_path = temp_dir_path.join("testfile");
    create_backup_file(&temp_dir_path, "testfile").unwrap();

    let rollback_item = RollbackItem::new(original_file_path.clone());

    // Perform cleanup
    rollback_item.cleanup().unwrap();

    assert!(
        original_file_path.exists(),
        "Original file should have been restored from backup"
    );
}

#[test]
fn test_remove_original_no_backup() {
    let (_temp_dir, temp_dir_path) = create_temp_dir();

    // Use the helper function to create only the original file without a backup
    let original_file_path = create_original_file(&temp_dir_path, "testfile").unwrap();

    let rollback_item = RollbackItem::new(original_file_path.clone());

    rollback_item.cleanup().unwrap();

    assert!(
        !original_file_path.exists(),
        "Original file should have been removed"
    );
}

#[test]
fn test_no_file_no_backup() {
    let (_temp_dir, temp_dir_path) = create_temp_dir();

    // Define a path for a non-existent file
    let non_existent_file_path = temp_dir_path.join("nonexistentfile");
    let rollback_item = RollbackItem::new(non_existent_file_path.clone());

    let result = rollback_item.cleanup();

    assert!(
        result.is_ok(),
        "Cleanup should not error when no files exist"
    );
}

// Test cleanup when all operations succeed
#[test]
fn test_cleanup_success() {
    let (_temp_dir, temp_dir_path) = create_temp_dir();

    // Create a dummy original file and a backup
    let original_file_path = create_original_file(&temp_dir_path, "testfile").unwrap();
    create_backup_file(&temp_dir_path, "testfile").unwrap();

    // Create a RollbackItem and perform cleanup
    let rollback_item = RollbackItem::new(original_file_path.clone());
    let rollback_items = vec![rollback_item];
    cleanup_rollback_items(&rollback_items);

    // Assert that the original file has been restored from the backup
    assert!(
        original_file_path.exists(),
        "Original file should exist after cleanup"
    );
    assert!(
        !original_file_path.with_extension("bak").exists(),
        "Backup file should not exist after cleanup"
    );
}

#[test]
fn test_cleanup_success_content() {
    let (_temp_dir, temp_dir_path) = create_temp_dir();

    // Define the content for the backup file
    let backup_content = "Backup content";

    // Create a dummy original file and a backup with specific content
    let original_file_path = create_original_file(&temp_dir_path, "testfile").unwrap();
    let backup_file_path = create_backup_file(&temp_dir_path, "testfile").unwrap();

    // Write the defined content to the backup file
    let mut backup_file = File::create(backup_file_path).unwrap();
    writeln!(backup_file, "{}", backup_content).unwrap();

    // Create a RollbackItem and perform cleanup
    let rollback_item = RollbackItem::new(original_file_path.clone());
    let rollback_items = vec![rollback_item];
    cleanup_rollback_items(&rollback_items);

    // Assert that the original file has been restored from the backup
    assert!(
        original_file_path.exists(),
        "Original file should exist after cleanup"
    );
    assert!(
        !original_file_path.with_extension("bak").exists(),
        "Backup file should not exist after cleanup"
    );

    // Read the content of the restored file
    let mut restored_content = String::new();
    let mut restored_file = File::open(original_file_path).unwrap();
    restored_file.read_to_string(&mut restored_content).unwrap();

    // Assert that the content of the restored file matches the backup content
    assert_eq!(
        restored_content.trim(),
        backup_content,
        "Restored file content should match backup content"
    );
}

#[test]
fn test_reset_rollback_items() {
    let (_temp_dir, temp_dir_path) = create_temp_dir();

    let file_names = vec!["testfile1", "testfile2"];
    let mut rollback_items = Vec::new();

    for file_name in &file_names {
        let original_file_path = create_original_file(&temp_dir_path, file_name).unwrap();
        create_backup_file(&temp_dir_path, file_name).unwrap();

        rollback_items.push(RollbackItem::new(original_file_path.clone()));
    }

    reset_rollback_items(&mut rollback_items);

    assert!(
        rollback_items.is_empty(),
        "Rollback items should be cleared after reset"
    );

    for file_name in &file_names {
        let original_file_path = temp_dir_path.join(file_name);
        assert!(
            original_file_path.exists(),
            "Original file should still exist after reset"
        );

        let backup_file_path = original_file_path.with_extension("bak");
        assert!(
            !backup_file_path.exists(),
            "Backup file should be removed after reset"
        );
    }
}

#[test]
fn test_reset_rollback_items_no_backups() {
    let (_temp_dir, temp_dir_path) = create_temp_dir();

    let file_names = vec!["testfile1", "testfile2"];
    let mut rollback_items = Vec::new();

    for file_name in &file_names {
        let original_file_path = create_original_file(&temp_dir_path, file_name).unwrap();

        rollback_items.push(RollbackItem::new(original_file_path.clone()));
    }

    reset_rollback_items(&mut rollback_items);

    assert!(
        rollback_items.is_empty(),
        "Rollback items should be cleared after reset even if no backups exist"
    );

    for file_name in &file_names {
        let original_file_path = temp_dir_path.join(file_name);
        assert!(
            original_file_path.exists(),
            "Original file should still exist after reset without backups"
        );
    }
}

#[test]
fn test_find_sdboot() {
    // Create a temporary directory to simulate the snapshot structure
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");
    fs::create_dir_all(&snapshot_dir.join("usr/lib/systemd-boot"))
        .expect("Failed to create systemd-boot path");
    fs::create_dir_all(&snapshot_dir.join("usr/lib/systemd/boot/efi"))
        .expect("Failed to create systemd-boot EFI fallback path");

    let sdboot_efi_path = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    fs::write(&sdboot_efi_path, "").expect("Failed to create dummy systemd-boot EFI file");

    let found_path = find_sdboot(Some(0), "x64", Some(temp_dir.path()));

    assert_eq!(found_path, sdboot_efi_path);
}

#[test]
fn test_find_sdboot_fallback() {
    // Create a temporary directory to simulate the snapshot structure
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("1").join("snapshot");

    // Create only the fallback directory structure without the primary EFI file
    fs::create_dir_all(&snapshot_dir.join("usr/lib/systemd-boot"))
        .expect("Failed to create systemd-boot path");
    let fallback_dir = snapshot_dir.join("usr/lib/systemd/boot/efi");
    fs::create_dir_all(&fallback_dir).expect("Failed to create systemd-boot EFI fallback path");

    // Create a dummy EFI file in the fallback location
    let fallback_efi_path = fallback_dir.join("systemd-bootx64.efi");
    File::create(&fallback_efi_path)
        .expect("Failed to create dummy systemd-boot EFI file in fallback location");

    // Call the function with the temporary directory as the prefix
    let found_path = find_sdboot(Some(1), "x64", Some(temp_dir.path()));

    // Assert that the returned path matches the fallback EFI file's path
    assert_eq!(
        found_path, fallback_efi_path,
        "The found path did not match the expected fallback path"
    );

    // TempDir is automatically cleaned up
}

#[test]
fn test_find_grub2_primary_path() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");
    fs::create_dir_all(&snapshot_dir.join(format!("usr/share/efi/{}/", ARCH)))
        .expect("Failed to create GRUB2 path");

    let grub2_efi_path = snapshot_dir.join(format!("usr/share/efi/{}/grub.efi", ARCH));
    fs::write(&grub2_efi_path, "").expect("Failed to create dummy GRUB2 EFI file");

    let found_path = find_grub2(Some(0), Some(temp_dir.path()));

    assert_eq!(found_path, grub2_efi_path);
}

#[test]
fn test_find_grub2_fallback_path() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");
    fs::create_dir_all(&snapshot_dir.join(format!("usr/share/grub2/{}-efi/", ARCH)))
        .expect("Failed to create GRUB2 fallback path");

    let grub2_efi_fallback_path =
        snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));
    fs::write(&grub2_efi_fallback_path, "")
        .expect("Failed to create dummy GRUB2 EFI file in fallback location");

    let found_path = find_grub2(Some(0), Some(temp_dir.path()));

    assert_eq!(found_path, grub2_efi_fallback_path);
}

#[test]
fn test_is_sdboot_only_systemd_boot_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let sdboot_efi_path = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    fs::create_dir_all(sdboot_efi_path.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    File::create(&sdboot_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert!(is_sdboot(Some(0), "x64", Some(temp_dir.path())));
}

#[test]
fn test_is_sdboot_systemd_boot_and_grub2_exist() {
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

    assert!(!is_sdboot(Some(0), "x64", Some(temp_dir.path())));
}

#[test]
fn test_is_sdboot_only_grub2_exist() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2_efi_path = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));

    fs::create_dir_all(grub2_efi_path.parent().unwrap())
        .expect("Failed to create directory for GRUB2 EFI file");

    File::create(&grub2_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert!(!is_sdboot(Some(0), "x64", Some(temp_dir.path())));
}

#[test]
fn test_is_sdboot_neither_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    assert!(!is_sdboot(Some(0), "x64", Some(temp_dir.path())));
}

#[test]
fn test_is_grub2_exists_primary() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    // Create the GRUB2 EFI file in the primary expected location
    let grub2_efi_path = snapshot_dir.join(format!("usr/share/efi/{}/grub.efi", ARCH));
    fs::create_dir_all(grub2_efi_path.parent().unwrap())
        .expect("Failed to create directory for GRUB2 EFI file");
    File::create(&grub2_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert!(is_grub2(Some(0), Some(temp_dir.path())));
}

#[test]
fn test_is_grub2_not_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    assert!(!is_grub2(Some(0), Some(temp_dir.path())));
}

#[test]
fn test_is_grub2_exists_fallback() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    // Create the GRUB2 EFI file in the fallback location
    let grub2_efi_fallback_path =
        snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));
    fs::create_dir_all(grub2_efi_fallback_path.parent().unwrap())
        .expect("Failed to create directory for GRUB2 EFI file in fallback location");
    File::create(&grub2_efi_fallback_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert!(is_grub2(Some(0), Some(temp_dir.path())));
}

#[test]
fn test_determine_boot_dst_only_systemd_boot_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let sdboot_efi_path = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    fs::create_dir_all(sdboot_efi_path.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    File::create(&sdboot_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert_eq!(
        determine_boot_dst(Some(0), "x64", Some(temp_dir.path())).unwrap(),
        "/EFI/systemd",
        "Failed to get boot_dst"
    );
}

#[test]
fn test_determine_boot_dst_systemd_boot_and_grub2_exist() {
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

    assert_eq!(
        determine_boot_dst(Some(0), "x64", Some(temp_dir.path())).unwrap(),
        "/EFI/opensuse",
        "Failed to get boot_dst"
    );
}

#[test]
fn test_determine_boot_dst_only_grub2_exist() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2_efi_path = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));

    fs::create_dir_all(grub2_efi_path.parent().unwrap())
        .expect("Failed to create directory for GRUB2 EFI file");

    File::create(&grub2_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert_eq!(
        determine_boot_dst(Some(0), "x64", Some(temp_dir.path())).unwrap(),
        "/EFI/opensuse",
        "Failed to get boot_dst"
    );
}

#[test]
fn test_determine_boot_dst_neither_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let result = determine_boot_dst(Some(0), "x64", Some(temp_dir.path()));
    assert!(
        result.is_err(),
        "Expected an error for file without version pattern"
    );
    assert_eq!(
        result.unwrap_err(),
        "Unsupported bootloader or unable to determine bootloader"
    );
}

#[test]
fn test_find_bootloader_sdboot_present() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let sdboot_efi_path = snapshot_dir.join("usr/lib/systemd-boot/systemd-bootx64.efi");
    fs::create_dir_all(sdboot_efi_path.parent().unwrap())
        .expect("Failed to create directory for systemd-boot EFI file");
    File::create(&sdboot_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert_eq!(
        find_bootloader(Some(0), "x64", Some(temp_dir.path())).unwrap(),
        sdboot_efi_path
    );
}

#[test]
fn test_find_bootloader_grub2_present() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2_efi_path = snapshot_dir.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));
    fs::create_dir_all(grub2_efi_path.parent().unwrap())
        .expect("Failed to create directory for GRUB2 EFI file");
    File::create(&grub2_efi_path)
        .unwrap()
        .write_all(b"")
        .unwrap();

    assert_eq!(
        find_bootloader(Some(0), "x64", Some(temp_dir.path())).unwrap(),
        grub2_efi_path
    );
}

#[test]
fn test_find_bootloader_none_present() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    assert!(find_bootloader(Some(0), "x64", Some(temp_dir.path())).is_err());
}

#[test]
fn test_find_bootloader_with_both_systemd_and_grub2() {
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

    let result = find_bootloader(Some(0), "x64", Some(temp_dir.path()));

    assert!(matches!(result, Ok(ref path) if path == &grub2_efi_path));
}

#[test]
fn test_find_version() {
    // Simulate a byte slice that contains a version string
    let content = b"Some text before version: START 1.2.3 END some text after version";
    // Define the start and end patterns that delimit the version string
    let start_pattern = b"START ";
    let end_pattern = b" END";

    // Call the `find_version` function with the content and patterns
    let version = find_version(content, start_pattern, end_pattern);

    // Assert that the returned version string is as expected
    assert_eq!(
        version,
        Some("1.2.3".to_string()),
        "The version found does not match the expected value"
    );
}

#[test]
fn test_find_version_no_match() {
    let content = b"The quick brown fox jumps over the lazy dog";
    let start_pattern = b"version:";
    let end_pattern = b";";

    let result = find_version(content, start_pattern, end_pattern);
    assert!(result.is_none(), "Expected None, but got Some");
}

#[test]
fn test_find_version_with_special_characters() {
    let content = b"Here is the version: \x012.34\x00; and some more text";
    let start_pattern = b"version: \x01";
    let end_pattern = b"\x00;";

    let result = find_version(content, start_pattern, end_pattern)
        .expect("Expected a version string, but none was found");

    assert_eq!(
        result, "2.34",
        "The extracted version string does not match the expected value"
    );
}

#[test]
fn test_bootloader_version_custom_systemd_boot() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let systemd_boot_path = snapshot_dir.join("EFI").join("systemd");
    fs::create_dir_all(&systemd_boot_path).expect("Failed to create systemd-boot path");
    let systemd_boot_test_file = systemd_boot_path.join("systemd-bootx64.efi");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &systemd_boot_test_file,
    )
    .expect("Failed to copy systemd-boot test file");

    let version_sdboot =
        bootloader_version(Some(0), "", "", "", "", Some(systemd_boot_test_file), None).unwrap();
    assert_eq!(
        version_sdboot, "255.4+suse.17.gbe772961ad",
        "Failed to detect systemd-boot version"
    );
}

#[test]
fn test_bootloader_version_custom_grub2() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let snapshot_dir = temp_dir.path().join("0").join("snapshot");

    let grub2_path = snapshot_dir.join("EFI").join("opensuse");
    fs::create_dir_all(&grub2_path).expect("Failed to create GRUB2 path");
    let grub2_test_file = grub2_path.join("grubx64.efi");
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/grub2.efi"),
        &grub2_test_file,
    )
    .expect("Failed to copy GRUB2 test file");

    let version_grub2 = bootloader_version(Some(0), "", "", "", "", Some(grub2_test_file), None).unwrap();
    assert_eq!(version_grub2, "2.12", "Failed to detect GRUB2 version");
}

#[test]
fn test_bootloader_version_systemd_boot() {
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

    let version_sdboot = bootloader_version(
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        None,
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(
        version_sdboot, "255.4+suse.17.gbe772961ad",
        "Failed to detect systemd-boot version"
    );
}

#[test]
fn test_bootloader_version_grub2() {
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

    let version_grub2 = bootloader_version(
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        None,
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(version_grub2, "2.12", "Failed to detect grub2 version");
}

#[test]
fn test_bootloader_version_shim() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

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
    fs::copy(
        PathBuf::from("src/unit_tests/test_files/systemd_boot.efi"),
        &shim_efi_file,
    )
    .expect("Failed to copy shim efi file");

    let version_sdboot = bootloader_version(
        Some(0),
        "x64",
        "/usr/share/efi/x86_64",
        "/boot/efi",
        "/EFI/systemd",
        None,
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(
        version_sdboot, "255.4+suse.17.gbe772961ad",
        "Failed to detect shim (systemd-boot) version"
    );
}

#[test]
fn test_bootloader_version_file_not_found() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let non_existent_file = temp_dir.path().join("non_existent_file.efi");

    let result = bootloader_version(
        Some(0),
        "x64",
        "/usr/lib",
        temp_dir.path().to_str().unwrap(),
        "EFI/nonexistent",
        Some(non_existent_file),
        None,
    );

    assert!(result.is_err(), "Expected an error for non-existent file");
    assert!(
        result.unwrap_err().starts_with("File does not exist:"),
        "Error message did not start with expected text"
    );
}

#[test]
fn test_bootloader_version_no_version_pattern() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("empty_version_pattern.efi");

    // Create a file with content that doesn't match any version pattern
    let mut file = fs::File::create(&file_path).expect("Failed to create test file");
    writeln!(
        file,
        "This file does not contain a recognizable version pattern."
    )
    .expect("Failed to write to test file");

    let result = bootloader_version(
        Some(0),
        "x64",
        "/usr/lib",
        temp_dir.path().to_str().unwrap(),
        "EFI/nonexistent",
        Some(file_path),
        None,
    );

    assert!(
        result.is_err(),
        "Expected an error for file without version pattern"
    );
    assert_eq!(result.unwrap_err(), "Version not found");
}

#[test]
fn test_is_installed_true() {
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

    let is_installed = is_installed(
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
fn test_is_installed_false_bootloader() {
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

    let is_installed = is_installed(
        Some(0),
        "",
        "",
        "",
        "",
        Some(systemd_boot_test_file),
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(is_installed, false, "Expected is_installed to return true")
}

#[test]
fn test_is_installed_false_flag() {
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

    let is_installed = is_installed(
        Some(0),
        "",
        "",
        "",
        "",
        Some(systemd_boot_test_file),
        Some(temp_dir.path()),
    )
    .unwrap();
    assert_eq!(is_installed, false, "Expected is_installed to return true")
}

#[test]
fn test_get_shimdir() {
    let expected_path = format!("/usr/share/efi/{}", ARCH);
    assert_eq!(get_shimdir(), expected_path);
}
