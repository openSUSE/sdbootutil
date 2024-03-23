use super::super::ui::*;
use mockall::predicate::*;
use super::test_lib;

#[test]
fn test_handle_menu_action() {
    assert_eq!(handle_menu_action(0), "Kernels action triggered");
    assert_eq!(handle_menu_action(1), "Snapshots action triggered");
    assert_eq!(handle_menu_action(999), "Unknown action");
}

#[test]
fn test_menu_new_and_into_select_view() {
    let items = vec!["Item1", "Item2"];
    let menu = Menu::new(items.clone());
    assert_eq!(menu.items, items);

    let select_view = menu.into_select_view();
    assert_eq!(select_view.len(), items.len());
}

#[test]
fn test_on_menu_select() {
    let mut mock_printer = test_lib::MockPrinter::new();
    mock_printer
        .expect_print_message()
        .with(eq("Kernels action triggered"))
        .times(1)
        .returning(|_| ());
    on_menu_select(&"0".to_string(), &mock_printer);
}
