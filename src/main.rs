use sdbootutil as lib;
use sdbootutil::cli::{ensure_root_permissions, parse_args, Commands};
use sdbootutil::fs;
use sdbootutil::io;
use std::path::PathBuf;

fn main() {
    if let Err(e) = ensure_root_permissions() {
        let message = format!("Failed to get root privileges: {}", e);
        io::print_error(&message);
        std::process::exit(1);
    }
    let args = parse_args();
    match fs::get_root_snapshot_info() {
        Ok((prefix, snapshot_id, full_path)) => {
            io::log_info(&format!(
                "Prefix: {}, Snapshot ID: {}, Full Path: {}",
                prefix, snapshot_id, full_path), 1
            );
        }
        Err(e) => {
            io::print_error(&format!("Error: {}", e));
        }
    }

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
        Some(Commands::Install {}) => lib::command_install(),
        Some(Commands::NeedsUpdate {}) => lib::command_needs_update(),
        Some(Commands::Update {}) => lib::command_update(),
        Some(Commands::ForceUpdate {}) => lib::command_force_update(),
        Some(Commands::UpdatePredictions {}) => lib::command_update_predictions(),
        None => lib::ui::show_main_menu(),
    };

    if fs::is_transactional().expect("Failed to check if filesystem is transactional") {
        io::log_info("It is a transactional system", 1)
    } else {
        io::log_info("It is not a transactional system", 1)
    }
    let (_temp_dir, _tmpdir_path) = fs::create_temp_dir();
    let rollback_items = vec![
        fs::RollbackItem::new(PathBuf::from("/path/to/file1")),
        fs::RollbackItem::new(PathBuf::from("/path/to/file2")),
    ];
    fs::cleanup_rollback_items(&rollback_items);
}
