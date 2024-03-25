use super::super::fs::*;
use super::super::CommandExecutor;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct MockOverlayFsExecutor;

impl CommandExecutor for MockOverlayFsExecutor {
    fn get_command_output(
        &self,
        _command: &str,
        _args: &[&str],
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok("overlayfs".to_string())
    }
}

pub struct MockNonOverlayFsExecutor;

impl CommandExecutor for MockNonOverlayFsExecutor {
    fn get_command_output(
        &self,
        _command: &str,
        _args: &[&str],
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok("ext4".to_string())
    }
}

pub struct MockFailingExecutor;

impl CommandExecutor for MockFailingExecutor {
    fn get_command_output(
        &self,
        _command: &str,
        _args: &[&str],
    ) -> Result<String, Box<dyn std::error::Error>> {
        Err("Simulated command failure".into())
    }
}

#[test]
fn test_is_transactional_with_error() {
    let mock_executor = MockFailingExecutor;
    let result = is_transactional(&mock_executor);
    assert!(
        result.is_err(),
        "Expected an error for the failing command execution"
    );
}

#[test]
fn test_is_transactional_with_overlayfs_mock() {
    let mock_executor = MockOverlayFsExecutor;
    let is_trans = is_transactional(&mock_executor).unwrap();
    assert_eq!(is_trans, true, "Expected true for overlayfs mock");
}

#[test]
fn test_is_transactional_with_non_overlayfs_mock() {
    let mock_executor = MockNonOverlayFsExecutor;
    let is_trans = is_transactional(&mock_executor).unwrap();
    assert_eq!(is_trans, false, "Expected false for non-overlayfs mock");
}

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
