use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::error;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::{App, InputMode, Popup};

pub fn start_ui(app: App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    if let Err(e) = res {
        error!("UI Crashed {}", e);
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        if let Ok(packet) = app.rx.try_recv() {
            app.list
                .items
                .push(format!("{}: {}", app.list.items.len(), packet));
        }

        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    InputMode::NormalMode => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') => app.list.next(),
                        KeyCode::Char('k') => app.list.prev(),
                        KeyCode::Char('g') => {
                            app.popup = Popup::GotoCommand;
                            app.mode = InputMode::EditMode;
                        }
                        KeyCode::Esc => app.list.unselect(),
                        _ => (),
                    },
                    InputMode::EditMode => match key.code {
                        KeyCode::Char(c) => app.input.push(c),
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Enter => match app.popup {
                            Popup::GotoCommand => {
                                if let Ok(selection) = app.input.parse::<u64>() {
                                    app.list.select(selection as usize);
                                }
                                app.input = String::new();
                                app.popup = Popup::None;
                                app.mode = InputMode::NormalMode;
                            }
                            Popup::None => {}
                        },
                        KeyCode::Esc => {
                            app.input = String::new();
                            app.popup = Popup::None;
                            app.mode = InputMode::NormalMode;
                        }
                        _ => (),
                    },
                }
            }
        }
        terminal.draw(|f| ui(f, &mut app))?;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = get_chunks(app.popup, &f.size());

    let height: usize = (f.size().height - 2) as usize;
    // Will be moved when selection movement will be implemented
    if let Some(selected) = app.list.state.selected() {
        if selected < app.scroll() {
            app.set_scroll(selected);
        }
    } else {
        app.set_scroll(app.list.items.len().checked_sub(height).unwrap_or(0));
    }

    let items: Vec<ListItem> = app.list.items[app.scroll()..]
        .iter()
        .map(|i| ListItem::new(Span::raw(i)))
        .collect();
    let list = List::new(items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(format!(
                    "Frames: {}, Selected: {}",
                    app.list.items.len(),
                    app.list.state.selected().unwrap_or(0)
                ))
                .title_alignment(Alignment::Center),
        );

    // TMP
    if app.popup == Popup::GotoCommand {
        f.render_widget(command_input(app.input.clone()), chunks[1]);
    }

    f.render_stateful_widget(list, chunks[0], &mut app.list.state);
}

fn command_input(input: String) -> Paragraph<'static> {
    Paragraph::new(Text::raw(format!("go to frame: {}", input))).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
}

fn get_chunks(popup: Popup, rect: &Rect) -> Vec<Rect> {
    let constraints: Vec<Constraint> = match popup {
        Popup::GotoCommand => [Constraint::Percentage(80), Constraint::Percentage(20)].to_vec(),
        Popup::None => [Constraint::Percentage(100)].to_vec(),
    };
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(*rect)
}
