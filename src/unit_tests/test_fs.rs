use super::super::fs::*;
use super::super::CommandExecutor;

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
    assert!(result.is_err(), "Expected an error for the failing command execution");
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