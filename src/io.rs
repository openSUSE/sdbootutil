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
    eprintln!("{}", message);
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
pub fn log_info(message: &str, log_verbosity: u8) {
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

    // Convert the output to a String
    let output_str = String::from_utf8(output.stdout)?.trim().to_string();

    Ok(output_str)
}
