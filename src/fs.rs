use super::io::{log_info, print_error};
use libbtrfs::subvolume;
use std::env::consts::ARCH;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A structure representing an item subject to rollback actions.
///
/// This structure is used to encapsulate the path of a file that may need to be
/// restored to a previous state or cleaned up as part of a rollback process.
///
/// # Fields
///
/// * `original_path`: The `PathBuf` representing the path to the original file.
pub(crate) struct RollbackItem {
    original_path: PathBuf,
}

impl RollbackItem {
    /// Creates a new `RollbackItem` with the specified original file path.
    ///
    /// # Arguments
    ///
    /// * `original_path` - A `PathBuf` indicating the path to the original file that might be subject to rollback.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `RollbackItem`.
    pub(crate) fn new(original_path: PathBuf) -> Self {
        RollbackItem { original_path }
    }

    /// Performs cleanup actions for the rollback item.
    ///
    /// If a backup file (with a `.bak` extension) exists, this function will attempt to restore the original file from the backup.
    /// If no backup is found but the original file exists, the original file will be removed.
    /// If neither the original file nor its backup exists, an informational message will be logged.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if cleanup actions are completed successfully, or an `IoError` if any file operations fail.
    pub(crate) fn cleanup(&self) -> std::io::Result<()> {
        let backup_path = self.original_path.with_extension("bak");
        if backup_path.exists() {
            fs::rename(&backup_path, &self.original_path).expect("Failed to restore from backup");
            let message = format!("restored {}", self.original_path.display());
            log_info(&message, 1);
        } else {
            if self.original_path.exists() {
                fs::remove_file(&self.original_path).expect("Failed to remove original file");
            } else {
                let message = format!(
                    "The following file doesn't exist and couldn't be removed: '{}'",
                    self.original_path.display()
                );
                log_info(&message, 1);
            }
        }
        Ok(())
    }
}

/// Creates a new temporary directory using the `tempfile` crate.
///
/// This function is typically used to create a temporary workspace for operations that require filesystem changes
/// which should not affect the permanent storage.
///
/// # Returns
///
/// Returns a tuple containing the `TempDir` object representing the temporary directory and its `PathBuf`.
/// The `TempDir` object ensures that the directory is deleted when it goes out of scope.
pub(crate) fn create_temp_dir() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create a temporary directory");
    let temp_dir_path = temp_dir.path().to_path_buf();
    (temp_dir, temp_dir_path)
}

/// Cleans up a list of `RollbackItem`s.
///
/// Iterates over each `RollbackItem` in the provided slice and performs cleanup actions.
/// If any cleanup action fails, an error message will be logged.
///
/// # Arguments
///
/// * `rollback_items` - A slice of `RollbackItem`s to be cleaned up.
pub(crate) fn cleanup_rollback_items(rollback_items: &[RollbackItem]) {
    for item in rollback_items {
        if let Err(e) = item.cleanup() {
            let message = format!("Error cleaning up item: {}", e);
            print_error(&message);
        }
    }
}

/// Resets the state of a list of `RollbackItem`s by removing any associated backup files.
///
/// For each item in the list, if a backup file exists, it will be removed. After processing all items,
/// the list will be cleared, indicating that no rollback items remain to be processed.
///
/// # Arguments
///
/// * `rollback_items` - A mutable reference to a `Vec<RollbackItem>` representing the list of items to be reset.
pub(crate) fn reset_rollback_items(rollback_items: &mut Vec<RollbackItem>) {
    for item in rollback_items.iter() {
        let backup_path = item.original_path.with_extension("bak");
        if backup_path.exists() {
            if let Err(e) = fs::remove_file(&backup_path) {
                let message = format!(
                    "Failed to remove backup file {}: {}",
                    backup_path.display(),
                    e
                );
                print_error(&message);
            } else {
                let message = format!("Removed backup file {}", backup_path.display());
                log_info(&message, 1)
            }
        }
    }
    rollback_items.clear();
}

/// Checks if the filesystem type of `/etc` is `overlayfs`.
///
/// # Returns
///
/// `Ok(true)` if the filesystem type is `overlayfs`, `Ok(false)` otherwise, or an `Error` if an instruction fails.
pub(crate) fn is_transactional(prefix: Option<&str>) -> Result<bool, String> {
    let mounts_file_path = match prefix {
        Some(prefix) => PathBuf::from(prefix)
            .join("proc/mounts")
            .to_string_lossy()
            .into_owned(),
        None => "/proc/mounts".to_string(),
    };
    let mounts_file = File::open(mounts_file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(mounts_file);

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 2 {
            let mount_point = parts[1];
            let fs_type = parts[2];

            if mount_point == "/etc" {
                return Ok(fs_type == "overlayfs");
            }
        }
    }

    Ok(false)
}

/// Retrieves detailed information about the root Btrfs snapshot.
///
/// This function extracts and returns the prefix path, the snapshot ID, and the full snapshot path from the system's
/// root directory. It's designed to parse the snapshot path to identify these components, crucial for Btrfs snapshot management.
///
/// # Returns
///
/// A Result containing a tuple of:
/// - The prefix path as a String.
/// - The snapshot ID as a u64.
/// - The full snapshot path as a String.
///
/// # Errors
///
/// Returns an error if the snapshot path does not conform to the expected structure or if any parsing fails.
pub(crate) fn get_root_snapshot_info() -> Result<(String, u64, String), Box<dyn std::error::Error>>
{
    let full_path = subvolume::get_subvol_path("/")?;
    let parts: Vec<&str> = full_path.split("/.snapshots/").collect();
    let prefix = parts.get(0).ok_or("Prefix not found")?.to_string();
    let snapshot_part = parts.get(1).ok_or("Snapshot part not found")?;
    let snapshot_id_str = snapshot_part
        .split('/')
        .next()
        .ok_or("Snapshot ID not found")?;
    let snapshot_id = snapshot_id_str.parse::<u64>()?;

    Ok((prefix, snapshot_id, full_path))
}

/// Determines if a specified Btrfs subvolume is set to read-only mode.
///
/// This function checks the read-only status of a given Btrfs subvolume.
///
/// # Parameters
///
/// - `subvol`: An `Option<String>` representing the path to the subvolume. If `None`, the function
///   assumes there's no subvolume to check and returns `Ok(false)`.
///
/// # Returns
///
/// - `Ok(true)`: If the subvolume is found and is set to read-only.
/// - `Ok(false)`: If the subvolume is not set to read-only or if `None` is passed as the subvolume path.
/// - `Err(e)`: If there's an error checking the read-only status of the subvolume, wrapped in a `Box<dyn Error>`.
///   The error includes a custom message indicating the failure to retrieve the read-only status, along with
///   the original error message.
pub(crate) fn is_subvol_ro(subvol: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
    match subvol {
        Some(subvol) => {
            let is_ro = subvolume::get_readonly(&subvol).map_err(|e| {
                Box::<dyn std::error::Error>::from(format!(
                    "Failed to get readonly status for subvolume '{}': {}",
                    subvol, e
                ))
            })?;
            Ok(is_ro)
        }
        None => Ok(false),
    }
}

/// Finds the path to the systemd-boot EFI file based on a given snapshot and firmware architecture,
/// with an optional prefix to override the default path for testing or other purposes.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory.
/// * `firmware_arch` - The architecture of the firmware, used to construct the EFI file name.
/// * `prefix_override` - An optional path to override the default "/.snapshots" prefix.
///
/// # Returns
///
/// Returns the `PathBuf` pointing to the systemd-boot EFI file.
pub(crate) fn find_sdboot(
    snapshot: Option<u64>,
    firmware_arch: &str,
    prefix_override: Option<&Path>,
) -> PathBuf {
    let base_prefix = match prefix_override {
        Some(override_path) => override_path.to_path_buf(),
        None => Path::new("/.snapshots").to_path_buf(),
    };
    let prefix = match snapshot {
        Some(snap) => base_prefix.join(snap.to_string()).join("snapshot"),
        None => base_prefix,
    };

    let mut sdboot_path = prefix.join(format!(
        "usr/lib/systemd-boot/systemd-boot{}.efi",
        firmware_arch
    ));

    if !sdboot_path.exists() {
        sdboot_path = prefix.join(format!(
            "usr/lib/systemd/boot/efi/systemd-boot{}.efi",
            firmware_arch
        ));
    }

    sdboot_path
}

/// Finds the path to the GRUB2 EFI file based on a given snapshot.
///
/// This function constructs a path within a specified snapshot directory to locate the GRUB2 EFI file.
/// It tries the primary expected location first and falls back to a secondary location if the EFI file is not found.
/// An optional override prefix can be provided for testing purposes.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory.
/// * `override_prefix` - An optional override prefix for the search, used primarily for testing.
///
/// # Returns
///
/// Returns the `PathBuf` pointing to the GRUB2 EFI file, whether it's in the primary or fallback location.
pub(crate) fn find_grub2(snapshot: Option<u64>, prefix_override: Option<&Path>) -> PathBuf {
    let base_prefix = match prefix_override {
        Some(override_path) => override_path.to_path_buf(),
        None => Path::new("/.snapshots").to_path_buf(),
    };
    let prefix = match snapshot {
        Some(snap) => base_prefix.join(snap.to_string()).join("snapshot"),
        None => base_prefix,
    };
    let mut grub2_path = prefix.join(format!("usr/share/efi/{}/grub.efi", ARCH));

    if !grub2_path.exists() {
        grub2_path = prefix.join(format!("usr/share/grub2/{}-efi/grub.efi", ARCH));
    }
    grub2_path
}

/// Determines if the systemd-boot bootloader is installed for a given snapshot and firmware architecture.
///
/// This function checks for the presence of a systemd-boot EFI file and the absence of a GRUB2 EFI file
/// to determine if systemd-boot is the installed bootloader.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory.
/// * `firmware_arch` - The architecture of the firmware, used to check for the systemd-boot EFI file.
///
/// # Returns
///
/// Returns `true` if the systemd-boot EFI file exists and the GRUB2 EFI file does not, indicating systemd-boot is installed.
pub(crate) fn is_sdboot(
    snapshot: Option<u64>,
    firmware_arch: &str,
    override_prefix: Option<&Path>,
) -> bool {
    let sdboot = find_sdboot(snapshot, firmware_arch, override_prefix);
    let grub2 = find_grub2(snapshot, override_prefix);

    sdboot.exists() && !grub2.exists()
}

/// Determines if the GRUB2 bootloader is installed for a given snapshot.
///
/// This function checks for the presence of a GRUB2 EFI file to determine if GRUB2 is the installed bootloader.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory.
///
/// # Returns
///
/// Returns `true` if the GRUB2 EFI file exists, indicating GRUB2 is installed.
pub(crate) fn is_grub2(snapshot: Option<u64>, override_prefix: Option<&Path>) -> bool {
    find_grub2(snapshot, override_prefix).exists()
}

/// Determines the boot destination path based on the installed bootloader for a given snapshot.
///
/// This function checks whether the systemd-boot or GRUB2 bootloader is installed for
/// the specified snapshot and firmware architecture. It returns the appropriate boot destination path
/// based on the bootloader detected. The function supports overriding the default search prefix through an optional parameter.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory. This is used to locate the snapshot-specific bootloader files.
/// * `firmware_arch` - The architecture of the firmware, such as "x64" or "arm64".
/// This is used to refine the search for the bootloader files.
/// * `override_prefix` - An optional path override. If provided, this path will be used as the base directory
/// for searching bootloader files, instead of the default path.
///
/// # Returns
///
/// Returns `Ok("/EFI/systemd")` if systemd-boot is detected as the installed bootloader, `Ok("/EFI/opensuse")` if GRUB2 is detected,
/// or an `Err` with a message indicating that the bootloader is unsupported or could not be determined.
pub(crate) fn determine_boot_dst(
    snapshot: Option<u64>,
    firmware_arch: &str,
    override_prefix: Option<&Path>,
) -> Result<&'static str, &'static str> {
    if is_sdboot(snapshot, firmware_arch, override_prefix) {
        Ok("/EFI/systemd")
    } else if is_grub2(snapshot, override_prefix) {
        Ok("/EFI/opensuse")
    } else {
        Err("Unsupported bootloader or unable to determine bootloader")
    }
}

/// Finds the installed bootloader (systemd-boot or GRUB2) for a given snapshot and firmware architecture.
///
/// This function attempts to determine which bootloader is installed by checking for the presence of systemd-boot and GRUB2 EFI files.
/// It favors systemd-boot unless only GRUB2 is found.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory.
/// * `firmware_arch` - The architecture of the firmware, used in the search for the systemd-boot EFI file.
///
/// # Returns
///
/// Returns a `Result` containing a `PathBuf` to the detected bootloader EFI file on success,
/// or an error string if no bootloader is detected.
pub(crate) fn find_bootloader(
    snapshot: Option<u64>,
    firmware_arch: &str,
    override_prefix: Option<&Path>,
) -> Result<PathBuf, &'static str> {
    if is_sdboot(snapshot, firmware_arch, override_prefix) {
        Ok(find_sdboot(snapshot, firmware_arch, override_prefix))
    } else if is_grub2(snapshot, override_prefix) {
        Ok(find_grub2(snapshot, override_prefix))
    } else {
        Err("Bootloader not detected")
    }
}

/// Extracts a version string from binary content based on start and end patterns.
///
/// This function searches the given binary content for a sequence that starts with the specified
/// `start_pattern` and ends with the `end_pattern`. It extracts and returns the bytes found between these two patterns
/// as a UTF-8 string. The function is useful for parsing version information from binary files, such as firmware images
/// or compiled executables, where version strings are embedded within the binary data.
///
/// # Arguments
///
/// * `content` - A slice of bytes representing the binary content to be searched.
/// * `start_pattern` - A slice of bytes representing the pattern that marks the beginning of the version string.
/// * `end_pattern` - A slice of bytes representing the pattern that marks the end of the version string.
///
/// # Returns
///
/// Returns an `Option<String>`:
/// - `Some(String)` containing the extracted version string if both start and end patterns are found
/// and the content between them is valid UTF-8.
/// - `None` if either pattern is not found, or if the content between the patterns is not valid UTF-8.
pub(crate) fn find_version(
    content: &[u8],
    start_pattern: &[u8],
    end_pattern: &[u8],
) -> Option<String> {
    if let Some(start_pos) = content
        .windows(start_pattern.len())
        .position(|window| window == start_pattern)
    {
        let version_start_pos = start_pos + start_pattern.len();
        if let Some(end_pos) = content[version_start_pos..]
            .windows(end_pattern.len())
            .position(|window| window == end_pattern)
        {
            let version_bytes = &content[version_start_pos..version_start_pos + end_pos];
            return std::str::from_utf8(version_bytes).ok().map(str::to_string);
        }
    }
    None
}

/// Determines the version of the installed bootloader by analyzing the binary content of the bootloader file.
///
/// This function attempts to find and read the bootloader file specified by the `filename` argument, or by constructing
/// a path based on provided parameters. It then searches the file's content for known version string patterns specific to
/// either systemd-boot or GRUB2 bootloaders. The function is designed to work with binary files where version information
/// is embedded within the data.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory, used when constructing the default path to the bootloader file.
/// * `firmware_arch` - The architecture of the firmware (e.g., "x64"), used in path construction and potentially in selecting
/// version patterns.
/// * `shimdir` - Directory containing the bootloader shim, which is part of the path if the default bootloader file is used.
/// * `boot_root` - The root directory for boot files, forming the base of the constructed path to the bootloader file.
/// * `boot_dst` - The destination directory for boot files, relative to `boot_root`, further specifying the constructed path.
/// * `filename` - An optional specific filename to directly check for the bootloader version. If provided, other path parameters
/// are ignored.
/// * `override_prefix` - An optional path override that, if provided, replaces the `boot_root` in the constructed path to
/// the bootloader file.
///
/// # Returns
///
/// Returns a `Result` with:
/// - `Ok(String)` containing the extracted version string if a known version pattern is found within the bootloader file's content.
/// - `Err(String)` with an appropriate error message if the file does not exist, cannot be read, or if no known version pattern is found.
pub(crate) fn bootloader_version(
    snapshot: Option<u64>,
    firmware_arch: &str,
    shimdir: &str,
    boot_root: &str,
    boot_dst: &str,
    filename: Option<PathBuf>,
    override_prefix: Option<&Path>,
) -> Result<String, String> {
    let prefix = override_prefix.unwrap_or(Path::new(""));
    let fn_path = match filename {
        Some(f) => f,
        None => {
            if PathBuf::from(format!("{}{}/shim.efi", prefix.display(), shimdir)).exists() {
                PathBuf::from(format!(
                    "{}{}{}/grub.efi",
                    prefix.display(),
                    boot_root,
                    boot_dst
                ))
            } else {
                let bootloader = find_bootloader(snapshot, firmware_arch, override_prefix)?;
                PathBuf::from(format!(
                    "{}{}{}/{}",
                    prefix.display(),
                    boot_root,
                    boot_dst,
                    bootloader.file_name().unwrap().to_str().unwrap()
                ))
            }
        }
    };
    if !fn_path.exists() {
        let err = format!("File does not exist: {}", fn_path.display());
        return Err(err);
    }

    let content = fs::read(&fn_path).map_err(|_| "Failed to read file")?;

    let patterns = [
        (&b"LoaderInfo: systemd-boot "[..], &b" ####"[..]),
        (&b"GNU GRUB  version %s\x00"[..], &b"\x00"[..]),
    ];
    for (start, end) in &patterns {
        if let Some(version) = find_version(&content, start, end) {
            return Ok(version);
        }
    }
    Err("Version not found".to_string())
}

/// Identifies the installed bootloader type for a given system setup.
///
/// This function determines which bootloader is currently installed by checking the presence and configuration
/// of systemd-boot and GRUB2. It returns the name of the detected bootloader as a string. The function uses
/// the `is_sdboot` and `is_grub2` helper functions to ascertain the bootloader type based on the snapshot,
/// firmware architecture, and optionally provided path prefix.
///
/// # Arguments
///
/// * `snapshot` - An optional snapshot identifier. If provided, the function checks the snapshot-specific
///   bootloader configuration instead of the system's current configuration.
/// * `firmware_arch` - The architecture of the firmware (e.g., "x64", "aa64"). This parameter is used to
///   locate the EFI files specific to the architecture.
/// * `override_prefix` - An optional path prefix that overrides the default path used to locate bootloader
///   files. This can be useful for testing or when working with chroot environments.
///
/// # Returns
///
/// A `Result` containing:
/// - `Ok(String)` with the bootloader name ("systemd-boot" or "grub2") if a bootloader is detected.
/// - `Err(&'static str)` with an error message ("Bootloader not detected") if neither systemd-boot nor GRUB2
///   configurations are found.
pub(crate) fn bootloader_name(
    snapshot: Option<u64>,
    firmware_arch: &str,
    override_prefix: Option<&Path>,
) -> Result<String, &'static str> {
    if is_sdboot(snapshot, firmware_arch, override_prefix) {
        Ok("systemd-boot".to_string())
    } else if is_grub2(snapshot, override_prefix) {
        Ok("grub2".to_string())
    } else {
        Err("Bootloader not detected")
    }
}

/// Checks whether systemd-boot is installed and marked by `sdbootutil`.
///
/// This function verifies if systemd-boot is installed by checking two criteria: the successful detection of the
/// bootloader version and the presence of a flag file indicating that `sdbootutil` was used for the installation.
/// It constructs the path to the flag file based on given parameters and checks for its existence.
///
/// # Arguments
///
/// * `snapshot` - A numeric identifier for the snapshot directory, used in determining the bootloader version.
/// * `firmware_arch` - The architecture of the firmware, such as "x64" or "arm64".
/// * `shimdir` - The directory containing the bootloader shim, part of the path if the default bootloader file is used.
/// * `boot_root` - The root directory for boot files, used in constructing the path to the flag file.
/// * `boot_dst` - The destination directory for boot files, relative to `boot_root`, used in constructing the path.
/// * `filename` - An optional specific filename to check for the bootloader version. If provided, other path parameters are ignored.
/// * `override_prefix` - An optional path override that replaces `boot_root` in the constructed path to the flag file.
///
/// # Returns
///
/// Returns `Ok(true)` if both the bootloader version is successfully detected and the installation flag file exists,
/// indicating systemd-boot was installed using `sdbootutil`. Returns `Ok(false)` otherwise.
/// Returns `Err(String)` with an error message if any operation (like reading the bootloader file) fails.
pub(crate) fn is_installed(
    snapshot: Option<u64>,
    firmware_arch: &str,
    shimdir: &str,
    boot_root: &str,
    boot_dst: &str,
    filename: Option<PathBuf>,
    override_prefix: Option<&Path>,
) -> Result<bool, String> {
    let prefix = override_prefix.unwrap_or(Path::new(""));
    let bootloader_version_successful = bootloader_version(
        snapshot,
        firmware_arch,
        shimdir,
        boot_root,
        boot_dst,
        filename,
        override_prefix,
    )
    .is_ok();
    let flag_path = format!(
        "{}{}{}/installed_by_sdbootutil",
        prefix.display(),
        boot_root,
        boot_dst
    );
    let installed_flag_path = Path::new(&flag_path);
    let installed_flag_exists = installed_flag_path.exists();

    Ok(bootloader_version_successful && installed_flag_exists)
}

pub(crate) fn get_shimdir() -> String {
    format!("/usr/share/efi/{}", ARCH)
}

/// Checks if the filesystem type of `/` is `btrfs` and the directory /.snapshots exists.
///
/// # Returns
///
/// `Ok(true)` if the filesystem type is `btrfs` and /.snapshots is a directory,
/// `Ok(false)` otherwise, or an `Error` if an instruction fails.
pub(crate) fn is_snapshotted() -> Result<bool, String> {
    let mounts_file = fs::File::open("/proc/mounts").expect("Could not open /proc/mounts");
    let reader = BufReader::new(mounts_file);
    let snapshots_dir = Path::new("/.snapshots");

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 2 {
            let mount_point = parts[1];
            let fs_type = parts[2];

            if mount_point == "/" {
                return Ok(fs_type == "btrfs" && snapshots_dir.is_dir());
            }
        }
    }

    Ok(false)
}
