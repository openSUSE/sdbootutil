use super::super::*;

#[test]
fn test_command_kernels() {
    let result = command_kernels();
    assert_eq!(result, 0);
}

#[test]
fn test_command_snapshots() {
    let result = command_snapshots();
    assert_eq!(result, 1);
}

#[test]
fn test_command_entries() {
    let result = command_entries();
    assert_eq!(result, 2);
}

#[test]
fn test_command_bootloader() {
    let result = command_bootloader();
    assert_eq!(result, 3);
}

#[test]
fn test_command_add_kernel() {
    let result = command_add_kernel("5.8.0-53-generic");
    assert_eq!(result, 4);
}

#[test]
fn test_command_add_all_kernels() {
    let result = command_add_all_kernels();
    assert_eq!(result, 5);
}

#[test]
fn test_command_mkinitrd() {
    let result = command_mkinitrd();
    assert_eq!(result, 6);
}

#[test]
fn test_command_remove_kernel() {
    let result = command_remove_kernel("5.8.0-53-generic");
    assert_eq!(result, 7);
}

#[test]
fn test_command_remove_all_kernels() {
    let result = command_remove_all_kernels();
    assert_eq!(result, 8);
}

#[test]
fn test_command_list_kernels() {
    let result = command_list_kernels();
    assert_eq!(result, 9);
}

#[test]
fn test_command_list_entries() {
    let result = command_list_entries();
    assert_eq!(result, 10);
}

#[test]
fn test_command_list_snapshots() {
    let result = command_list_snapshots();
    assert_eq!(result, 11);
}

#[test]
fn test_command_set_default_snapshot() {
    let result = command_set_default_snapshot();
    assert_eq!(result, 12);
}

#[test]
fn test_command_is_bootable() {
    let result = command_is_bootable();
    assert_eq!(result, 13);
}

#[test]
fn test_command_install() {
    let result = command_install();
    assert_eq!(result, 14);
}

#[test]
fn test_command_needs_update() {
    let result = command_needs_update();
    assert_eq!(result, 15);
}

#[test]
fn test_command_update() {
    let result = command_update();
    assert_eq!(result, 16);
}

#[test]
fn test_command_force_update() {
    let result = command_force_update();
    assert_eq!(result, 17);
}

#[test]
fn test_command_update_predictions() {
    let result = command_update_predictions();
    assert_eq!(result, 18);
}
