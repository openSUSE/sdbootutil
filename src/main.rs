use sdbootutil as lib;
use sdbootutil::cli::{parse_args, Commands};

fn main() -> Result<(), String> {
    let (root_snapshot, _root_prefix, _root_subvol, firmware_arch, boot_dst, shimdir, boot_root, _entry_token, _root_uuid, _root_device) = match lib::get_system_info() {
        Ok(info) => info,
        Err(e) => {
            let message = format!("Error getting system info: {}", e);
            return Err(message)
        }
    };
    lib::test_functions();
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
        Some(Commands::IsInstalled {}) => lib::command_is_installed(root_snapshot, &firmware_arch, &shimdir, &boot_root, &boot_dst, None, None),
        Some(Commands::Install {}) => lib::command_install(),
        Some(Commands::NeedsUpdate {}) => lib::command_needs_update(),
        Some(Commands::Update {}) => lib::command_update(),
        Some(Commands::ForceUpdate {}) => lib::command_force_update(),
        Some(Commands::UpdatePredictions {}) => lib::command_update_predictions(),
        None => lib::ui::show_main_menu(),
    };
    Ok(())
}
