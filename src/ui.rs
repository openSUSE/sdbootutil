pub(crate) use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
pub(crate) use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Event<I> {
    Input(I),
}

pub struct Menu {
    pub items: Vec<String>,
    pub selected_index: usize,
}

impl Menu {
    pub fn new(items: Vec<&str>) -> Self {
        Menu {
            items: items.into_iter().map(String::from).collect(),
            selected_index: 0,
        }
    }

    pub fn handle_input(&mut self, input: KeyCode) {
        match input {
            KeyCode::Down => {
                self.selected_index = (self.selected_index + 1) % self.items.len();
            }
            KeyCode::Up => {
                if self.selected_index == 0 {
                    self.selected_index = self.items.len() - 1;
                } else {
                    self.selected_index -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn get_selected_action(&self) -> Option<&str> {
        self.items.get(self.selected_index).map(String::as_str)
    }
}

fn exit_menu(
    terminal: &mut Terminal<CrosstermBackend<impl Write>>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn show_menu() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || loop {
        if event::poll(tick_rate).unwrap() {
            if let CEvent::Key(key) = event::read().unwrap() {
                tx.send(Event::Input(key)).unwrap();
            }
        }
    });

    let menu_items = vec!["Kernels", "Snapshots", "Entries", "Install/Update"];
    let mut menu = Menu::new(menu_items);

    let mut list_state = ListState::default();
    list_state.select(Some(menu.selected_index));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().borders(Borders::ALL).title("sdbootutil");
            let menu_items: Vec<ListItem> = menu
                .items
                .iter()
                .map(|item| {
                    ListItem::new(item.clone()).style(Style::default().add_modifier(Modifier::BOLD))
                })
                .collect();
            let menu_list = List::new(menu_items)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            let constraints = [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ];

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_ref())
                .split(size);

            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.as_ref())
                .split(vertical_layout[1]);

            f.render_stateful_widget(menu_list, horizontal_layout[1], &mut list_state);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    exit_menu(&mut terminal)?;
                    break;
                }
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    exit_menu(&mut terminal)?;
                    break;
                }
                KeyCode::Down | KeyCode::Up => {
                    menu.handle_input(event.code);
                    list_state.select(Some(menu.selected_index));
                }
                KeyCode::Enter => {
                    if let Some(action) = menu.get_selected_action() {
                        match action {
                            "Kernels" => println!("Kernels action triggered"),
                            "Snapshots" => println!("Snapshots action triggered"),
                            "Entries" => println!("Entries action triggered"),
                            "Install/Update" => println!("Install/Update action triggered"),
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
