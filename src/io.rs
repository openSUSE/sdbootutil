use super::cli;
use std::process::Command;

/// Prints a given message to the standard output.
///
/// This function is a simple helper to output messages, typically used for logging informational messages.
///
/// # Arguments
///
/// * `message` - A string slice that holds the message to be printed.
fn print_message(message: &str) {
    println!("{}", message);
}

/// Prints a given error message to the standard error output.
///
/// This function is intended for logging error messages, ensuring they are directed to the standard error stream.
///
/// # Arguments
///
/// * `message` - A string slice containing the error message to be printed.
pub fn print_error(message: &str) {
    eprintln!("Error: {}", message);
}

/// Logs an informational message to the console based on the specified verbosity level.
///
/// This function compares the application's current verbosity level with the provided `log_verbosity`
/// parameter. If the current verbosity level is equal to or higher than `log_verbosity`, the message
/// is printed to the standard output. This allows for granular control over which messages are displayed
/// under different verbosity settings.
///
/// # Arguments
///
/// * `message` - A string slice representing the message to be logged.
/// * `log_verbosity` - The verbosity level required for this message to be logged. A lower value
///   indicates higher importance; the message is logged if the application's verbosity level is
///   this value or higher.
///
pub(crate) fn log_info(message: &str, log_verbosity: u8) {
    let verbosity = cli::parse_args().verbosity;
    if verbosity >= log_verbosity {
        print_message(message)
    }
}

/// Executes a specified command with arguments and returns its output as a String.
///
/// # Arguments
///
/// * `command` - A string slice that holds the command to execute.
/// * `args` - A slice of string slices that holds the arguments to the command.
///
/// # Returns
///
/// The command's output as a `String` if successful, or an `Error` if the command fails.
pub(crate) fn get_command_output(
    command: &str,
    args: &[&str],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut command = Command::new(command);

    command.args(args);

    let output = command.output()?;
    let output_str = String::from_utf8(output.stdout)?.trim().to_string();

    Ok(output_str)
}

/// Retrieves system bootloader information using the `bootctl` command.
///
/// This function invokes the `bootctl` command with the `--no-pager` argument to ensure that the output
/// is not paginated. It then parses the output to extract three key pieces of information:
/// - The firmware architecture (`Firmware Arch`)
/// - The bootloader entry token (`token`)
/// - The boot root path (`$BOOT`)
///
/// These pieces of information are parsed from the lines that contain their respective identifiers.
/// The function is designed to handle cases where the information may not be at the start of the line.
///
/// # Returns
///
/// A `Result` containing a tuple of three `String` values:
/// - `firmware_arch`: The architecture of the system's firmware.
/// - `entry_token`: A unique token associated with the bootloader entry.
/// - `boot_root`: The path to the boot root directory.
///
/// If the command execution fails, or any of the required information is missing from the output,
/// the function returns an `Err` with a descriptive message.
///
/// # Errors
///
/// The function will return an error in the following cases:
/// - If the `bootctl` command execution fails.
/// - If any of the required pieces of information (`Firmware Arch`, `token`, `boot_root`) are not found in the command output.
pub(crate) fn get_bootctl_info() -> Result<(String, String, String), String> {
    let output = get_command_output("bootctl", &["--no-pager"])
        .map_err(|e| format!("Bootctl call failed: {}", e))?;

    #[derive(Default)]
    struct BootctlInfo {
        firmware_arch: Option<String>,
        entry_token: Option<String>,
        boot_root: Option<String>,
    }

    let result = output
        .lines()
        .fold(BootctlInfo::default(), |mut acc, line| {
            if acc.firmware_arch.is_none() && line.contains("Firmware Arch: ") {
                acc.firmware_arch = line.split("Firmware Arch: ").nth(1).map(str::to_string);
            } else if acc.entry_token.is_none() && line.contains("token: ") {
                acc.entry_token = line.split("token: ").nth(1).map(str::to_string);
            } else if acc.boot_root.is_none() && line.contains("$BOOT: ") {
                let start_index = line.find("$BOOT: ").unwrap() + "$BOOT: ".len();
                let end_index = line[start_index..]
                    .find(' ')
                    .map_or(line.len(), |i| start_index + i);
                acc.boot_root = Some(line[start_index..end_index].to_string());
            }
            acc
        });

    let firmware_arch = result
        .firmware_arch
        .ok_or_else(|| "Firmware Arch not found".to_string())?;
    let entry_token = result
        .entry_token
        .ok_or_else(|| "Entry token not found".to_string())?;
    let boot_root = result
        .boot_root
        .ok_or_else(|| "Boot root not found".to_string())?;

    Ok((firmware_arch, entry_token, boot_root))
}

/// Retrieves the UUID and source device of the root filesystem using the `findmnt` command.
///
/// This function invokes the `findmnt` command with specific arguments to obtain the UUID and the source device
/// associated with the root filesystem ('/'). The command output is then parsed to extract these two pieces of information.
///
/// # Returns
///
/// A `Result` containing a tuple of two `String` values:
/// - `root_uuid`: The UUID of the root filesystem.
/// - `root_device`: The source device for the root filesystem.
///
/// If the command execution fails, or if the UUID or the device information cannot be found in the command output,
/// the function returns an `Err` with a descriptive message.
///
/// # Errors
///
/// The function will return an error in the following cases:
/// - If the `findmnt` command execution fails.
/// - If the UUID of the root filesystem is not found in the command output.
/// - If the source device of the root filesystem is not found in the command output.
pub(crate) fn get_root_filesystem_info() -> Result<(String, String), String> {
    let output = get_command_output("findmnt", &["/", "-v", "-r", "-n", "-o", "UUID,SOURCE"])
        .map_err(|e| format!("Bootctl call failed: {}", e))?;

    let mut parts = output.split_whitespace();
    let root_uuid = parts.next().ok_or("UUID not found")?.to_string();
    let root_device = parts.next().ok_or("Device not found")?.to_string();

    Ok((root_uuid, root_device))
}
