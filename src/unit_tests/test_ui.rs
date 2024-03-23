use super::super::ui::*;
#[test]
fn test_menu_navigation() {
    let mut menu = Menu::new(vec!["Item 1", "Item 2", "Item 3"]);
    assert_eq!(menu.selected_index, 0);

    // Navigate down
    menu.handle_input(KeyCode::Down);
    assert_eq!(menu.selected_index, 1);

    // Navigate down wraps around
    menu.handle_input(KeyCode::Down);
    menu.handle_input(KeyCode::Down);
    assert_eq!(menu.selected_index, 0);

    // Navigate up wraps around
    menu.handle_input(KeyCode::Up);
    assert_eq!(menu.selected_index, 2);
}

#[test]
fn test_get_selected_action() {
    let menu = Menu::new(vec!["Action 1", "Action 2"]);
    assert_eq!(menu.get_selected_action(), Some("Action 1"));
}
