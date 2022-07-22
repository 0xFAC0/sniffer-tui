use std::sync::mpsc::{self, Receiver, Sender};

use tui::widgets::ListState;

pub mod sniffer;
pub mod ui;

pub struct StateList<T> {
    items: Vec<T>,
    state: ListState,
}

impl<T> StateList<T> {
    pub fn new() -> StateList<T> {
        StateList {
            items: vec![],
            state: ListState::default(),
        }
    }

    pub fn next(&mut self) {
        // Making sure there is at least one item
        if self.items.len() == 0 {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i + 1 < self.items.len() {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => i.checked_sub(1).unwrap_or(0),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self, select: usize) {
        if select > self.items.len() {
            self.state.select(Some(self.items.len() - 1));
        } else {
            self.state.select(Some(select));
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub enum InputMode {
    EditMode,
    NormalMode,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Popup {
    GotoCommand,
    None,
}

pub struct App {
    // TODO: Change string for a made struct
    pub list: StateList<String>,
    pub rx: Receiver<String>,
    scroll: usize,
    pub mode: InputMode,
    pub popup: Popup,
    pub input: String,
}

impl App {
    pub fn new() -> (App, Sender<String>) {
        let (tx, rx) = mpsc::channel();
        (
            App {
                list: StateList::new(),
                rx,
                scroll: 0,
                mode: InputMode::NormalMode,
                popup: Popup::None,
                input: String::new(),
            },
            tx,
        )
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn set_scroll(&mut self, to: usize) {
        if to > self.list.items.len() {
            self.scroll = self.list.items.len() - 1;
        } else {
            self.scroll = to
        }
    }
}
