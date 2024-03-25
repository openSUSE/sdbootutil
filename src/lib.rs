pub mod cli;
pub mod fs;
pub mod io;
pub mod ui;

use io::log_info;
use std::error::Error;

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

#[cfg(test)]
mod unit_tests;
