use super::super::*;

#[test]
fn test_command_kernels() {
    let result = command_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_snapshots() {
    let result = command_snapshots().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_entries() {
    let result = command_entries().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_bootloader() {
    let result = command_bootloader().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_add_kernel() {
    let result = command_add_kernel("5.8.0-53-generic").unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_add_all_kernels() {
    let result = command_add_all_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_mkinitrd() {
    let result = command_mkinitrd().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_remove_kernel() {
    let result = command_remove_kernel("5.8.0-53-generic").unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_remove_all_kernels() {
    let result = command_remove_all_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_list_kernels() {
    let result = command_list_kernels().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_list_entries() {
    let result = command_list_entries().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_list_snapshots() {
    let result = command_list_snapshots().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_set_default_snapshot() {
    let result = command_set_default_snapshot().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_is_bootable() {
    let result = command_is_bootable().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_install() {
    let result = command_install().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_needs_update() {
    let result = command_needs_update().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_update() {
    let result = command_update().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_force_update() {
    let result = command_force_update().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_command_update_predictions() {
    let result = command_update_predictions().unwrap();
    assert_eq!(result, true);
}
