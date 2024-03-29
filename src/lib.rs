pub mod cli;
pub mod fs;
pub mod io;
pub mod ui;

use fs::is_installed;
use io::log_info;
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
///     Some(0),
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
    snapshot: Option<u64>,
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
    )
    .is_ok();
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

/// Gathers comprehensive system information crucial for bootloader management and system configuration.
///
/// This function compiles essential system details, including the Btrfs snapshot information (if available),
/// directory paths for bootloader files, firmware architecture, and bootloader entry token. It validates root
/// permissions before proceeding to ensure the required system information can be safely accessed. The function
/// uses `io::get_bootctl_info` and `io::get_root_filesystem_info` to gather boot and filesystem information,
/// `fs::get_root_snapshot_info` for snapshot-related details, and `fs::determine_boot_dst` for the boot destination.
/// Command-line arguments are processed to override default values where provided.
///
/// # Returns
///
/// A `Result` with either:
/// - An `Ok` variant containing a tuple with:
///   - `root_snapshot`: Optional snapshot identifier (`u64`), present if the system uses Btrfs snapshots.
///   - `root_prefix`: Optional prefix of the root filesystem (`String`), present if snapshots are used.
///   - `root_subvol`: Optional subvolume of the root filesystem (`String`), present if snapshots are used.
///   - `root_uuid`: UUID of the root filesystem (`String`), obtained from `findmnt`.
///   - `root_device`: Source device of the root filesystem (`String`), obtained from `findmnt`.
///   - `firmware_arch`: Firmware architecture (`String`), either specified by the user or obtained from `bootctl`.
///   - `entry_token`: Bootloader entry token (`String`), either specified by the user or obtained from `bootctl`.
///   - `boot_dst`: Destination directory for bootloader files (`String`), determined based on the system configuration.
///   - `boot_root`: Path of the ESP partition (`String`), obtained from `bootctl`.
///   - `image`: Bootloader image file name (`String`), either specified by the user or derived from the firmware architecture.
///   - `no_variables`, `regenerate_initrd`, `no_random_seed`, `all`: Boolean flags reflecting command-line options.
///   - `shimdir`: Directory containing bootloader shim files (`String`), derived from a standard location and architecture.
///
/// - An `Err` variant with a descriptive error message if any step in gathering information fails.
///
/// # Errors
///
/// This function may return an error if:
/// - Root permissions are not granted.
/// - Essential system information (e.g., from `bootctl` or `findmnt`) cannot be obtained.
/// - The system uses Btrfs snapshots, but snapshot-related information cannot be retrieved.
/// - The boot destination directory cannot be determined.
///
/// # Example
///
/// ```no_run
/// let system_info = sdbootutil::process_args_and_get_system_info()
///     .expect("Failed to gather system information");
///
/// let (
///     root_snapshot,
///     root_prefix,
///     root_subvol,
///     root_uuid,
///     root_device,
///     firmware_arch,
///     entry_token,
///     boot_root,
///     boot_dst,
///     image,
///     no_variables,
///     regenerate_initrd,
///     no_random_seed,
///     all,
///     shimdir,
///     cmd
/// ) = system_info;
/// ```
/// Note that this function should be used with care, as it relies on obtaining root privileges
/// and accessing various system paths and configuration details.
pub fn process_args_and_get_system_info() -> Result<
    (
        Option<u64>,
        Option<String>,
        Option<String>,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        bool,
        bool,
        bool,
        bool,
        String,
        Option<cli::Commands>,
    ),
    String,
> {
    if let Err(e) = cli::ensure_root_permissions() {
        let message = format!("Failed to get root privileges: {}", e);
        return Err(message);
    }

    let args = cli::parse_args();

    if let Some(ref path) = args.esp_path {
        std::env::set_var("SYSTEMD_ESP_PATH", path);
    }

    let has_snapshots =
        fs::is_snapshotted().map_err(|e| format!("Couldn't find out if snapshotted: {}", e))?;
    let (default_firmware_arch, default_entry_token, boot_root) =
        io::get_bootctl_info().map_err(|e| format!("Couldn't get bootctl info: {}", e))?;
    let (root_uuid, root_device) = io::get_root_filesystem_info()
        .map_err(|e| format!("Couldn't get root filesystem info: {}", e))?;
    let (root_snapshot, root_prefix, root_subvol) = if has_snapshots {
        fs::get_root_snapshot_info()
            .map(|(prefix, snapshot_id, full_path)| {
                (Some(snapshot_id), Some(prefix), Some(full_path))
            })
            .map_err(|e| format!("Failed to get root snapshot info: {}", e))?
    } else {
        (None, None, None)
    };
    let shimdir = fs::get_shimdir();

    let firmware_arch = args.arch.unwrap_or(default_firmware_arch);
    let image = match args.image {
        Some(ref img) => img.clone(),
        None => match firmware_arch.as_str() {
            "x64" => "vmlinuz".to_string(),
            "aa64" => "Image".to_string(),
            _ => return Err(format!("Unsupported architecture '{}'. Supported are: x64, aa64", firmware_arch)),
        },
    };
    let entry_token = args.entry_token.unwrap_or(default_entry_token);
    let no_variables = args.no_variables;
    let regenerate_initrd = args.regenerate_initrd;
    let no_random_seed = args.no_random_seed;
    let all = args.all;
    let boot_dst = match fs::determine_boot_dst(root_snapshot, &firmware_arch, None) {
        Ok(dst) => dst.to_string(),
        Err(e) => {
            let message = format!("Failed to determine boot_dst: {}", e);
            return Err(message);
        }
    };

    if let Some(ref esp_path) = args.esp_path {
        if esp_path != &boot_root {
            return Err("ESP paths don't match".to_string());
        }
    }

    Ok((
        root_snapshot,
        root_prefix,
        root_subvol,
        root_uuid,
        root_device,
        firmware_arch,
        entry_token,
        boot_root,
        boot_dst,
        image,
        no_variables,
        regenerate_initrd,
        no_random_seed,
        all,
        shimdir,
        args.cmd,
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

    if fs::is_subvol_ro(None).expect("Failed to check if filesystem is ro") {
        log_info("It is ro", 1)
    } else {
        log_info("Subvol is not ro", 1)
    }
}

#[cfg(test)]
mod unit_tests;
