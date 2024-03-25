pub mod cli;
pub mod fs;
pub mod ui;

use std::error::Error;
use std::process::Command;

pub trait CommandExecutor {
    fn get_command_output(
        &self,
        command: &str,
        args: &[&str],
    ) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct RealCommandExecutor;
impl CommandExecutor for RealCommandExecutor {
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use sdbootutil::RealCommandExecutor;
    /// # use sdbootutil::CommandExecutor;
    /// let executor = RealCommandExecutor;
    /// let output = executor.get_command_output("echo", &["Hello, world!"]).unwrap();
    /// assert_eq!(output, "Hello, world!");
    /// ```
    fn get_command_output(
        &self,
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
}

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

pub trait MessagePrinter {
    fn log_info(&self, message: &str, log_verbosity: u8);
}
pub struct ConsolePrinter;
impl MessagePrinter for ConsolePrinter {
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
    fn log_info(&self, message: &str, log_verbosity: u8) {
        let verbosity = cli::parse_args().verbosity;
        if verbosity >= log_verbosity {
            print_message(message)
        }
    }
}

/// Returns the identifier of the root snapshot.
///
/// This function is a placeholder and currently returns a fixed value.
///
/// # Returns
///
/// Returns a `u64` that represents the identifier of the root snapshot.
///
/// # Examples
///
/// ```
/// let root_snapshot_id = sdbootutil::get_root_snapshot();
/// assert_eq!(root_snapshot_id, 42);
/// ```
pub fn get_root_snapshot() -> u64 {
    42
}

/// Logs a message indicating that the "Kernels command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_kernels, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_kernels(&printer);
/// // Check your logging output to verify the message "Kernels command called" was logged
/// ```
pub fn command_kernels(printer: &dyn MessagePrinter) {
    let message = "Kernels command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "Snapshots command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_snapshots, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_snapshots(&printer);
/// // Check your logging output to verify the message "Snapshots command called" was logged
/// ```
pub fn command_snapshots(printer: &dyn MessagePrinter) {
    let message = "Snapshots command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "Entries command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_entries, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_entries(&printer);
/// // Check your logging output to verify the message "Entries command called" was logged
/// ```
pub fn command_entries(printer: &dyn MessagePrinter) {
    let message = "Entries command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "Bootloader command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_bootloader, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_bootloader(&printer);
/// // Check your logging output to verify the message "Bootloader command called" was logged
/// ```
pub fn command_bootloader(printer: &dyn MessagePrinter) {
    let message = "Bootloader command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "AddKernel command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_add_kernel, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_add_kernel(&printer, "6.7.10-lqx1-2-liquorix");
/// // Check your logging output to verify the message "AddKernel command called" was logged
/// ```
pub fn command_add_kernel(printer: &dyn MessagePrinter, kernel_version: &str) {
    let message = format!("AddKernel command called with version {}", kernel_version);
    printer.log_info(&message, 1);
}

/// Logs a message indicating that the "AddAllKernels command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_add_all_kernels, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_add_all_kernels(&printer);
/// // Check your logging output to verify the message "AddAllKernels command called" was logged
/// ```
pub fn command_add_all_kernels(printer: &dyn MessagePrinter) {
    let message = "AddAllKernels command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "Mkinitrd command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_mkinitrd, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_mkinitrd(&printer);
/// // Check your logging output to verify the message "Mkinitrd command called" was logged
/// ```
pub fn command_mkinitrd(printer: &dyn MessagePrinter) {
    let message = "Mkinitrd command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "RemoveKernel command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_remove_kernel, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_remove_kernel(&printer, "6.7.10-lqx1-2-liquorix");
/// // Check your logging output to verify the message "RemoveKernel command called" was logged
/// ```
pub fn command_remove_kernel(printer: &dyn MessagePrinter, kernel_version: &str) {
    let message = format!(
        "RemoveKernel command called with version {}",
        kernel_version
    );
    printer.log_info(&message, 1);
}

/// Logs a message indicating that the "RemoveAllKernels command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_remove_all_kernels, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_remove_all_kernels(&printer);
/// // Check your logging output to verify the message "RemoveAllKernels command called" was logged
/// ```
pub fn command_remove_all_kernels(printer: &dyn MessagePrinter) {
    let message = "RemoveAllKernels command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "ListKernels command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_list_kernels, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_list_kernels(&printer);
/// // Check your logging output to verify the message "ListKernels command called" was logged
/// ```
pub fn command_list_kernels(printer: &dyn MessagePrinter) {
    let message = "ListKernels command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "ListEntries command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_list_entries, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_list_entries(&printer);
/// // Check your logging output to verify the message "ListEntries command called" was logged
/// ```
pub fn command_list_entries(printer: &dyn MessagePrinter) {
    let message = "ListEntries command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "ListSnapshots command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_list_snapshots, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_list_snapshots(&printer);
/// // Check your logging output to verify the message "ListSnapshots command called" was logged
/// ```
pub fn command_list_snapshots(printer: &dyn MessagePrinter) {
    let message = "ListSnapshots command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "SetDefaultSnapshot command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_set_default_snapshot, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_set_default_snapshot(&printer);
/// // Check your logging output to verify the message "SetDefaultSnapshot command called" was logged
/// ```
pub fn command_set_default_snapshot(printer: &dyn MessagePrinter) {
    let message = "SetDefaultSnapshot command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "IsBootable command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_is_bootable, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_is_bootable(&printer);
/// // Check your logging output to verify the message "IsBootable command called" was logged
/// ```
pub fn command_is_bootable(printer: &dyn MessagePrinter) {
    let message = "IsBootable command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "Install command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_install, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_install(&printer);
/// // Check your logging output to verify the message "Install command called" was logged
/// ```
pub fn command_install(printer: &dyn MessagePrinter) {
    let message = "Install command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "NeedsUpdate command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_needs_update, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_needs_update(&printer);
/// // Check your logging output to verify the message "NeedsUpdate command called" was logged
/// ```
pub fn command_needs_update(printer: &dyn MessagePrinter) {
    let message = "NeedsUpdate command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "Update command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_update, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_update(&printer);
/// // Check your logging output to verify the message "Update command called" was logged
/// ```
pub fn command_update(printer: &dyn MessagePrinter) {
    let message = "Update command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "ForceUpdate command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_force_update, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_force_update(&printer);
/// // Check your logging output to verify the message "ForceUpdate command called" was logged
/// ```
pub fn command_force_update(printer: &dyn MessagePrinter) {
    let message = "ForceUpdate command called";
    printer.log_info(message, 1);
}

/// Logs a message indicating that the "UpdatePredictions command" was called.
///
/// This function is part of the command handling logic and is typically invoked
/// in response to a specific user command.
///
/// # Arguments
///
/// * `printer` - A reference to an object that implements the `MessagePrinter` trait,
/// allowing the function to log messages in a flexible manner.
///
/// # Examples
///
/// ```
/// # use sdbootutil::{command_update_predictions, ConsolePrinter, MessagePrinter};
/// let printer = ConsolePrinter;
/// command_update_predictions(&printer);
/// // Check your logging output to verify the message "UpdatePredictions command called" was logged
/// ```
pub fn command_update_predictions(printer: &dyn MessagePrinter) {
    let message = "UpdatePredictions command called";
    printer.log_info(message, 1);
}

#[cfg(test)]
mod unit_tests;
