
pub(crate) use cursive::views::{Dialog, SelectView, TextView, LinearLayout};
pub(crate) use cursive::traits::*;
pub(crate) use cursive::event::Event;
pub(crate) use super::{MessagePrinter, ConsolePrinter};

pub struct Menu {
    pub items: Vec<String>,
}

pub fn handle_menu_action(idx: usize) -> &'static str {
    match idx {
        0 => "Kernels action triggered",
        1 => "Snapshots action triggered",
        _ => "Unknown action",
    }
}

pub(crate) fn on_menu_select(idx: &String, printer: &dyn MessagePrinter) {
    let idx = idx.parse::<usize>().unwrap_or_default();
    let message = handle_menu_action(idx);
    printer.print_message(&message);
}

impl Menu {
    pub fn new(items: Vec<&str>) -> Self {
        Menu {
            items: items.into_iter().map(String::from).collect(),
        }
    }

    pub fn into_select_view(self) -> SelectView<String> {
        let mut select_view = SelectView::new().h_align(cursive::align::HAlign::Center);

        for (idx, item) in self.items.iter().enumerate() {
            select_view.add_item(item.clone(), idx.to_string());
        }

        select_view
    }
}

pub fn show_main_menu() {
    let menu_items = vec!["Kernels", "Snapshots", "Entries", "Install/Update"];
    let menu = Menu::new(menu_items);
    let console_printer = ConsolePrinter;
    let mut siv = cursive::default();

    let title = TextView::new("Systemd-boot").center();
    let select_view = menu.into_select_view().on_submit(move |_, idx| {
        on_menu_select(idx, &console_printer);
    });

    let dialog = Dialog::around(select_view.scrollable()).title("Main Menu");
    let layout = LinearLayout::vertical()
        .child(title)
        .child(dialog);

    siv.add_layer(layout);
    siv.add_global_callback(Event::Char('q'), |s| s.quit());
    siv.run();
}