pub mod cli;
pub mod fs;
pub mod io;
pub mod ui;

use cli::ensure_root_permissions;
use fs::is_installed;
use io::{log_info, print_error};
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
/// let status = sdbootutil::command_kernels();
/// assert_eq!(status, 0);
/// ```
pub fn command_kernels() -> u32 {
    let message = "Kernels command called";
    log_info(message, 1);
    0
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
/// let status = sdbootutil::command_snapshots();
/// assert_eq!(status, 1);
/// ```
pub fn command_snapshots() -> u32 {
    let message = "Snapshots command called";
    log_info(message, 1);
    1
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
/// let status = sdbootutil::command_entries();
/// assert_eq!(status, 2);
/// ```
pub fn command_entries() -> u32 {
    let message = "Entries command called";
    log_info(message, 1);
    2
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
/// let status = sdbootutil::command_bootloader();
/// assert_eq!(status, 3);
/// ```
pub fn command_bootloader() -> u32 {
    let message = "Bootloader command called";
    log_info(message, 1);
    3
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
/// let status = sdbootutil::command_add_kernel("5.8.0-53-generic");
/// assert_eq!(status, 4);
/// ```
pub fn command_add_kernel(kernel_version: &str) -> u32 {
    let message = format!("AddKernel command called with version {}", kernel_version);
    log_info(&message, 1);
    4
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
/// let status = sdbootutil::command_add_all_kernels();
/// assert_eq!(status, 5);
/// ```
pub fn command_add_all_kernels() -> u32 {
    let message = "AddAllKernels command called";
    log_info(message, 1);
    5
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
/// let status = sdbootutil::command_mkinitrd();
/// assert_eq!(status, 6);
/// ```
pub fn command_mkinitrd() -> u32 {
    let message = "Mkinitrd command called";
    log_info(message, 1);
    6
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
/// let status = sdbootutil::command_remove_kernel("5.8.0-53-generic");
/// assert_eq!(status, 7);
/// ```
pub fn command_remove_kernel(kernel_version: &str) -> u32 {
    let message = format!(
        "RemoveKernel command called with version {}",
        kernel_version
    );
    log_info(&message, 1);
    7
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
/// let status = sdbootutil::command_remove_all_kernels();
/// assert_eq!(status, 8);
/// ```
pub fn command_remove_all_kernels() -> u32 {
    let message = "RemoveAllKernels command called";
    log_info(message, 1);
    8
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
/// let status = sdbootutil::command_list_kernels();
/// assert_eq!(status, 9);
/// ```
pub fn command_list_kernels() -> u32 {
    let message = "ListKernels command called";
    log_info(message, 1);
    9
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
/// let status = sdbootutil::command_list_entries();
/// assert_eq!(status, 10);
/// ```
pub fn command_list_entries() -> u32 {
    let message = "ListEntries command called";
    log_info(message, 1);
    10
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
/// let status = sdbootutil::command_list_snapshots();
/// assert_eq!(status, 11);
/// ```
pub fn command_list_snapshots() -> u32 {
    let message = "ListSnapshots command called";
    log_info(message, 1);
    11
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
/// let status = sdbootutil::command_set_default_snapshot();
/// assert_eq!(status, 12);
/// ```
pub fn command_set_default_snapshot() -> u32 {
    let message = "SetDefaultSnapshot command called";
    log_info(message, 1);
    12
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
/// let status = sdbootutil::command_is_bootable();
/// assert_eq!(status, 13);
/// ```
pub fn command_is_bootable() -> u32 {
    let message = "IsBootable command called";
    log_info(message, 1);
    13
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
/// );
///
/// assert!(!installed, "Expected systemd-boot to not be detected as installed");
/// ```
pub fn command_is_installed(
    snapshot: u64,
    firmware_arch: &str,
    shimdir: &str,
    boot_root: &str,
    boot_dst: &str,
    filename: Option<PathBuf>,
    override_prefix: Option<&Path>,
) -> bool {
    return is_installed(
        snapshot,
        firmware_arch,
        shimdir,
        boot_root,
        boot_dst,
        filename,
        override_prefix,
    );
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
/// let status = sdbootutil::command_install();
/// assert_eq!(status, 14);
/// ```
pub fn command_install() -> u32 {
    let message = "Install command called";
    log_info(message, 1);
    14
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
/// let status = sdbootutil::command_needs_update();
/// assert_eq!(status, 15);
/// ```
pub fn command_needs_update() -> u32 {
    let message = "NeedsUpdate command called";
    log_info(message, 1);
    15
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
/// let status = sdbootutil::command_update();
/// assert_eq!(status, 16);
/// ```
pub fn command_update() -> u32 {
    let message = "Update command called";
    log_info(message, 1);
    16
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
/// let status = sdbootutil::command_force_update();
/// assert_eq!(status, 17);
/// ```
pub fn command_force_update() -> u32 {
    let message = "ForceUpdate command called";
    log_info(message, 1);
    17
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
/// let status = sdbootutil::command_update_predictions();
/// assert_eq!(status, 18);
/// ```
pub fn command_update_predictions() -> u32 {
    let message = "UpdatePredictions command called";
    log_info(message, 1);
    18
}


/// only for demonstration purposes
pub fn test_functions() {
    if let Err(e) = ensure_root_permissions() {
        let message = format!("Failed to get root privileges: {}", e);
        print_error(&message);
        std::process::exit(1);
    }

    let mut root_snapshot = 1;
    let mut _root_prefix = "";
    let mut _root_subvol = "";
    let firmware_arch = "x64";
    match fs::get_root_snapshot_info() {
        Ok((prefix, snapshot_id, full_path)) => {
            io::log_info(
                &format!(
                    "Prefix: {}, Snapshot ID: {}, Full Path: {}",
                    prefix, snapshot_id, full_path
                ),
                1,
            );
            root_snapshot = snapshot_id;
            _root_prefix = &prefix;
            _root_subvol = &full_path;
        }
        Err(e) => {
            print_error(&format!("{}", e));
        }
    }
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

    let boot_dst = match fs::determine_boot_dst(root_snapshot, firmware_arch, None) {
        Ok(dst) => dst,
        Err(e) => {
            print_error(e);
            "";
            return;
        }
    };

    if command_is_installed(
        root_snapshot,
        firmware_arch,
        "/usr/share/efi/x86_64",
        "/boot/efi",
        boot_dst,
        None,
        None,
    ) {
        log_info("systemd-boot was installed using this tool", 0)
    }
    else {
        log_info("systemd-boot was not installed using this tool", 0)
    }
}

#[cfg(test)]
mod unit_tests;
