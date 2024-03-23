pub mod cli;
pub mod ui;

pub trait MessagePrinter {
    fn print_message(&self, message: &str);
}

pub struct ConsolePrinter;
impl MessagePrinter for ConsolePrinter {
    fn print_message(&self, message: &str) {
        println!("{}", message);
    }
}

pub fn get_root_snapshot() -> u64 {
    42
}

pub fn command_kernels(printer: &dyn MessagePrinter) {
    let message = "Kernels command called";
    printer.print_message(message);
}

pub fn command_snapshots(printer: &dyn MessagePrinter) {
    let message = "Snapshots command called";
    printer.print_message(message);
}

pub fn command_entries(printer: &dyn MessagePrinter) {
    let message = "Entries command called";
    printer.print_message(message);
}

pub fn command_bootloader(printer: &dyn MessagePrinter) {
    let message = "Bootloader command called";
    printer.print_message(message);
}

pub fn command_add_kernel(printer: &dyn MessagePrinter, kernel_version: &str) {
    let message = format!("AddKernel command called with version {}", kernel_version);
    printer.print_message(&message);
}

pub fn command_add_all_kernels(printer: &dyn MessagePrinter) {
    let message = "AddAllKernels command called";
    printer.print_message(message);
}

pub fn command_mkinitrd(printer: &dyn MessagePrinter) {
    let message = "Mkinitrd command called";
    printer.print_message(message);
}

pub fn command_remove_kernel(printer: &dyn MessagePrinter, kernel_version: &str) {
    let message = format!("RemoveKernel command called with version {}", kernel_version);
    printer.print_message(&message);
}

pub fn command_remove_all_kernels(printer: &dyn MessagePrinter) {
    let message = "RemoveAllKernels command called";
    printer.print_message(message);
}

pub fn command_list_kernels(printer: &dyn MessagePrinter) {
    let message = "ListKernels command called";
    printer.print_message(message);
}

pub fn command_list_entries(printer: &dyn MessagePrinter) {
    let message = "ListEntries command called";
    printer.print_message(message);
}

pub fn command_list_snapshots(printer: &dyn MessagePrinter) {
    let message = "ListSnapshots command called";
    printer.print_message(message);
}

pub fn command_set_default_snapshot(printer: &dyn MessagePrinter) {
    let message = "SetDefaultSnapshot command called";
    printer.print_message(message);
}

pub fn command_is_bootable(printer: &dyn MessagePrinter) {
    let message = "IsBootable command called";
    printer.print_message(message);
}

pub fn command_install(printer: &dyn MessagePrinter) {
    let message = "Install command called";
    printer.print_message(message);
}

pub fn command_needs_update(printer: &dyn MessagePrinter) {
    let message = "NeedsUpdate command called";
    printer.print_message(message);
}

pub fn command_update(printer: &dyn MessagePrinter) {
    let message = "Update command called";
    printer.print_message(message);
}

pub fn command_force_update(printer: &dyn MessagePrinter) {
    let message = "ForceUpdate command called";
    printer.print_message(message);
}

pub fn command_update_predictions(printer: &dyn MessagePrinter) {
    let message = "UpdatePredictions command called";
    printer.print_message(message);
}

#[cfg(test)]
mod unit_tests;