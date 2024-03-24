pub mod cli;
pub mod ui;

pub trait MessagePrinter {
    fn log_info(&self, message: &str);
}

fn print_message(message: &str) {
    println!("{}", message);
}

pub struct ConsolePrinter;
impl MessagePrinter for ConsolePrinter {
    fn log_info(&self, message: &str) {
        let verbosity = cli::parse_args().verbosity;
        if verbosity > 0 {
            print_message(message)
        }
    }
}

pub fn get_root_snapshot() -> u64 {
    42
}

pub fn command_kernels(printer: &dyn MessagePrinter) {
    let message = "Kernels command called";
    printer.log_info(message);
}

pub fn command_snapshots(printer: &dyn MessagePrinter) {
    let message = "Snapshots command called";
    printer.log_info(message);
}

pub fn command_entries(printer: &dyn MessagePrinter) {
    let message = "Entries command called";
    printer.log_info(message);
}

pub fn command_bootloader(printer: &dyn MessagePrinter) {
    let message = "Bootloader command called";
    printer.log_info(message);
}

pub fn command_add_kernel(printer: &dyn MessagePrinter, kernel_version: &str) {
    let message = format!("AddKernel command called with version {}", kernel_version);
    printer.log_info(&message);
}

pub fn command_add_all_kernels(printer: &dyn MessagePrinter) {
    let message = "AddAllKernels command called";
    printer.log_info(message);
}

pub fn command_mkinitrd(printer: &dyn MessagePrinter) {
    let message = "Mkinitrd command called";
    printer.log_info(message);
}

pub fn command_remove_kernel(printer: &dyn MessagePrinter, kernel_version: &str) {
    let message = format!("RemoveKernel command called with version {}", kernel_version);
    printer.log_info(&message);
}

pub fn command_remove_all_kernels(printer: &dyn MessagePrinter) {
    let message = "RemoveAllKernels command called";
    printer.log_info(message);
}

pub fn command_list_kernels(printer: &dyn MessagePrinter) {
    let message = "ListKernels command called";
    printer.log_info(message);
}

pub fn command_list_entries(printer: &dyn MessagePrinter) {
    let message = "ListEntries command called";
    printer.log_info(message);
}

pub fn command_list_snapshots(printer: &dyn MessagePrinter) {
    let message = "ListSnapshots command called";
    printer.log_info(message);
}

pub fn command_set_default_snapshot(printer: &dyn MessagePrinter) {
    let message = "SetDefaultSnapshot command called";
    printer.log_info(message);
}

pub fn command_is_bootable(printer: &dyn MessagePrinter) {
    let message = "IsBootable command called";
    printer.log_info(message);
}

pub fn command_install(printer: &dyn MessagePrinter) {
    let message = "Install command called";
    printer.log_info(message);
}

pub fn command_needs_update(printer: &dyn MessagePrinter) {
    let message = "NeedsUpdate command called";
    printer.log_info(message);
}

pub fn command_update(printer: &dyn MessagePrinter) {
    let message = "Update command called";
    printer.log_info(message);
}

pub fn command_force_update(printer: &dyn MessagePrinter) {
    let message = "ForceUpdate command called";
    printer.log_info(message);
}

pub fn command_update_predictions(printer: &dyn MessagePrinter) {
    let message = "UpdatePredictions command called";
    printer.log_info(message);
}

#[cfg(test)]
mod unit_tests;