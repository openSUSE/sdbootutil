pub mod cli;
pub mod fs;
pub mod io;
pub mod ui;

use cli::ensure_root_permissions;
use fs::is_installed;
use io::log_info;
use std::error::Error;
use std::path::{Path, PathBuf};

/// Executes the `Kernels` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `0`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_kernels().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_kernels() -> Result<bool, String> {
    let message = "Kernels command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `Snapshots` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `1`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_snapshots().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_snapshots() -> Result<bool, String> {
    let message = "Snapshots command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `Entries` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `2`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_entries().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_entries() -> Result<bool, String> {
    let message = "Entries command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `Bootloader` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `3`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_bootloader().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_bootloader() -> Result<bool, String> {
    let message = "Bootloader command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `AddKernel` command with a specified kernel version.
///
/// Logs the action including the kernel version and returns a status code.
///
/// # Arguments
///
/// * `kernel_version` - A string slice that specifies the kernel version to remove.
///
/// # Returns
///
/// Always returns `4`, which could represent a specific status related to the command's outcome.
///
/// # Example
///
/// ```
/// let status = sdbootutil::command_add_kernel("5.8.0-53-generic").unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_add_kernel(kernel_version: &str) -> Result<bool, String> {
    let message = format!("AddKernel command called with version {}", kernel_version);
    log_info(&message, 1);
    Ok(true)
}

/// Executes the `AddAllKernels` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `5`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_add_all_kernels().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_add_all_kernels() -> Result<bool, String> {
    let message = "AddAllKernels command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `Mkinitrd` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `6`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_mkinitrd().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_mkinitrd() -> Result<bool, String> {
    let message = "Mkinitrd command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `RemoveKernel` command with a specified kernel version.
///
/// Logs the action including the kernel version and returns a status code.
///
/// # Arguments
///
/// * `kernel_version` - A string slice that specifies the kernel version to remove.
///
/// # Returns
///
/// Always returns `7`, which could represent a specific status related to the command's outcome.
///
/// # Example
///
/// ```
/// let status = sdbootutil::command_remove_kernel("5.8.0-53-generic").unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_remove_kernel(kernel_version: &str) -> Result<bool, String> {
    let message = format!(
        "RemoveKernel command called with version {}",
        kernel_version
    );
    log_info(&message, 1);
    Ok(true)
}

/// Executes the `RemoveAllKernels` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `8`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_remove_all_kernels().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_remove_all_kernels() -> Result<bool, String> {
    let message = "RemoveAllKernels command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `ListKernels` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `9`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_list_kernels().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_list_kernels() -> Result<bool, String> {
    let message = "ListKernels command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `ListEntries` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `10`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_list_entries().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_list_entries() -> Result<bool, String> {
    let message = "ListEntries command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `ListSnapshots` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `11`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_list_snapshots().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_list_snapshots() -> Result<bool, String> {
    let message = "ListSnapshots command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `SetDefaultSnapshot` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `12`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_set_default_snapshot().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_set_default_snapshot() -> Result<bool, String> {
    let message = "SetDefaultSnapshot command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `IsBootable` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `13`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_is_bootable().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_is_bootable() -> Result<bool, String> {
    let message = "IsBootable command called";
    log_info(message, 1);
    Ok(true)
}

/// Checks if systemd-boot is installed by `sdbootutil`.
///
/// This function verifies the installation status of systemd-boot by checking for the presence of a bootloader version file
/// and an installation flag file. It determines whether systemd-boot was installed using `sdbootutil` based on these criteria.
///
/// # Arguments
/// - `snapshot`: The snapshot identifier used to locate the snapshot-specific bootloader files.
/// - `firmware_arch`: The architecture of the firmware, such as "x86_64" or "arm64", used to refine the search for bootloader files.
/// - `shimdir`: The directory containing the bootloader shim, if any. This is used in constructing the path to check for a shim EFI file.
/// - `boot_root`: The root directory for boot files. This path is used as the base for constructing the paths to the bootloader version file and the installation flag file.
/// - `boot_dst`: The destination directory for boot files, relative to `boot_root`. This is further used in constructing the path to the installation flag file.
/// - `filename`: An optional filename for the bootloader version file. If provided, this file will be checked directly; otherwise, the function will attempt to determine the appropriate file based on other parameters.
/// - `override_prefix`: An optional path override that, if provided, will be used as the base directory for searching the bootloader files instead of the default path.
///
/// # Returns
/// - Returns `true` if both the bootloader version check is successful and the installation flag file is found,
/// indicating that systemd-boot was likely installed using `sdbootutil`.
/// - Returns `false` otherwise.
///
/// # Examples
/// ```
/// use sdbootutil::command_is_installed;
///
/// let installed = command_is_installed(
///     0,
///     "x86_64",
///     "/usr/share/efi/x86_64",
///     "/boot/efi",
///     "EFI/systemd",
///     None,
///     None,
/// ).is_ok();
///
/// assert!(installed, "Expected systemd-boot to not be detected as installed");
/// ```
pub fn command_is_installed(
    snapshot: u64,
    firmware_arch: &str,
    shimdir: &str,
    boot_root: &str,
    boot_dst: &str,
    filename: Option<PathBuf>,
    override_prefix: Option<&Path>,
) -> Result<bool, String> {
    let result = is_installed(
        snapshot,
        firmware_arch,
        shimdir,
        boot_root,
        boot_dst,
        filename,
        override_prefix,
    ).is_ok();
    if result {
        log_info("systemd-boot was installed using this tool", 0)
    } else {
        log_info("systemd-boot was not installed using this tool", 0)
    }
    Ok(result)
}

/// Executes the `Install` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `14`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_install().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_install() -> Result<bool, String> {
    let message = "Install command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `NeedsUpdate` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `15`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_needs_update().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_needs_update() -> Result<bool, String> {
    let message = "NeedsUpdate command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `Update` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `16`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_update().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_update() -> Result<bool, String> {
    let message = "Update command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `ForceUpdate` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `17`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_force_update().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_force_update() -> Result<bool, String> {
    let message = "ForceUpdate command called";
    log_info(message, 1);
    Ok(true)
}

/// Executes the `UpdatePredictions` command.
///
/// This function logs the action and returns a predefined status code.
///
/// # Returns
///
/// Always returns `18`, indicating a specific status after execution.
///
/// # Examples
///
/// ```
/// let status = sdbootutil::command_update_predictions().unwrap();
/// assert_eq!(status, true);
/// ```
pub fn command_update_predictions() -> Result<bool, String> {
    let message = "UpdatePredictions command called";
    log_info(message, 1);
    Ok(true)
}

/// Gathers essential system information required for bootloader management.
///
/// This function aggregates various pieces of system information, including snapshot identifier, directory paths,
/// and firmware architecture. It ensures root privileges are available before proceeding to collect the information.
/// The function relies on `fs::get_root_snapshot_info` and `fs::determine_boot_dst` to retrieve snapshot-related information
/// and boot destination directory, respectively.
///
/// # Returns
///
/// Returns `Ok` with a tuple containing:
/// - Snapshot identifier (u64).
/// - Root prefix (String).
/// - Root subvolume (String).
/// - Firmware architecture (String, currently hardcoded to "x64").
/// - Boot destination directory (String).
/// - Shim directory (String, currently hardcoded).
/// - Boot root directory (String, currently hardcoded).
///
/// Returns `Err(String)` with an error message if any step of information gathering fails.
///
/// # Examples
///
/// ```
/// // Commented as it required root permissions
/// //let system_info = sdbootutil::get_system_info().expect("Failed to get system information");
///
/// // Example of destructuring the returned tuple
/// //let (snapshot, prefix, subvol, arch, boot_dst, shimdir, boot_root) = system_info;
/// ```
pub fn get_system_info() -> Result<(u64, String, String, String, String, String, String), String> {
    if let Err(e) = ensure_root_permissions() {
        let message = format!("Failed to get root privileges: {}", e);
        return Err(message);
    }
    let firmware_arch = "x64".to_string();

    let (root_snapshot, root_prefix, root_subvol) = match fs::get_root_snapshot_info() {
        Ok((prefix, snapshot_id, full_path)) => (snapshot_id, prefix, full_path),
        Err(e) => {
            let message = format!("Failed to get root snapshot info: {}", e);
            return Err(message);
        }
    };

    let boot_dst = match fs::determine_boot_dst(root_snapshot, &firmware_arch, None) {
        Ok(dst) => dst.to_string(),
        Err(e) => {
            let message = format!("Failed to determine boot_dst: {}", e);
            return Err(message);
        }
    };
    let shimdir = "/usr/share/efi/x86_64".to_string();
    let boot_root = "/boot/efi".to_string();

    Ok((
        root_snapshot,
        root_prefix,
        root_subvol,
        firmware_arch,
        boot_dst,
        shimdir,
        boot_root
    ))
}

/// only for demonstration purposes
pub fn test_functions() {
    if fs::is_transactional().expect("Failed to check if filesystem is transactional") {
        log_info("It is a transactional system", 1)
    } else {
        log_info("It is not a transactional system", 1)
    }
    let (_temp_dir, _tmpdir_path) = fs::create_temp_dir();
    let mut rollback_items = vec![
        fs::RollbackItem::new(PathBuf::from("/path/to/file1")),
        fs::RollbackItem::new(PathBuf::from("/path/to/file2")),
    ];
    fs::cleanup_rollback_items(&rollback_items);
    fs::reset_rollback_items(&mut rollback_items);
}

#[cfg(test)]
mod unit_tests;
