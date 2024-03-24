use super::*;

/// Checks if the filesystem type of `/etc` is `overlayfs`.
///
/// # Returns
///
/// `Ok(true)` if the filesystem type is `overlayfs`, `Ok(false)` otherwise, or an `Error` if the command execution fails.
pub fn is_transactional(command_executor: &dyn CommandExecutor) -> Result<bool, Box<dyn Error>> {
    
    let filesystem_type = command_executor.get_command_output("stat", &["-f", "-c", "%T", "/etc"])?;
    Ok(filesystem_type == "overlayfs")
}