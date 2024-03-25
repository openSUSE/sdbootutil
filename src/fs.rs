use super::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct RollbackItem {
    original_path: PathBuf,
}

impl RollbackItem {
    pub fn new(original_path: PathBuf) -> Self {
        RollbackItem { original_path }
    }

    pub fn cleanup(&self) -> std::io::Result<()> {
        let console_printer = ConsolePrinter;
        let backup_path = self.original_path.with_extension("bak");
        if backup_path.exists() {
            fs::rename(&backup_path,&self.original_path).expect("Failed to restore from backup");
            let message = format!("restored {}", self.original_path.display());
            console_printer.log_info(&message);
        }
        else {
            if self.original_path.exists() {
                fs::remove_file(&self.original_path).expect("Failed to remove original file");
            }
            else {
                let message = format!("The following file doesn't exist and couldn't be removed: '{}'", self.original_path.display());
                console_printer.log_info(&message);
            }
        }
    Ok(())
    }
}

pub fn create_temp_dir() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create a temporary directory");
    let temp_dir_path = temp_dir.path().to_path_buf();
    (temp_dir, temp_dir_path)
}


pub fn cleanup_rollback_items(rollback_items: &[RollbackItem]) {
    for item in rollback_items {
        if let Err(e) = item.cleanup() {
            let message = format!("Error cleaning up item: {}", e);
            print_error(&message);
        }
    }
}

/// Checks if the filesystem type of `/etc` is `overlayfs`.
///
/// # Returns
///
/// `Ok(true)` if the filesystem type is `overlayfs`, `Ok(false)` otherwise, or an `Error` if the command execution fails.
pub fn is_transactional(command_executor: &dyn CommandExecutor) -> Result<bool, Box<dyn Error>> {
    
    let filesystem_type = command_executor.get_command_output("stat", &["-f", "-c", "%T", "/etc"])?;
    Ok(filesystem_type == "overlayfs")
}