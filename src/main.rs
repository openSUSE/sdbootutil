use lib::test_functions;
use sdbootutil as lib;
use sdbootutil::cli::{parse_args, Commands};

fn main() {
    test_functions();
    let args = parse_args();
    let _result = match args.cmd {
        Some(Commands::Kernels {}) => lib::command_kernels(),
        Some(Commands::Snapshots {}) => lib::command_snapshots(),
        Some(Commands::Entries {}) => lib::command_entries(),
        Some(Commands::Bootloader {}) => lib::command_bootloader(),
        Some(Commands::AddKernel { kernel_version }) => lib::command_add_kernel(&kernel_version),
        Some(Commands::AddAllKernels {}) => lib::command_add_all_kernels(),
        Some(Commands::Mkinitrd {}) => lib::command_mkinitrd(),
        Some(Commands::RemoveKernel { kernel_version }) => {
            lib::command_remove_kernel(&kernel_version)
        }
        Some(Commands::RemoveAllKernels {}) => lib::command_remove_all_kernels(),
        Some(Commands::ListKernels {}) => lib::command_list_kernels(),
        Some(Commands::ListEntries {}) => lib::command_list_entries(),
        Some(Commands::ListSnapshots {}) => lib::command_list_snapshots(),
        Some(Commands::SetDefaultSnapshot {}) => lib::command_set_default_snapshot(),
        Some(Commands::IsBootable {}) => lib::command_is_bootable(),
        Some(Commands::IsInstalled {}) => lib::command_install(),
        Some(Commands::Install {}) => lib::command_install(),
        Some(Commands::NeedsUpdate {}) => lib::command_needs_update(),
        Some(Commands::Update {}) => lib::command_update(),
        Some(Commands::ForceUpdate {}) => lib::command_force_update(),
        Some(Commands::UpdatePredictions {}) => lib::command_update_predictions(),
        None => lib::ui::show_main_menu(),
    };
}
