use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

pub fn set_raw_mode() {
    if let Err(e) = enable_raw_mode() {
        eprintln!("Failed to set raw mode: {}", e);
    }
}

pub fn reset_terminal() {
    if let Err(e) = disable_raw_mode() {
        eprintln!("Failed to reset terminal: {}", e);
    }
}

pub fn show_shortcut_menu(menu_items: &[String]) -> io::Result<Option<String>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut search_query = String::new();
    let list_items: Vec<ListItem> = menu_items
        .iter()
        .map(|item| ListItem::new(item.to_string()))
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let mut selected_item = None;
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(size);

            let search_input = Paragraph::new(Spans::from(vec![Span::styled(
                format!("Search: {}", search_query),
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            let filtered_items: Vec<ListItem> = menu_items
                .iter()
                .filter(|item| item.to_lowercase().contains(&search_query.to_lowercase()))
                .map(|item| ListItem::new(item.clone()))
                .collect();

            let block = Block::default()
                .title(" Shortcuts [press ESC to go back] ")
                .borders(Borders::ALL);
            let list = List::new(filtered_items)
                .block(block)
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut list_state);
            f.render_widget(search_input, chunks[1]);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    search_query.push(c);
                    list_state.select(Some(0));
                }
                KeyCode::Backspace => {
                    search_query.pop();
                    list_state.select(Some(0));
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Down => {
                    let pos = list_state.selected().unwrap_or(0);
                    let new_pos = (pos + 1) % list_items.len();
                    list_state.select(Some(new_pos));
                }
                KeyCode::Up => {
                    let pos = list_state.selected().unwrap_or(0);
                    let new_pos = if pos == 0 {
                        list_items.len() - 1
                    } else {
                        pos - 1
                    };
                    list_state.select(Some(new_pos));
                }
                KeyCode::Enter => {
                    selected_item = list_state
                        .selected()
                        .map(|index| menu_items[index].to_string());
                    break;
                }
                _ => {}
            }
        }
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(selected_item)
}
