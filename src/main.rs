mod cli;
mod ui;

use cli::{parse_args, Commands};
use sdbootutil as lib;

fn main() {
    let args = parse_args();
    let _snapshot = args.snapshot.unwrap_or_else(lib::get_root_snapshot);
    let console_printer = lib::ConsolePrinter;

    match args.cmd {
        Some(Commands::Kernels {}) => lib::command_kernels(&console_printer),
        Some(Commands::Snapshots {}) => lib::command_snapshots(&console_printer),
        Some(Commands::Entries {}) => lib::command_entries(&console_printer),
        Some(Commands::Bootloader {}) => lib::command_bootloader(&console_printer),
        Some(Commands::AddKernel { kernel_version }) => lib::command_add_kernel(&console_printer, &kernel_version),
        Some(Commands::AddAllKernels {}) => lib::command_add_all_kernels(&console_printer),
        Some(Commands::Mkinitrd {}) => lib::command_mkinitrd(&console_printer),
        Some(Commands::RemoveKernel { kernel_version }) => lib::command_remove_kernel(&console_printer, &kernel_version),
        Some(Commands::RemoveAllKernels {}) => lib::command_remove_all_kernels(&console_printer),
        Some(Commands::ListKernels {}) => lib::command_list_kernels(&console_printer),
        Some(Commands::ListEntries {}) => lib::command_list_entries(&console_printer),
        Some(Commands::ListSnapshots {}) => lib::command_list_snapshots(&console_printer),
        Some(Commands::SetDefaultSnapshot {}) => lib::command_set_default_snapshot(&console_printer),
        Some(Commands::IsBootable {}) => lib::command_is_bootable(&console_printer),
        Some(Commands::Install {}) => lib::command_install(&console_printer),
        Some(Commands::NeedsUpdate {}) => lib::command_needs_update(&console_printer),
        Some(Commands::Update {}) => lib::command_update(&console_printer),
        Some(Commands::ForceUpdate {}) => lib::command_force_update(&console_printer),
        Some(Commands::UpdatePredictions {}) => lib::command_update_predictions(&console_printer),
        None => ui::show_menu().expect("Failed to display the menu"),
    }
}