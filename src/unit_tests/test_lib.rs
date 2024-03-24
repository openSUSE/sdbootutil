use super::super::*;
use mockall::predicate::*;
use mockall::*;

mock! {
    pub(crate) Printer {}
    impl MessagePrinter for Printer {
        fn log_info(&self, message: &str);
    }
}

#[test]
fn test_command_kernels() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("Kernels command called"))
        .times(1)
        .returning(|_| ());
    command_kernels(&mock_printer);
}

#[test]
fn test_command_snapshots() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("Snapshots command called"))
        .times(1)
        .returning(|_| ());
    command_snapshots(&mock_printer);
}

#[test]
fn test_command_entries() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("Entries command called"))
        .times(1)
        .returning(|_| ());
    command_entries(&mock_printer);
}

#[test]
fn test_command_bootloader() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("Bootloader command called"))
        .times(1)
        .returning(|_| ());
    command_bootloader(&mock_printer);
}

#[test]
fn test_command_add_kernel() {
    let mut mock_printer = MockPrinter::new();

    mock_printer
        .expect_log_info()
        .with(eq("AddKernel command called with version 5.8.0"))
        .times(1)
        .returning(|_| ());

    command_add_kernel(&mock_printer, "5.8.0");
}

#[test]
fn test_command_add_all_kernels() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("AddAllKernels command called"))
        .times(1)
        .returning(|_| ());

    command_add_all_kernels(&mock_printer);
}

#[test]
fn test_command_mkinitrd() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("Mkinitrd command called"))
        .times(1)
        .returning(|_| ());

    command_mkinitrd(&mock_printer);
}

#[test]
fn test_command_remove_kernel() {
    let mut mock_printer = MockPrinter::new();

    mock_printer
        .expect_log_info()
        .with(eq("RemoveKernel command called with version 5.8.0"))
        .times(1)
        .returning(|_| ());

    command_remove_kernel(&mock_printer, "5.8.0");
}

#[test]
fn test_command_remove_all_kernels() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("RemoveAllKernels command called"))
        .times(1)
        .returning(|_| ());

    command_remove_all_kernels(&mock_printer);
}

#[test]
fn test_command_list_kernels() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("ListKernels command called"))
        .times(1)
        .returning(|_| ());

    command_list_kernels(&mock_printer);
}

#[test]
fn test_command_list_entries() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("ListEntries command called"))
        .times(1)
        .returning(|_| ());

    command_list_entries(&mock_printer);
}

#[test]
fn test_command_list_snapshots() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("ListSnapshots command called"))
        .times(1)
        .returning(|_| ());

    command_list_snapshots(&mock_printer);
}

#[test]
fn test_command_set_default_snapshot() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("SetDefaultSnapshot command called"))
        .times(1)
        .returning(|_| ());

    command_set_default_snapshot(&mock_printer);
}

#[test]
fn test_command_is_bootable() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("IsBootable command called"))
        .times(1)
        .returning(|_| ());

    command_is_bootable(&mock_printer);
}

#[test]
fn test_command_install() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("Install command called"))
        .times(1)
        .returning(|_| ());

    command_install(&mock_printer);
}

#[test]
fn test_command_needs_update() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("NeedsUpdate command called"))
        .times(1)
        .returning(|_| ());

    command_needs_update(&mock_printer);
}

#[test]
fn test_command_update() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("Update command called"))
        .times(1)
        .returning(|_| ());

    command_update(&mock_printer);
}

#[test]
fn test_command_force_update() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("ForceUpdate command called"))
        .times(1)
        .returning(|_| ());

    command_force_update(&mock_printer);
}

#[test]
fn test_command_update_predictions() {
    let mut mock_printer = MockPrinter::new();
    mock_printer
        .expect_log_info()
        .with(eq("UpdatePredictions command called"))
        .times(1)
        .returning(|_| ());

    command_update_predictions(&mock_printer);
}

#[test]
fn test_get_root_snapshot() {
    assert_eq!(get_root_snapshot(), 42);
}
